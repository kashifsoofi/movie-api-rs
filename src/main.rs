use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router};
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health));

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
struct HealthResponse {
    ok: bool
}

async fn health() -> impl IntoResponse {
    let health_response = HealthResponse {
        ok: true
    };
    (StatusCode::OK, Json(health_response))
}