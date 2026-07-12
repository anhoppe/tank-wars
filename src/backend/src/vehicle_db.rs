use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct VehicleDb {
    pub id: Uuid,
    pub player_id: Uuid,
    pub blueprint_id: Uuid,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_vehicle(
    pool: &sqlx::PgPool,
    player_id: Uuid,
    blueprint_id: Uuid,
) -> Result<VehicleDb, sqlx::Error> {
    sqlx::query_as!(
        VehicleDb,
        r#"
        INSERT INTO vehicle (id, player_id, blueprint_id)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING id, player_id, blueprint_id, created_at
        "#,
        player_id,
        blueprint_id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_vehicle_by_id(
    pool: &sqlx::PgPool,
    vehicle_id: Uuid,
) -> Result<Option<VehicleDb>, sqlx::Error> {
    sqlx::query_as!(
        VehicleDb,
        r#"
        SELECT *
        FROM vehicle
        WHERE id = $1
        "#,
        vehicle_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_vehicles_of_player(
    pool: &sqlx::PgPool,
    player_id: Uuid,
) -> Result<Vec<VehicleDb>, sqlx::Error> {
    sqlx::query_as!(
        VehicleDb,
        r#"
        SELECT *
        FROM vehicle
        WHERE player_id = $1
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_unused_vehicles_of_player(
    pool: &sqlx::PgPool,
    player_id: Uuid,
) -> Result<Vec<VehicleDb>, sqlx::Error> {
    sqlx::query_as!(
        VehicleDb,
        r#"
        SELECT *
        FROM vehicle
        WHERE player_id = $1
        AND id NOT IN (SELECT vehicle_id FROM vehicle_on_map)
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
}
