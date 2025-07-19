use axum::{extract::Path, routing::get, Json, Router};
use serde::Serialize;
use crate::{error::{AppResult}, gql::client::check_user_exists};

pub fn leetcode_routes()-> Router{
    Router::new()
    .route("/exists/{username}", get(user_exists))
}

#[derive(Serialize)]
struct ExistsResponse {
    username: String,
    exists: bool,
}

async fn user_exists(Path(username): Path<String>) -> AppResult<Json<ExistsResponse>> {
  let exists = check_user_exists(&username).await?;
  Ok(Json(ExistsResponse { username, exists }))
   
}