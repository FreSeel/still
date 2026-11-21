use std::path::PathBuf;

use axum::http::StatusCode;
use axum_test::{
    TestServer,
    multipart::{MultipartForm, Part},
};
use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};

use still_backend::{image_storage::ImageMetadata, routes::router_with_images_dir};

fn create_test_server(storage_dir: PathBuf) -> TestServer {
    TestServer::try_new(router_with_images_dir(storage_dir)).expect("Failed to create test server")
}

fn create_test_png() -> Vec<u8> {
    let mut bytes = Vec::new();

    PngEncoder::new(&mut bytes)
        .write_image(&[255, 0, 0, 255], 1, 1, ExtendedColorType::Rgba8)
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
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = create_test_server(temp_dir.path().to_path_buf());

    // Multipart form with title and tags, but no file
    let form = MultipartForm::new()
        .add_text("title", "Test Image")
        .add_text("tags", "rust");

    // Act: submit invalid upload
    let response = server.post("/uploads").multipart(form).await;

    // Assert: correct HTTP status
    response.assert_status(StatusCode::BAD_REQUEST);

    // Assert: correct error message
    assert_eq!(response.text(), "Missing file or title");

    // Assert: no files were written
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries
            .next_entry()
            .await
            .expect("Failed to inspect directory")
            .is_none(),
        "Upload should not create files when file is missing"
    );
}

#[tokio::test]
async fn upload_missing_title_returns_bad_request() {
    // Arrange: isolated storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

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
    let response = server.post("/uploads").multipart(form).await;

    // Assert: correct error response
    response.assert_status(StatusCode::BAD_REQUEST);

    assert_eq!(response.text(), "Missing file or title");

    // Assert: no files were created
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries
            .next_entry()
            .await
            .expect("Failed to inspect directory")
            .is_none(),
        "Upload should not create files when title is missing"
    );
}

#[tokio::test]
async fn upload_invalid_image_returns_bad_request() {
    // Arrange: isolated storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = TestServer::try_new(router_with_images_dir(temp_dir.path().to_path_buf()))
        .expect("Failed to create test server");

    // File claims to be PNG, but contains plain text
    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(b"not an actual image".to_vec())
                .file_name("fake.png")
                .mime_type("image/png"),
        )
        .add_text("title", "Invalid Image")
        .add_text("tags", "testing");

    // Act
    let response = server.post("/uploads").multipart(form).await;

    // Assert: validation failure
    response.assert_status(StatusCode::BAD_REQUEST);
    assert_eq!(response.text(), "Invalid image format");

    // Assert: no image or metadata was persisted
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries.next_entry().await.unwrap().is_none(),
        "Invalid upload should not create files"
    );
}

#[tokio::test]
async fn upload_corrupted_image_returns_bad_request() {
    // Arrange: the PNG signature is valid, but the remaining data is corrupt
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");
    let server = create_test_server(temp_dir.path().to_path_buf());
    let corrupted_png = b"\x89PNG\r\n\x1a\ninvalid data";

    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(corrupted_png.to_vec())
                .file_name("corrupted.png")
                .mime_type("image/png"),
        )
        .add_text("title", "Corrupted Image");

    // Act
    let response = server.post("/uploads").multipart(form).await;

    // Assert: recognizing the signature is not enough to accept the image
    response.assert_status(StatusCode::BAD_REQUEST);
    assert_eq!(response.text(), "Invalid image data");

    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries.next_entry().await.unwrap().is_none(),
        "Corrupted upload should not create files"
    );
}

#[tokio::test]
async fn upload_unsupported_format_returns_bad_request() {
    // Arrange
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = TestServer::try_new(router_with_images_dir(temp_dir.path().to_path_buf()))
        .expect("Failed to create test server");

    // Valid 1x1 GIF image
    let gif_bytes = vec![
        0x47, 0x49, 0x46, 0x38, 0x39, 0x61, 0x01, 0x00, 0x01, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00,
        0x00, 0xFF, 0xFF, 0xFF, 0x21, 0xF9, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x2C, 0x00, 0x00,
        0x00, 0x00, 0x01, 0x00, 0x01, 0x00, 0x00, 0x02, 0x02, 0x44, 0x01, 0x00, 0x3B,
    ];

    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(gif_bytes)
                .file_name("test.gif")
                .mime_type("image/gif"),
        )
        .add_text("title", "Unsupported Image");

    // Act
    let response = server.post("/uploads").multipart(form).await;

    // Assert: format explicitly rejected
    response.assert_status(StatusCode::BAD_REQUEST);
    assert_eq!(response.text(), "Unsupported image format");

    // Assert: no files persisted
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to read temporary directory");

    assert!(
        entries.next_entry().await.unwrap().is_none(),
        "Unsupported upload should not create files"
    );
}

#[tokio::test]
async fn get_uploaded_image_returns_correct_content() {
    // Arrange: isolated storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = TestServer::try_new(router_with_images_dir(temp_dir.path().to_path_buf()))
        .expect("Failed to create test server");

    // Generate a valid PNG
    let image_bytes = create_test_png();

    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(image_bytes.clone())
                .file_name("test.png")
                .mime_type("image/png"),
        )
        .add_text("title", "Retrieval Test")
        .add_text("tags", "testing");

    // Act: upload image
    let upload_response = server.post("/uploads").multipart(form).await;

    upload_response.assert_status(StatusCode::CREATED);

    // Extract the generated filename
    let uploaded_path = PathBuf::from(upload_response.text());

    let filename = uploaded_path
        .file_name()
        .and_then(|name| name.to_str())
        .expect("Invalid uploaded filename");

    // Act: retrieve image through the API
    let response = server.get(&format!("/images/{filename}")).await;

    // Assert: HTTP status
    response.assert_status(StatusCode::OK);

    // Assert: correct content type
    response.assert_header(axum::http::header::CONTENT_TYPE, "image/png");

    // Assert: exact image bytes
    assert_eq!(
        response.as_bytes().as_ref(),
        image_bytes.as_slice(),
        "Retrieved image differs from uploaded image"
    );
}

#[tokio::test]
async fn get_nonexistent_image_returns_not_found() {
    // Arrange: isolated empty storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    let server = TestServer::try_new(router_with_images_dir(temp_dir.path().to_path_buf()))
        .expect("Failed to create test server");

    // Act: request a nonexistent image
    let response = server.get("/images/nonexistent.png").await;

    // Assert: file not found
    response.assert_status(StatusCode::NOT_FOUND);
}
