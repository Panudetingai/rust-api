use crate::{AppState, entity::users, utils::lib::error::AppError};
use argon2::{
    Argon2,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use axum::{Json, extract::State};
use sea_orm::EntityTrait;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignupRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}
pub async fn signup(
    State(state): State<AppState>,
    Json(payload): Json<SignupRequest>,
) -> Result<Json<users::Model>, AppError> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    let start = std::time::Instant::now();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::HashError(e.to_string()))?
        .to_string();

    println!("Password hashing took: {:?}", start.elapsed());

    let new_user = users::ActiveModel {
        name: sea_orm::ActiveValue::set(payload.name),
        email: sea_orm::ActiveValue::set(payload.email),
        password: sea_orm::ActiveValue::set(password_hash),
        created_at: sea_orm::ActiveValue::set((chrono::Utc::now()).naive_utc()),
        updated_at: sea_orm::ActiveValue::set((chrono::Utc::now()).naive_utc()),
        ..Default::default()
    };

    let insert_result = users::Entity::insert(new_user)
        .exec(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    let created_user = users::Entity::find_by_id(insert_result.last_insert_id)
        .one(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    match created_user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFoundError()),
    }
}
