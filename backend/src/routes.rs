use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use crate::handlers::{health::health, images::get_images, uploads::upload_image};

pub fn router() -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/images", get(get_images))
        .route(
            "/uploads",
            post(upload_image).layer(DefaultBodyLimit::max(11 * 1024 * 1024)),
        )
}
