use serde::Serialize;

#[derive(Serialize)]
pub struct HealthCheckResponse {
    status: String,
}

pub fn check_health() -> HealthCheckResponse {
    HealthCheckResponse {
        status: "ok".to_string(),
    }
}
