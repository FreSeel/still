use std::path::PathBuf;

use axum::{
    Extension, Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::{
    handlers::{health::health, images::get_images, uploads::upload_image, websocket::images_ws},
    image_storage::{UploadDir, images_dir},
};

pub fn router() -> Router {
    router_with_images_dir(images_dir())
}

pub(crate) fn router_with_images_dir(storage_dir: PathBuf) -> Router {
    let (tx, _) = broadcast::channel::<&'static str>(16);

    Router::new()
        .route("/health", get(health))
        .route("/images", get(get_images))
        .route("/ws/images", get(images_ws))
        .nest_service("/images/", ServeDir::new(&storage_dir))
        .route(
            "/uploads",
            post(upload_image).layer(DefaultBodyLimit::max(11 * 1024 * 1024)),
        )
        .layer(Extension(UploadDir(storage_dir)))
        .with_state(tx)
}
