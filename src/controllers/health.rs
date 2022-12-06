use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct HealthResponse {
    ok: bool,
}

pub async fn get() -> impl IntoResponse {
    let health_response = HealthResponse { ok: true };
    (StatusCode::OK, Json(health_response))
}
