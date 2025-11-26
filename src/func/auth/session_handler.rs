use axum::response::IntoResponse;
use uuid::Uuid;


pub async fn create_session(
    user_id: Uuid,
) -> impl IntoResponse {
    let session_token = Uuid::new_v4().to_string();
}