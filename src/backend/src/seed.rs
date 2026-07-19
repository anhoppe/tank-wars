use crate::{component_definition_db::insert_component_definition, 
    component_mount_point_db::insert_component_mount_point};

const CHASSIS_KIND: &str = "chassis";
const TURRET_KIND: &str = "turret";
const HEAVY_GUN_KIND: &str = "heavy_gun";
const LIGHT_GUN_KIND: &str = "light_gun";

pub async fn seed_database(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    let tank_id = insert_component_definition(pool, 
        CHASSIS_KIND, 
        "Tank", 
        "player/base.png",
        "vehicles/tank.png", 
        100).await?;

    insert_component_mount_point(pool, tank_id, TURRET_KIND).await?;

    let truck_id = insert_component_definition(pool, 
        CHASSIS_KIND, 
        "Truck", 
        "player/truck.png",
        "vehicles/truck.png",
        50).await?;

    insert_component_mount_point(pool, truck_id, LIGHT_GUN_KIND).await?;

    let scout_id = insert_component_definition(pool, 
        TURRET_KIND, 
        "Scout", 
        "player/scout.png",
        "vehicles/scout.png", 
        20).await?;
    insert_component_mount_point(pool, scout_id, HEAVY_GUN_KIND).await?;

    insert_component_definition(pool, 
        LIGHT_GUN_KIND, 
        "Light MG", 
        "player/light_mg.png",
        "vehicles/light_mg.png", 
        10).await?;

    insert_component_definition(pool, 
        HEAVY_GUN_KIND, 
        "Main Gun", 
        "player/main_gun.png",
        "vehicles/main_gun.png", 
        30).await?;

    Ok(())
}
