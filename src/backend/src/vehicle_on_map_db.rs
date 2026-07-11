use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct VehicleOnMapDb {
    pub id: Uuid,
    pub player_id: Uuid,
    pub vehicle_id: Uuid,
    pub x: i32,
    pub y: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_vehicles_on_map(
    pool: &sqlx::PgPool,
    player_id: Uuid,
) -> Result<Vec<VehicleOnMapDb>, sqlx::Error> {
    sqlx::query_as!(
        VehicleOnMapDb,
        r#"
        SELECT *
        FROM vehicle_on_map
        WHERE player_id = $1
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
}

pub async fn place_vehicle_on_map(
    pool: &sqlx::PgPool,
    player_id: Uuid,
    vehicle_id: Uuid,
    x: i32,
    y: i32,
) -> Result<VehicleOnMapDb, sqlx::Error> {
    let id = Uuid::new_v4();
    let created_at = chrono::Utc::now();

    sqlx::query_as!(
        VehicleOnMapDb,
        r#"
        INSERT INTO vehicle_on_map (id, player_id, vehicle_id, x, y, created_at)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
        "#,
        id,
        player_id,
        vehicle_id,
        x,
        y,
        created_at
    )
    .fetch_one(pool)
    .await
}