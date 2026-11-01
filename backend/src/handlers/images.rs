use axum::{Json, http::StatusCode};
use serde::Serialize;
use tokio::fs;
use tracing::{error, info};

use crate::image_storage::{ImageMetadata, images_dir};

#[derive(Serialize)]
pub struct FeedImage {
    pub url: String,
    #[serde(flatten)]
    pub metadata: ImageMetadata,
}

pub async fn get_images() -> Result<Json<Vec<FeedImage>>, StatusCode> {
    let upload_dir = images_dir();

    let is_new = !upload_dir.exists();

    fs::create_dir_all(&upload_dir).await.map_err(|err| {
        error!("Failed to initialize images directory: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if is_new {
        info!("Created images directory: {}", upload_dir.display());
    } else {
        info!("Loaded existing images directory: {}", upload_dir.display());
    }

    let mut entries = fs::read_dir(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut images = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        let path = entry.path();

        if path.extension().is_some_and(|ext| ext == "json") {
            continue;
        }

        let Ok(json) = fs::read(path.with_extension("json")).await else {
            continue;
        };

        let Ok(metadata) = serde_json::from_slice::<ImageMetadata>(&json) else {
            info!("Invalid metadata for {}", path.display());
            continue;
        };

        let modified = entry.metadata().await.ok().and_then(|m| m.modified().ok());

        images.push((
            modified,
            FeedImage {
                url: format!("/images/{}", entry.file_name().to_string_lossy()),
                metadata,
            },
        ));
    }

    images.sort_unstable_by(|a, b| b.0.cmp(&a.0));

    Ok(Json(images.into_iter().map(|(_, image)| image).collect()))
}
