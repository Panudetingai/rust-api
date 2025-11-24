mod entity;
mod handlers;
mod routes;
mod utils;
use axum::Router;
use sea_orm::DatabaseConnection;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();
    dotenvy::dotenv().ok();

    let db_conn = utils::seaorm::connect_db().await;

    let state = AppState { db: db_conn };

    let app = Router::new()
        .nest("/api", routes::users::users_router().await)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8880").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Server running on http://0.0.0.0:8880");
}
