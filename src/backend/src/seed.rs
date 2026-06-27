use crate::component_db::insert_component;

pub async fn seed_database(pool: &sqlx::PgPool) -> Result<(), sqlx::Error> {
    insert_component(pool, 
        "chassis", 
        "Tank", 
        "vehicles/tank.png", 
        100).await?;

    insert_component(pool, 
        "chassis", 
        "Truck", 
        "vehicles/truck.png", 
        50).await?;

    Ok(())
}
