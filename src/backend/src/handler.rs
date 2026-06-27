
use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState, 
    blueprint_db::{BlueprintDb, insert_blueprint},
    blueprint_dto::BlueprintDto,
    component_definition_db::get_all_chassis_component_definitions,
    component_definition_dto::ComponentDefinitionDto,
    player_db::{PlayerDb, insert_player}, 
    player_dto::PlayerDto, 
    map_dto::MapDto, 
    map_db::{MapDb, insert_map}};

pub async fn create_blueprint(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>,
    Json(body): Json<serde_json::Value>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request to create blueprint for player_id: {}", player_id);

    let name = match body["name"].as_str() {
        Some(s) => s.to_string(),
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing name field"})))),
    };

    let new_blueprint = insert_blueprint(&data.db, player_id, &name).await;

    match new_blueprint {
        Ok(blueprint) => {
            let blueprint_dto = BlueprintDto {
                id: blueprint.id.to_string(),
                player_id: blueprint.player_id.to_string(),
                name: blueprint.name,
                buying_price: blueprint.buying_price,
                total_weight: blueprint.total_weight,
            };
            Ok(Json(json!(blueprint_dto)))
        }
        Err(err) => {
            eprintln!("Failed to create blueprint: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create blueprint"}))))
        }
    }
}

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
                money: player.money,
                name: player.name,
                score: player.score,
            };
            Ok(Json(json!(player_dto)))
        }
        Ok(None) => {
            let new_player = insert_player(&data.db, &body.name).await;

            match new_player {
                Ok(player) => {
                    let player_dto = PlayerDto {
                        id: player.id.to_string(),
                        money: player.money,
                        name: player.name,
                        score: player.score,
                    };

                    let _ = insert_map(&data.db, player.id).await;

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

pub async fn get_blueprints_of_player(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for blueprints of player_id: {}", player_id);

    let blueprints_lookup = sqlx::query_as!(
        BlueprintDb,
        r#"SELECT * FROM blueprint WHERE player_id = $1"#,
        player_id,
    )
    .fetch_all(&data.db)
    .await;

    match blueprints_lookup {
        Ok(blueprints) => {
            let blueprint_dtos: Vec<BlueprintDto> = blueprints.into_iter().map(|bp| BlueprintDto {
                id: bp.id.to_string(),
                player_id: bp.player_id.to_string(),
                name: bp.name,
                buying_price: bp.buying_price,
                total_weight: bp.total_weight,
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

pub async fn get_enemies(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for enemies of player_id: {}", player_id);

    let enemies_lookup = crate::player_db::get_enemies(&data.db, player_id).await;

    match enemies_lookup {
        Ok(enemies) => {
            let enemy_dtos: Vec<PlayerDto> = enemies.into_iter().map(|enemy| PlayerDto {
                id: enemy.id.to_string(),
                money: enemy.money,
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

pub async fn get_vehicel_types(State(data): State<Arc<AppState>>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for vehicle types");

    let vehicel_types_lookup = get_all_chassis_component_definitions(&data.db).await;

    match vehicel_types_lookup {
        Ok(components) => {
            let component_dtos: Vec<ComponentDefinitionDto> = components.into_iter().map(|component| component.into()).collect();
            Ok(Json(json!(component_dtos)))
        }
        Err(err) => {
            eprintln!("Failed to query vehicle types: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query vehicle types"}))))
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

