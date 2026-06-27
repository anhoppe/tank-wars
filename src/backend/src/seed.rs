use crate::component_definition_db::insert_component_definition;

pub async fn seed_database(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    insert_component_definition(pool, 
        "chassis", 
        "Tank", 
        "vehicles/tank.png", 
        100).await?;

    insert_component_definition(pool, 
        "chassis", 
        "Truck", 
        "vehicles/truck.png", 
        50).await?;

    Ok(())
}
