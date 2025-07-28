pub mod leetcode;
pub mod auth;

use axum::{routing::get, Json, Router};
use serde::Serialize;

use crate::{error::{AppError, AppResult}, routes::{auth::auth_routes, leetcode::leetcode_routes}, state::AppState};

pub fn create_router(state : AppState) -> Router{
    Router::new()
    .route("/healthz", get(health_handler))
    .route("/crash", get(crash_handler))
    .nest("/leet", leetcode_routes(state.clone()))
    .merge(auth_routes(state.clone()))
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

