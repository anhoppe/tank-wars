
use std::sync::Arc;

use axum::{Json, extract::{Path, State}, http::StatusCode, response::IntoResponse};
use serde_json::json;

use crate::{AppState, 
    blueprint_component_db::create_blueprint_component,
    blueprint_db::{BlueprintDb, create_blueprint, get_blueprint_by_id},
    blueprint_dto::BlueprintDto,
    component_definition_db::{get_component_definition_by_id, get_all_chassis_component_definitions},
    component_definition_dto::ComponentDefinitionDto,
    player_db::{PlayerDb, get_player_by_id, insert_player, update_player}, 
    player_dto::PlayerDto, 
    map_dto::MapDto, 
    map_db::{MapDb, insert_map, set_map_data},
    vehicle_db::{create_vehicle},
    vehicle_dto::vehicle_db_to_dto,
};

type ApiError = (StatusCode, Json<serde_json::Value>);
type ApiResult<T> = Result<T, ApiError>;

pub async fn buy_blueprint_for_player(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>,
    Json(body): Json<serde_json::Value>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let component_definition_id = match body["componentDefinitionId"].as_str() {
        Some(s) => match uuid::Uuid::parse_str(s) {
            Ok(id) => id,
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid componentDefinitionId format"})))),
        }
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing componentDefinitionId field"})))),
    };

    println!("Received request to buy blueprint for player_id: {}, component_definition_id: {}", player_id, component_definition_id);

    let chassis = match get_component_definition_by_id(&data.db, component_definition_id).await {
        Ok(chassis) => chassis,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Component definition not found"})))),
    };

    let mut player = load_player_or_404(&data.db, player_id).await?;

    if player.money < chassis.price {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Insufficient funds"}))));
    }

    player.money -= chassis.price;
    let player = update_player(&data.db, player)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update player"}))))?;

    let blueprint_name = chassis.name.clone();
    let blueprint = create_blueprint(&data.db, player_id, blueprint_name, chassis.price)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create blueprint"}))))?;
    
    create_blueprint_component(&data.db, blueprint.id, component_definition_id, chassis.kind, chassis.game_image_url, chassis.menu_image_url)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create blueprint component"}))))?;

    let player_dto: PlayerDto = player.into();

    Ok(Json(json!(player_dto)))
}

pub async fn buy_vehicle_for_player(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>,
    Json(body): Json<serde_json::Value>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    
    let blueprint_id = match body["blueprintId"].as_str() {
        Some(s) => match uuid::Uuid::parse_str(s) {
            Ok(id) => id,
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid blueprintId format"})))),
        }
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing blueprintId field"})))),
    };

    let blueprint = match get_blueprint_by_id(&data.db, blueprint_id).await {
        Ok(blueprint) => blueprint,
        Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Blueprint not found"})))),
    };

    let blueprint = match blueprint {
        Some(bp) => bp,
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Blueprint not found"})))),
    };

    let mut player = load_player_or_404(&data.db, player_id).await?;

    if player.money < blueprint.buying_price {
        return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Insufficient funds"}))));
    }

    player.money -= blueprint.buying_price;

    create_vehicle(&data.db, player_id, blueprint_id)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to create vehicle"}))))?;


    let player_dto: PlayerDto = player.into();

    Ok(Json(json!(player_dto)))
}

pub async fn get_vehicles_of_player(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let vehicles = crate::vehicle_db::get_vehicles_of_player(&data.db, player_id)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query vehicles"}))))?;

    let mut vehicles_dtos = Vec::with_capacity(vehicles.len());
    for vehicle in vehicles {
        let vehicle_dto = vehicle_db_to_dto(&data.db, vehicle).await;
        vehicles_dtos.push(vehicle_dto);
    }
    Ok(Json(json!(vehicles_dtos)))
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
                    let _ = insert_map(&data.db, player.id).await;
                    let player_dto: PlayerDto = player.into();

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

pub async fn get_vehicles_on_map(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    println!("Received request for vehicles on map of player_id: {}", player_id);

    let vehicles_on_map_lookup = crate::vehicle_on_map_db::get_vehicles_on_map(&data.db, player_id).await;

    match vehicles_on_map_lookup {
        Ok(vehicles_on_map) => {
            let mut vehicle_dtos = Vec::with_capacity(vehicles_on_map.len());
            for vehicle_on_map in vehicles_on_map {
                let vehicle_dto = crate::vehicle_on_map_dto::vehicle_on_map_db_to_dto(&data.db, vehicle_on_map).await;
                vehicle_dtos.push(vehicle_dto);
            }
            Ok(Json(json!(vehicle_dtos)))
        }
        Err(err) => {
            eprintln!("Failed to query vehicles on map: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query vehicles on map"}))))
        }
    }
}

pub async fn get_vehicle_types(State(data): State<Arc<AppState>>)
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

pub async fn place_vehicle_on_map(State(data): State<Arc<AppState>>,
    Path(player_id): Path<uuid::Uuid>,
    Json(body): Json<serde_json::Value>)
    -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {

    let vehicle_id = match body["vehicleId"].as_str() {
        Some(s) => match uuid::Uuid::parse_str(s) {
            Ok(id) => id,
            Err(_) => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Invalid vehicleId format"})))),
        }
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing vehicleId field"})))),
    };

    let x = match body["x"].as_i64() {
        Some(x) => x as i32,
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing or invalid x field"})))),
    };

    let y = match body["y"].as_i64() {
        Some(y) => y as i32,
        None => return Err((StatusCode::BAD_REQUEST, Json(json!({"error": "Missing or invalid y field"})))),
    };

    println!("Received request to place vehicle on map for player_id: {}, vehicle_id: {}, x: {}, y: {}", player_id, vehicle_id, x, y);

    let placed_vehicle = crate::vehicle_on_map_db::place_vehicle_on_map(&data.db, player_id, vehicle_id, x, y).await;

    match placed_vehicle {
        Ok(vehicle_on_map) => {
            let vehicle_dto = crate::vehicle_on_map_dto::vehicle_on_map_db_to_dto(&data.db, vehicle_on_map).await;
            Ok(Json(json!(vehicle_dto)))
        }
        Err(err) => {
            eprintln!("Failed to place vehicle on map: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to place vehicle on map"}))))
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

    let update_result = set_map_data(&data.db, player_id, map_data).await;

    match update_result {
        Ok(_) => Ok(StatusCode::OK),
        Err(err) => {
            eprintln!("Failed to update map: {}", err);
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update map"}))))
        }
    }
}

async fn load_player_or_404(pool: &sqlx::PgPool, player_id: uuid::Uuid) -> ApiResult<PlayerDb> {
    let player = get_player_by_id(pool, player_id)
    .await
    .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to query player"}))))?;

    let player = match player {
        Some(player) => player,
        None => return Err((StatusCode::NOT_FOUND, Json(json!({"error": "Player not found"})))),
    };
    Ok(player)
}