use std::io;
use std::path::{Path, PathBuf};

use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;
use image::{ImageFormat, ImageReader, Limits};

const MAX_IMAGE_SIZE: usize = 10 * 1024 * 1024; 

pub fn validate_image(bytes: &[u8]) -> Result<ImageFormat, &'static str> {
    if bytes.len() > MAX_IMAGE_SIZE {
        return Err("Image exceeds 10MB");
    }

    let format = image::guess_format(bytes).map_err(|_| "Invalid image format")?;

    match format {
        ImageFormat::Png | ImageFormat::Jpeg | ImageFormat::WebP => {
            Ok(format)
        }
        _ => Err("Unsupported image format"),
    }
}

pub async fn save_image(
    bytes: &[u8],
    upload_dir: &Path,
    extension: &str,
) -> io::Result<PathBuf> {
    fs::create_dir_all(upload_dir).await?;

    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let filepath = upload_dir.join(filename);

    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filepath)
        .await?;

    file.write_all(bytes).await?;

    Ok(filepath)
}