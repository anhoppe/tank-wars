
use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
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

                    let map_db = sqlx::query_as!(
                        MapDb,
                        r#"INSERT INTO map (id, player_id, map_data, width, height) VALUES (gen_random_uuid(), $1, '{}', 20, 20)"#,
                        player.id,
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
