use axum::Json;
use core::services::health_check::{check_health, HealthCheckResponse};

pub async fn health_check_handler() -> Json<HealthCheckResponse> {
    Json(check_health())
}
