use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::Body,
};

/// Token estático para validação
const AUTH_TOKEN: &str = "meu_token_secreto";

pub async fn auth_middleware(request: Request<Body>, next: Next) -> Result<Response, StatusCode> {
    // Extrai o header Authorization
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok());

    // Verifica se o token é válido
    match auth_header {
        Some(header) if header == format!("Bearer {}", AUTH_TOKEN) => {
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}