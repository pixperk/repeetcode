use axum::{routing::{post, get}, Router};

use crate::{auth::handler::{current_user, login_handler, signup_handler}, state::AppState};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .route("/me", get(current_user))
        .with_state(state)
}