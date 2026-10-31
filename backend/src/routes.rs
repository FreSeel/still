use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use crate::handlers::{health::health, uploads::upload_image};

pub fn router() -> Router {
    Router::new().route("/health", get(health)).route(
        "/uploads",
        post(upload_image).layer(DefaultBodyLimit::max(11 * 1024 * 1024)),
    )
}
