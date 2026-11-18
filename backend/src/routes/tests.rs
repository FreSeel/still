use std::path::PathBuf;

use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};

use super::router_with_images_dir;
use crate::image_storage::ImageMetadata;

fn create_test_server(storage_dir: PathBuf) -> TestServer {
    TestServer::try_new(
        router_with_images_dir(storage_dir)
    )
    .expect("Failed to create test server")
}

fn create_test_png() -> Vec<u8> {
    let mut bytes = Vec::new();

    PngEncoder::new(&mut bytes)
        .write_image(
            &[255, 0, 0, 255],
            1,
            1,
            ExtendedColorType::Rgba8,
        )
        .expect("Failed to generate test PNG");

    bytes
}

#[tokio::test]
async fn upload_valid_image_returns_created() {
    // Arrange: isolated storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = create_test_server(temp_dir.path().to_path_buf());

    // Generate a valid 1x1 PNG
    let image_bytes = create_test_png();

    // Construct multipart request
    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(image_bytes.clone())
                .file_name("test.png")
                .mime_type("image/png"),
        )
        .add_text("title", "Test Image")
        .add_text("tags", "rust")
        .add_text("tags", "testing");

    // Act: upload the image
    let response = server.post("/uploads").multipart(form).await;

    // Assert: expected HTTP response
    response.assert_status(StatusCode::CREATED);

    let saved_path = PathBuf::from(response.text());

    // Assert: correct storage directory
    assert!(
        saved_path.starts_with(temp_dir.path()),
        "Image was saved outside temporary storage"
    );

    // Assert: correct extension
    assert_eq!(
        saved_path.extension().and_then(|ext| ext.to_str()),
        Some("png")
    );

    // Assert: original image bytes preserved
    let stored_bytes = tokio::fs::read(&saved_path)
        .await
        .expect("Uploaded image was not saved");

    assert_eq!(stored_bytes, image_bytes);

    // Assert: metadata persisted
    let metadata_path = saved_path.with_extension("json");

    let metadata_bytes = tokio::fs::read(metadata_path)
        .await
        .expect("Image metadata was not saved");

    let metadata: ImageMetadata =
        serde_json::from_slice(&metadata_bytes).expect("Invalid metadata JSON");

    assert_eq!(metadata.title, "Test Image");
    assert_eq!(metadata.tags, vec!["rust", "testing"]);
}


#[tokio::test]
async fn upload_missing_file_returns_bad_request() {
    // Arrange: create isolated storage
    let temp_dir = tempfile::tempdir()
        .expect("Failed to create temporary directory");

    let server = create_test_server(temp_dir.path().to_path_buf());

    // Multipart form with title and tags, but no file
    let form = MultipartForm::new()
        .add_text("title", "Test Image")
        .add_text("tags", "rust");

    // Act: submit invalid upload
    let response = server
        .post("/uploads")
        .multipart(form)
        .await;

    // Assert: correct HTTP status
    response.assert_status(StatusCode::BAD_REQUEST);

    // Assert: correct error message
    assert_eq!(
        response.text(),
        "Missing file or title"
    );

    // Assert: no files were written
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries.next_entry().await
            .expect("Failed to inspect directory")
            .is_none(),
        "Upload should not create files when file is missing"
    );
}


#[tokio::test]
async fn upload_missing_title_returns_bad_request() {
    // Arrange: isolated storage
    let temp_dir = tempfile::tempdir()
        .expect("Failed to create temporary directory");

    let server = create_test_server(temp_dir.path().to_path_buf());

    // Generate a valid 1x1 PNG
    let image_bytes = create_test_png();

    // Valid file and tags, but no title
    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(image_bytes)
                .file_name("test.png")
                .mime_type("image/png"),
        )
        .add_text("tags", "rust");

    // Act
    let response = server
        .post("/uploads")
        .multipart(form)
        .await;

    // Assert: correct error response
    response.assert_status(StatusCode::BAD_REQUEST);

    assert_eq!(
        response.text(),
        "Missing file or title"
    );

    // Assert: no files were created
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries.next_entry().await
            .expect("Failed to inspect directory")
            .is_none(),
        "Upload should not create files when title is missing"
    );
}
