use axum::{routing::get, Router};
use crate::handlers::health_check::health_check_handler;

pub fn health_routes() -> Router {
    Router::new().route("/health", get(health_check_handler))
}
