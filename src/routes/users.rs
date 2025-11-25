use axum::Router;
use axum::routing::{delete, get, post, put};

use crate::AppState;
use crate::handlers::users::{create_user, delete_user, get_user_all, get_user_query, update_user};
pub async fn users_router() -> Router<AppState> {
    Router::new()
        .route("/users", get(get_user_all))
        .route("/user", post(get_user_query))
        .route("/user/create", post(create_user))
        .route("/user/update", put(update_user))
        .route("/user/delete", delete(delete_user))
}