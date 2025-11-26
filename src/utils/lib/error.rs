use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use sea_orm::DbErr;
use serde_json::json;

pub enum AppError {
    NotFoundError(),
    DatabaseError(sea_orm::DbErr),
    UnauthorizedError(String),
    HashError(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFoundError() => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::UnauthorizedError(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::HashError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };
        let body = Json(json!({
            "error": error_message
        }));

        (status, body).into_response()
    }
}
