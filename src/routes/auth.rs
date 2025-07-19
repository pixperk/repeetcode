use axum::{routing::post, Router};

use crate::{auth::handler::{login_handler, signup_handler}, state::AppState};

pub fn auth_routes(state: AppState) -> Router {
    Router::new()
        .route("/login", post(login_handler))
        .route("/signup", post(signup_handler))
        .with_state(state)
}