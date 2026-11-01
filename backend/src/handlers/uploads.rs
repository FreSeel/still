use std::path::Path;

use axum::{extract::Multipart, http::StatusCode};

use crate::image_storage::{ImageMetadata, process_upload};

pub async fn upload_image(mut multipart: Multipart) -> (StatusCode, String) {
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
        return (StatusCode::BAD_REQUEST, "Missing file or title".into());
    };

    let metadata = ImageMetadata { title, tags };

    match process_upload(&bytes, metadata, Path::new("../public/images")).await {
        Ok(path) => (StatusCode::CREATED, path.display().to_string()),
        Err(err) if err.is::<std::io::Error>() => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save upload".into(),
        ),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}
