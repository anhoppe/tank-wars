use axum::{Router};
use sqlx::postgres::PgPool;
use tower_http::cors::{CorsLayer, Any};
use std::sync::Arc;

use crate::{
    AppState,
    handler::{buy_blueprint_for_player,
        buy_vehicle_for_player,
        get_enemies, 
        get_blueprints_of_player, 
        get_player_map, 
        set_player_map, 
        get_or_create_player, 
        get_vehicle_types,
        get_vehicles_of_player}
};

pub async fn serve(pool: &PgPool) {
        let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // build our application with a single route
    let app = Router::new()
        .route("/api/enemies/{player_id}", axum::routing::get(get_enemies))
        .route("/api/blueprints/{player_id}", axum::routing::get(get_blueprints_of_player).post(buy_blueprint_for_player))
        .route("/api/fleet/{player_id}", axum::routing::get(get_vehicles_of_player).post(buy_vehicle_for_player))
        .route("/api/map/{player_id}", axum::routing::get(get_player_map).put(set_player_map))
        .route("/api/player", axum::routing::post(get_or_create_player))
        .route("/api/vehicle-types", axum::routing::get(get_vehicle_types))
        .layer(cors)
        .with_state(Arc::new(AppState { db: pool.clone() }));

    // listen globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3001");
    axum::serve(listener, app).await.unwrap();
}
