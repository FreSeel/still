use std::path::Path;

use axum::{extract::Multipart, http::StatusCode};

use crate::image_storage::process_image;

pub async fn upload_image(mut multipart: Multipart) -> (StatusCode, String) {
    let field = match multipart.next_field().await {
        Ok(Some(field)) => field,
        _ => return (StatusCode::BAD_REQUEST, "Missing or invalid file".into()),
    };

    let bytes = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(_) => return (StatusCode::BAD_REQUEST, "Invalid upload".into()),
    };

    match process_image(&bytes, Path::new("../public/images")).await {
        Ok(path) => (StatusCode::CREATED, path.display().to_string()),
        Err(err) if err.is::<std::io::Error>() => (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save image".into(),
        ),
        Err(err) => (StatusCode::BAD_REQUEST, err.to_string()),
    }
}
