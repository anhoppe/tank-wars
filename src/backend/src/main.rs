use dotenv::dotenv;
use sqlx::{PgPool, postgres::PgPoolOptions};

mod blueprint_db;
mod blueprint_dto;
mod component_db;
mod component_dto;
mod handler;
mod map_db;
mod map_dto;
mod player_db;
mod player_dto;
mod seed;
mod server;

use server::serve;
use crate::seed::seed_database;

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

    let args: Vec<String> = std::env::args().collect();

    if (args.len() < 2) || (args[1] != "serve" && args[1] != "seed") {
        println!("Usage: {} [serve|seed]", args[0]);
        std::process::exit(1);
    }

    match args[1].as_str() {
        "serve" => {
            serve(&pool).await;
        }
        "seed" => {
            seed_database(&pool).await.expect("Failed to seed database");
        }
        _ => {
            println!("Unknown command: {}", args[1]);
            std::process::exit(1);
        }
    }
}
