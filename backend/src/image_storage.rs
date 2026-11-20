use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};

use image::ImageFormat;
use serde::{Deserialize, Serialize};
use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ImageMetadata {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Clone)]
pub struct UploadDir(pub PathBuf);

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024;

pub fn images_dir() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../public/images")
}

pub async fn save_metadata(
    image_path: &Path,
    metadata: &ImageMetadata,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let json_path = image_path.with_extension("json");
    let json = serde_json::to_vec_pretty(metadata)?;

    tokio::fs::write(json_path, json).await?;

    Ok(())
}

pub async fn process_upload(
    bytes: &[u8],
    metadata: ImageMetadata,
    upload_dir: &Path,
) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
    let path = match process_image(bytes, upload_dir).await {
        Ok(path) => path,
        Err(err) => {
            error!("Image processing failed: {err}");
            return Err(err);
        }
    };

    if let Err(metadata_err) = save_metadata(&path, &metadata).await {
        error!("Failed to save metadata: {metadata_err}");

        if let Err(rollback_err) = tokio::fs::remove_file(&path).await {
            let err = io::Error::new(
                rollback_err.kind(),
                format!(
                    "Failed to save metadata: {metadata_err}; failed to remove image {} during rollback: {rollback_err}",
                    path.display()
                ),
            );

            error!("Upload rollback failed: {err}");
            return Err(err.into());
        }

        return Err(metadata_err);
    }

    info!(
        "Image uploaded successfully: {} ({} bytes, {} tags)",
        path.display(),
        bytes.len(),
        metadata.tags.len()
    );

    Ok(path)
}

pub async fn process_image(
    bytes: &[u8],
    upload_dir: &Path,
) -> Result<PathBuf, Box<dyn Error + Send + Sync>> {
    let format = validate_image(bytes)?;

    let extension = match format {
        ImageFormat::Png => "png",
        ImageFormat::Jpeg => "jpg",
        ImageFormat::WebP => "webp",
        _ => return Err("Unsupported image format".into()),
    };

    let path = save_image(bytes, upload_dir, extension).await?;

    Ok(path)
}

pub fn validate_image(bytes: &[u8]) -> Result<ImageFormat, &'static str> {
    if bytes.len() > MAX_IMAGE_SIZE {
        return Err("Image exceeds 10MB");
    }

    let format = image::guess_format(bytes).map_err(|_| "Invalid image format")?;

    match format {
        ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP => {}
        _ => return Err("Unsupported image format"),
    }

    image::load_from_memory_with_format(bytes, format).map_err(|_| "Invalid image data")?;

    Ok(format)
}

pub async fn save_image(bytes: &[u8], upload_dir: &Path, extension: &str) -> io::Result<PathBuf> {
    fs::create_dir_all(upload_dir).await?;

    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let filepath = upload_dir.join(filename);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filepath)
        .await?;

    file.write_all(bytes).await?;
    file.flush().await?;

    Ok(filepath)
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum_test::{
        TestServer,
        multipart::{MultipartForm, Part},
    };
    use image::{ExtendedColorType, ImageEncoder, codecs::png::PngEncoder};

    use crate::routes::router_with_images_dir;

    #[tokio::test]
    async fn upload_storage_failure_returns_internal_error() {
    // Arrange: create isolated temporary storage
    let temp_dir = tempfile::tempdir().expect("Failed to create temporary directory");

    // Create a regular file where the upload directory should be
    let blocked_path = temp_dir.path().join("blocked_storage");
    let original_content = b"existing file";

    tokio::fs::write(&blocked_path, original_content)
        .await
        .expect("Failed to create blocking file");

    // Configure the application with the invalid directory
    let server = TestServer::try_new(router_with_images_dir(blocked_path.clone()))
        .expect("Failed to create test server");

    // Generate a valid PNG so validation succeeds
    let mut image_bytes = Vec::new();

    PngEncoder::new(&mut image_bytes)
        .write_image(&[255, 0, 0, 255], 1, 1, ExtendedColorType::Rgba8)
        .expect("Failed to generate test image");

    let form = MultipartForm::new()
        .add_part(
            "file",
            Part::bytes(image_bytes)
                .file_name("test.png")
                .mime_type("image/png"),
        )
        .add_text("title", "Storage Failure Test")
        .add_text("tags", "testing");

    // Act
    let response = server.post("/uploads").multipart(form).await;

    // Assert: internal server error
    response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);

    assert_eq!(response.text(), "Failed to save upload");

    // Assert: existing file remains unchanged
    let stored_content = tokio::fs::read(&blocked_path)
        .await
        .expect("Blocking file should still exist");

    assert_eq!(stored_content, original_content);

    // Assert: no additional files were created
    let mut entries = tokio::fs::read_dir(temp_dir.path())
        .await
        .expect("Failed to inspect temporary directory");

    let mut entry_count = 0;

    while entries.next_entry().await.unwrap().is_some() {
        entry_count += 1;
    }

        assert_eq!(
            entry_count, 1,
            "Failed upload should not create additional files"
        );
    }
}
