use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct MapDb {
    pub id: Uuid,
    pub player_id: Uuid,
    pub map_data: String,
    pub width: i32,
    pub height: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn insert_map(
    pool: &sqlx::PgPool,
    player_id: Uuid,
) -> Result<MapDb, sqlx::Error> {

    let width: i32 = 100;
    let height: i32 = 100;
    let empty_map: Vec<Vec<i32>> = vec![vec![0; width as usize]; height as usize];
    let map_data = serde_json::to_string(&empty_map).unwrap();

    sqlx::query_as!(
        MapDb,
        r#"
        INSERT INTO map (id, player_id, map_data, width, height)
        VALUES (gen_random_uuid(), $1, $2, $3, $4)
        RETURNING id, player_id, map_data, width, height, created_at
        "#,
        player_id,
        map_data,
        width,
        height
    )
    .fetch_one(pool)
    .await
}