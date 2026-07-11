use serde::{Deserialize, Serialize};
use crate::{vehicle_on_map_db::VehicleOnMapDb,
    blueprint_db::get_blueprint_by_id,
    blueprint_component_db::get_blueprint_component_by_id_and_kind,
    vehicle_db::get_vehicle_by_id};

#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleOnMapDto {
    pub id: String,
    pub player_id: String,
    pub vehicle_id: String,
    pub x: i32,
    pub y: i32,
    pub game_image_url: String,
}

pub async fn vehicle_on_map_db_to_dto(pool: &sqlx::PgPool, vehicle_on_map_db: VehicleOnMapDb) -> VehicleOnMapDto {
  
    let vehicle = match get_vehicle_by_id(&pool, vehicle_on_map_db.vehicle_id).await {
        Ok(Some(v)) => v,
        Ok(None) => {
            // Handle the case where the vehicle is not found
            panic!("Vehicle not found for vehicle_id: {}", vehicle_on_map_db.vehicle_id);
        }
        Err(e) => {
            // Handle the database error
            panic!("Database error while fetching vehicle: {}", e);
        }
    };
    
    let blueprint = match get_blueprint_by_id(&pool, vehicle.blueprint_id).await {
        Ok(Some(blueprint)) => blueprint,
        Ok(None) => {
            // Handle the case where the blueprint is not found
            panic!("Blueprint not found for blueprint_id: {}", vehicle.blueprint_id);
        }
        Err(e) => {
            // Handle the database error
            panic!("Database error while fetching blueprint: {}", e);
        }
    };

    let blueprint_component = match get_blueprint_component_by_id_and_kind(&pool, blueprint.id, "chassis").await {
        Ok(Some(component)) => component,
        Ok(None) => {
            // Handle the case where the blueprint component is not found
            panic!("Blueprint component not found for blueprint_id: {}", blueprint.id);
        }
        Err(e) => {
            // Handle the database error
            panic!("Database error while fetching blueprint component: {}", e);
        }
    };

    VehicleOnMapDto {
        id: vehicle_on_map_db.id.to_string(),
        player_id: vehicle_on_map_db.player_id.to_string(),
        vehicle_id: vehicle_on_map_db.vehicle_id.to_string(),
        x: vehicle_on_map_db.x,
        y: vehicle_on_map_db.y,
        game_image_url: blueprint_component.game_image_url,
    }
}
