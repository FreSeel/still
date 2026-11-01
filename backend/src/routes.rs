use axum::{
    Router,
    extract::DefaultBodyLimit,
    routing::{get, post},
};

use tokio::sync::broadcast;
use tower_http::services::ServeDir;

use crate::handlers::{
    health::health, images::get_images, uploads::upload_image, websocket::images_ws,
};

pub fn router() -> Router {
    let (tx, _) = broadcast::channel::<&'static str>(16);

    Router::new()
        .route("/health", get(health))
        .route("/images", get(get_images))
        .route("/ws/images", get(images_ws))
        .nest_service("/images/", ServeDir::new("../public/images"))
        .route(
            "/uploads",
            post(upload_image).layer(DefaultBodyLimit::max(11 * 1024 * 1024)),
        )
        .with_state(tx)
}
