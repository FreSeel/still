use std::io;
use std::path::{Path, PathBuf};

use tokio::fs::{self, OpenOptions};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

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