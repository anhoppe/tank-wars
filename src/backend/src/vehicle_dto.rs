use serde::{Deserialize, Serialize};

use crate::blueprint_component_db::get_blueprint_component_by_id_and_kind;
use crate::blueprint_db::get_blueprint_by_id;
use crate::vehicle_db::VehicleDb;

#[derive(Debug, Deserialize, Serialize)]
pub struct VehicleDto {
    pub id: String,
    pub player_id: String,
    pub blueprint_id: String,
    pub game_image_url: String,
    pub name: String,
}

pub async fn vehicle_db_to_dto(pool: &sqlx::PgPool, vehicle_db: VehicleDb) -> VehicleDto {

    let blueprint = match get_blueprint_by_id(&pool, vehicle_db.blueprint_id).await {
        Ok(Some(blueprint)) => blueprint,
        Ok(None) => {
            // Handle the case where the blueprint is not found
            panic!("Blueprint not found for blueprint_id: {}", vehicle_db.blueprint_id);
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

    VehicleDto {
        id: vehicle_db.id.to_string(),
        player_id: vehicle_db.player_id.to_string(),
        blueprint_id: vehicle_db.blueprint_id.to_string(),
        game_image_url: blueprint_component.game_image_url,
        name: "".to_string(),
    }
}
