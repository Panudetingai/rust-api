use crate::{
    AppState,
    func::auth::login_handler::{self},
};
use axum::{Json, extract::State, response::IntoResponse};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SigninRequest {
    pub email: String,
    pub password: String,
}

pub async fn signin(
    State(state): State<AppState>,
    Json(payload): Json<SigninRequest>,
) -> impl axum::response::IntoResponse {
    let login = login_handler::login_handler(
        State(state),
        Json(login_handler::LoginPayload {
            email: payload.email.clone(),
            password: payload.password.clone(),
        }),
    )
    .await;

    match login {
        Ok(response) => response.into_response(),
        Err(err_response) => err_response.into_response(),
    }
}
