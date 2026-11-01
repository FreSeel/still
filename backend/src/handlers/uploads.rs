use axum::{
    extract::{Multipart, State},
    http::StatusCode,
};
use tokio::sync::broadcast;
use tracing::{error, warn};

use crate::image_storage::{ImageMetadata, images_dir, process_upload};

pub async fn upload_image(
    State(tx): State<broadcast::Sender<&'static str>>,
    mut multipart: Multipart,
) -> (StatusCode, String) {
    let mut bytes = None;
    let mut title = None;
    let mut tags = Vec::new();

    while let Ok(Some(field)) = multipart.next_field().await {
        match field.name() {
            Some("file") => bytes = field.bytes().await.ok(),
            Some("title") => title = field.text().await.ok(),
            Some("tags") => {
                if let Ok(tag) = field.text().await {
                    tags.push(tag);
                }
            }
            _ => {}
        }
    }

    let (Some(bytes), Some(title)) = (bytes, title) else {
        warn!("Upload rejected: missing file or title");
        return (StatusCode::BAD_REQUEST, "Missing file or title".into());
    };

    let metadata = ImageMetadata { title, tags };

    match process_upload(&bytes, metadata, &images_dir()).await {
        Ok(path) => {
            let _ = tx.send("image_uploaded");
            (StatusCode::CREATED, path.display().to_string())
        }
        Err(err) if err.is::<std::io::Error>() => {
            error!("Failed to save upload: {err}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to save upload".into(),
            )
        }
        Err(err) => {
            warn!("Upload rejected: {err}");
            (StatusCode::BAD_REQUEST, err.to_string())
        }
    }
}
