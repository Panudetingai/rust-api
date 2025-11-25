mod entity;
mod handlers;
mod routes;
mod utils;
use std::env;

use axum::{
    Router,
    extract::State,
    http::{
        HeaderValue,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use sea_orm::DatabaseConnection;
use tower_http::cors::{Any, CorsLayer};
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

    let cors_layer = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(
            env::var("FRONT_APP_URL")
                .unwrap()
                .parse::<HeaderValue>()
                .unwrap(),
        )
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let db_conn = utils::seaorm::connect_db().await;

    let state = AppState { db: db_conn };

    let app = Router::new()
        .nest("/api", routes::users::users_router().await)
        .nest("/api", routes::auth::auth_router().await)
        .with_state(state)
        .layer(cors_layer);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8880").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    println!("Server running on http://0.0.0.0:8880");
}
