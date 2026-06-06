use axum::{Router, routing::get};
use dotenv::dotenv;
use serde_json::json;
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;

mod handler;
mod player_db;
mod player_dto;
mod map_db;
mod map_dto;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Connected to DB successfully");
            pool
        }
        Err(err) => {
            println!("Failed to connect to DB: {}", err);
            std::process::exit(1);
        }
    };

    // build our application with a single route
    let app = Router::new()
        .route("/api/player", axum::routing::post(handler::get_or_create_player))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
