pub mod leetcode;

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{error::{AppError, AppResult}, routes::leetcode::leetcode_routes};

pub fn create_router() -> Router{
    Router::new()
    .route("/healthz", get(health_handler))
    .route("/crash", get(crash_handler))
    .nest("/leet", leetcode_routes())
}

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
}

async fn health_handler() -> AppResult<Json<HealthResponse>> {
    // TDOD: add DB + scheduler checks
    Ok(Json(HealthResponse { status: "ok" }))
}

async fn crash_handler() -> AppResult<Json<HealthResponse>> {
    // Simulate an unexpected crash (will be 500)
    Err(AppError::Internal)
}

