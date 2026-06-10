
use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState, player_db::PlayerDb, player_dto::PlayerDto, map_dto::MapDto, map_db::MapDb};

pub async fn get_or_create_player(State(data): State<Arc<AppState>>,
    Json(body): Json<PlayerDto>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let player_lookup = sqlx::query_as!(
        PlayerDb,
        r#"SELECT * FROM player WHERE name = $1"#,
        &body.name,
    )
    .fetch_optional(&data.db)
    .await;

    match player_lookup {
        Ok(Some(player)) => {
            let player_dto = PlayerDto {
                id: player.id.to_string(),
                name: player.name,
                score: player.score,
            };
            Ok(Json(json!(player_dto)))
        }
        Ok(None) => {
            let new_player = sqlx::query_as!(
                PlayerDb,
                r#"INSERT INTO player (id, name, score) VALUES (gen_random_uuid(), $1, 0) RETURNING id, name, score, created_at"#,
                &body.name,
            )
            .fetch_one(&data.db)
            .await;

            match new_player {
                Ok(player) => {
                    let player_dto = PlayerDto {
                        id: player.id.to_string(),
                        name: player.name,
                        score: player.score,
                    };

                    let width: usize = 100;
                    let height: usize = 100;
                    let empty_map: Vec<Vec<i32>> = vec![vec![0; width]; height];
                    let map_data = serde_json::to_string(&empty_map).unwrap();

                    let map_db = sqlx::query_as!(
                        MapDb,
                        r#"INSERT INTO map (id, player_id, map_data, width, height) VALUES (gen_random_uuid(), $1, $2, $3, $4) RETURNING *"#,
                        player.id,
                        map_data,
                        width as i32,
                        height as i32,
                    )
                    .fetch_one(&data.db)
                    .await;

                    Ok(Json(json!(player_dto)))
                }
                Err(err) => {
                    eprintln!("Failed to create player: {}", err);
                    Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create player"}))))
                }
            }
        },
        Err(err) => {
            eprintln!("Failed to query player: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query player"}))))
        }
    }
}

pub async fn get_player_blueprints(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for blueprints of player_id: {}", player_id);

    let blueprints_lookup = sqlx::query_as!(
        PlayerDb,
        r#"SELECT * FROM blueprint WHERE player_id = $1"#,
        player_id,
    )
    .fetch_all(&data.db)
    .await;

    match blueprints_lookup {
        Ok(blueprints) => {
            let blueprint_dtos: Vec<PlayerDto> = blueprints.into_iter().map(|bp| PlayerDto {
                id: bp.id.to_string(),
                name: bp.name,
                score: 0, // Blueprints don't have a score, so we set it to 0
            }).collect();
            Ok(Json(json!(blueprint_dtos)))
        }
        Err(err) => {
            eprintln!("Failed to query blueprints: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query blueprints"}))))
        }
    }
}

pub async fn get_player_map(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for player_id: {}", player_id);

    let map_lookup = sqlx::query_as!(
        MapDb,
        r#"SELECT * FROM map WHERE player_id = $1"#,
        player_id,
    )
    .fetch_optional(&data.db)
    .await;

    match map_lookup {
        Ok(Some(map)) => {
            let map_dto = MapDto {
                id: map.id.to_string(),
                map_data: map.map_data,
                width: map.width,
                height: map.height,
                created_at: map.created_at.map(|t| t.to_string()),
            };
            Ok(Json(json!(map_dto)))
        }
        Ok(None) => Err((StatusCode::NOT_FOUND, Json(json!({"error": "Map not found"})))),
        Err(err) => {
            eprintln!("Failed to query map: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query map"}))))
        }
    }
}

pub async fn set_player_map(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>,
    Json(body): Json<serde_json::Value>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Set map for player_id: {}", player_id);

    let map_data = match body["mapData"].as_str() {
        Some(s) => s.to_string(),
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing mapData field"})))),
    };

    let update_result = sqlx::query!(
        r#"UPDATE map SET map_data = $1 WHERE player_id = $2"#,
        map_data,
        player_id,
    )
    .execute(&data.db)
    .await;

    match update_result {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            eprintln!("Failed to update map: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update map"}))))
        }
    }
}

pub async fn get_enemies(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for enemies of player_id: {}", player_id);

    let enemies_lookup = sqlx::query_as!(
        PlayerDb,
        r#"SELECT * FROM player WHERE id != $1"#,
        player_id,
    )
    .fetch_all(&data.db)
    .await;

    match enemies_lookup {
        Ok(enemies) => {
            let enemy_dtos: Vec<PlayerDto> = enemies.into_iter().map(|enemy| PlayerDto {
                id: enemy.id.to_string(),
                name: enemy.name,
                score: enemy.score,
            }).collect();
            Ok(Json(json!(enemy_dtos)))
        }
        Err(err) => {
            eprintln!("Failed to query enemies: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query enemies"}))))
        }
    }
}
