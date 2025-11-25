use axum::{Router, routing::post};

use crate::AppState;

pub async fn signup_router() -> Router<AppState> {
    Router::new().route("/auth/signup", post(|| async { "Signup ))
}