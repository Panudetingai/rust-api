use crate::{AppState, entity::users, utils::lib::error::AppError};
use argon2::{Argon2, PasswordHash, password_hash::PasswordVerifier};
use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
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
) -> Result<Json<users::Model>, AppError> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(payload.email))
        .one(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    match user {
        Some(user) => {
            let parsed_hash = PasswordHash::new(&user.password)
                .map_err(|_| AppError::UnauthorizedError("Invalid email or password".into()))?;
            let argon2 = Argon2::default();
            argon2
                .verify_password(payload.password.as_bytes(), &parsed_hash)
                .map_err(|_| AppError::UnauthorizedError("Invalid email or password".into()))?;

            Ok(Json(user))
        }
        None => Err(AppError::UnauthorizedError(
            "Invalid email or password".into(),
        )),
    }
}
