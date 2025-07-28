use axum::{extract::{Path, Query}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use crate::{auth::middleware::AuthenticatedUser, db, error::{AppError, AppResult}, gql::client::check_user_exists, state::AppState, sync::sync_user_problems};

pub fn leetcode_routes(state: AppState)-> Router{
    Router::new()
    .route("/exists/{username}", get(user_exists))
    .route("/sync", post(sync_user_problems_handler))
    .with_state(state)
   
}

#[derive(Serialize)]
struct ExistsResponse {
    username: String,
    exists: bool,
}

#[derive(Serialize)]
struct SyncResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct SyncQuery {
    pub limit: Option<usize>,
}

async fn user_exists(Path(username): Path<String>) -> AppResult<Json<ExistsResponse>> {
  let exists = check_user_exists(&username).await?;
  Ok(Json(ExistsResponse { username, exists }))
   
}

use axum::extract::State;

async fn sync_user_problems_handler(
  user: AuthenticatedUser,
  State(state): State<AppState>,
Query(sync_query): Query<SyncQuery>

) -> AppResult<Json<SyncResponse>> {
    let limit = sync_query.limit;
   
    let user_id = user.user_id;
    let username = &user.username;

    if limit.is_some() && limit.unwrap() > 20 {
        return Err(AppError::BadRequest("Limit cannot exceed 20".into()));
    }

    sync_user_problems(&state.db, user_id, username, limit)
        .await
        .map_err(|e| AppError::InternalWith(e.to_string()))?;
    Ok(Json(SyncResponse { message: "Sync successful".into() }))
}