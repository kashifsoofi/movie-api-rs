use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};

use crate::store::store::DynStore;

#[derive(Deserialize, Serialize)]
struct HealthResponse {
    ok: bool,
    store_ok: bool,
}

pub async fn get(State(store): State<DynStore>) -> impl IntoResponse {
    let store_ok = store.is_connected().await;
    let health_response = HealthResponse { ok: true, store_ok };
    (StatusCode::OK, Json(health_response))
}
