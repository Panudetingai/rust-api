use crate::{AppState, func::auth::verify_credentials::verify_credentials};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use axum_extra;
use jsonwebtoken::{EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let user_id = verify_credentials(State(state), &payload.email, &payload.password)
        .await
        .map_err(|err| {
            (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({ "error": err })),
            )
        })?;

    let exp = (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp();
    let claim = Claims {
        sub: user_id.to_string(),
        exp: exp as usize,
    };

    let token = encode(
        &Header::default(),
        &claim,
        &EncodingKey::from_secret(
            "eydGRcyR/yX/ICOwUYMjNtCMJLk7C3tDDduime8Jc3xLBktPj0o/ifuvEDB40A
            hxqqwhwFPEgPlq+dLRbDSz/YPu88Jws+PUlD6eWFFciooEMRkLchbA9ZUvVlM0Oa
            g6Pr1X+8pDLcv+cfDlARPceSo2ddVhS/3uuIXsJbJjf56P5lucvmpXOi6F8mjSUF
            m3wk+vNz/Gdj9MxPPf7uc0wsO6S45QcGI+Jg=="
                .as_ref(),
        ),
    )
    .unwrap();

    let cookie = axum_extra::extract::cookie::Cookie::build(("rust_api_token", token))
        .http_only(true)
        .secure(true)
        .same_site(axum_extra::extract::cookie::SameSite::Strict)
        .build();

    let jar = axum_extra::extract::cookie::CookieJar::new().add(cookie);

    Ok((
        StatusCode::OK,
        jar,
        Json(serde_json::json!({ "message": "Login successful" })),
    ))
}
