use axum::{Json, extract::State};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::Deserialize;
use uuid::Uuid;

use crate::{AppState, entity::users, utils::lib::error::AppError};

// GET /api/users
pub async fn get_user_all(State(state): State<AppState>) -> Json<Vec<users::Model>> {
    let user_all = users::Entity::find()
        .all(&state.db)
        .await
        .unwrap_or_default();
    Json(user_all)
}

// Get query parameters for creating a user
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]

pub struct GetUserQuery {
    pub id: Option<Uuid>,
    pub email: Option<String>,
    pub name: Option<String>,
}

pub async fn get_user_query(
    State(state): State<AppState>,
    Json(payload): Json<GetUserQuery>,
) -> Result<Json<users::Model>, AppError> {
    let mut query = users::Entity::find();

    let mut conditions = sea_orm::Condition::all();

    if let Some(id) = payload.id {
        conditions = conditions.add(users::Column::Id.eq(id));
    }

    if let Some(email) = payload.email {
        conditions = conditions.add(users::Column::Email.eq(email));
    }

    if let Some(name) = payload.name {
        conditions = conditions.add(users::Column::Name.eq(name));
    }

    query = query.filter(conditions);

    let user = query
        .one(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;
    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFoundError()),
    }
}

// POST /api/users (create user) - to be implemented
#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: Option<sea_orm::prelude::DateTime>,
}
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<users::Model>, AppError> {
    let active_user = users::ActiveModel {
        name: Set(payload.name),
        email: Set(payload.email),
        password: Set(payload.password),
        created_at: Set(payload
            .created_at
            .unwrap_or_else(|| chrono::Utc::now().naive_utc())),
        ..Default::default()
    };

    let _res = users::Entity::insert(active_user)
        .exec(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    let created_users = users::Entity::find_by_id(_res.last_insert_id)
        .one(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    match created_users {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFoundError()),
    }
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}
// update and delete handlers to be implemented
pub async fn update_user(
    State(state): State<AppState>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<users::Model>, AppError> {
    let updated_user = users::ActiveModel {
        id: Set(payload.id),
        name: Set(payload.name.unwrap_or_default()),
        email: Set(payload.email.unwrap_or_default()),
        password: Set(payload.password.unwrap_or_default()),
        ..Default::default()
    };

    let _res = users::Entity::update(updated_user)
        .exec(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    let user = users::Entity::find_by_id(_res.id)
        .one(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    match user {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFoundError()),
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DeleteRequest {
    pub id: Uuid,
}
// delete handler to be implemented
pub async fn delete_user(
    State(state): State<AppState>,
    Json(payload): Json<DeleteRequest>,
) -> Result<Json<users::Model>, AppError> {
    let delete_user = users::ActiveModel {
        id: Set(payload.id),
        ..Default::default()
    };

    let _res = users::Entity::delete(delete_user)
        .exec(&state.db)
        .await
        .map_err(AppError::DatabaseError)?;

    Err(AppError::NotFoundError())
}
