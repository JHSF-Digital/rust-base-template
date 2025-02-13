use axum::{routing::get, Router};
use crate::handlers::health_check::health_check_handler;
use crate::middlewares::auth::auth_middleware;
use tower::ServiceBuilder;

pub fn health_routes() -> Router {
    Router::new()
      .route("/health", get(health_check_handler))
      .route_layer(ServiceBuilder::new().layer(axum::middleware::from_fn(auth_middleware)))
}
