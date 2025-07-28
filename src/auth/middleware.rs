use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
    
};
use reqwest::header::AUTHORIZATION;

use crate::{auth::jwt::{decode_jwt}, error::AppError, state::AppState};

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id : uuid::Uuid,
    pub username: String,
}

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        
       let app_state = AppState::from_ref(state);
       

        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .to_str()
            .map_err(|_| AppError::Unauthorized)?
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;


        let token_data = decode_jwt(token, &app_state).map_err(|_| AppError::Unauthorized)?;

        Ok(AuthenticatedUser {
            user_id: uuid::Uuid::parse_str(&token_data.sub)
                .map_err(|_| AppError::Unauthorized)?,
            username: token_data.username,
        })
    }
}
