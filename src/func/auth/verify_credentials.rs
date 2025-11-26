use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::extract::State;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use tokio::task;
use uuid::Uuid;

use crate::{AppState, entity::users};

pub async fn verify_credentials(
    State(state): State<AppState>,
    email: &str,
    password: &str,
) -> Result<Uuid, String> {
    let user = users::Entity::find()
        .filter(users::Column::Email.eq(email))
        .one(&state.db)
        .await
        .map_err(|err| format!("Database error: {}", err))?
        .ok_or("Invalid email or password".to_string())?;

    let password_hash = user.password.clone();
    let password_input = password.to_string();

    task::spawn_blocking(move || {
        let parsed_hash =
            PasswordHash::new(&password_hash).map_err(|_| "Invalid password hash".to_string())?;

        Argon2::default()
            .verify_password(password_input.as_bytes(), &parsed_hash)
            .map_err(|_| "Invalid email or password".to_string())
    })
    .await
    .map_err(|err| format!("Task error: {}", err))??;

    Ok(user.id)
}
