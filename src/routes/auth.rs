use crate::AppState;
use crate::handlers::{signin, signup};
use axum::{Router, routing::post};

pub async fn auth_router() -> Router<AppState> {
    Router::new()
        .route("/auth/signin", post(signin::signin))
        .route("/auth/signup", post(signup::signup))
}
