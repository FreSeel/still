use axum::{Router, routing::get};

use crate::handlers::health::health;

pub fn router() -> Router {
    Router::new().route("/health", get(health))
}
