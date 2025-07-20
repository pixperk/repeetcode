use axum::{Json, extract::State};
use bcrypt::verify;
use serde::{Deserialize, Serialize};

use crate::{
    auth::{
        jwt::create_jwt, middleware::AuthenticatedUser, service::{create_user_if_not_exists, get_user_by_username, hash_password}
    },
    error::{AppError, AppResult},
    gql::client::check_user_exists,
    state::AppState,
};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct SignupPayload {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> AppResult<Json<AuthResponse>> {
    let user = get_user_by_username(&state.db, &payload.username)
        .await?;

    let hashed_pass = match &user.password{
            Some(pw) => pw,
            None => {
                return  Err(AppError::Internal);
            }
        };

    let valid =
        verify(&payload.password, hashed_pass).map_err(|_| AppError::Internal)?;
    if !valid {
        return Err(AppError::Unauthorized);
    }

    let token = create_jwt(user.id, user.username, &state);
    Ok(Json(AuthResponse { token }))
}

pub async fn signup_handler(
    State(state): State<AppState>,
    Json(payload): Json<SignupPayload>,
) -> AppResult<Json<AuthResponse>> {
    if !check_user_exists(&payload.username).await? {
        return Err(AppError::BadRequest("Invalid LeetCode user".into()));
    }

    let password = hash_password(&payload.password)?;
    let user =
        create_user_if_not_exists(&state.db, &payload.username, &payload.email, &password).await?;
      let token = create_jwt(user.id, user.username, &state);
    Ok(Json(AuthResponse { token }))
}

pub async fn current_user(
    State(state): State<AppState>,
    user: AuthenticatedUser,
) -> String{
    format!("Hello, {}! Your ID is {}", user.username, user.user_id)
}