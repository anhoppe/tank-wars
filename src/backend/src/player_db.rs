use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayerDb {
    pub id: Uuid,
    pub money: i32,
    pub name: String,
    pub score: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get_enemies(
    pool: &sqlx::PgPool,
    player_id: Uuid,
) -> Result<Vec<PlayerDb>, sqlx::Error> {
    sqlx::query_as!(
        PlayerDb,
        r#"
        SELECT *
        FROM player
        WHERE id != $1
        "#,
        player_id
    )
    .fetch_all(pool)
    .await
}

pub async fn insert_player(
    pool: &sqlx::PgPool,
    name: &str,
) -> Result<PlayerDb, sqlx::Error> {
    sqlx::query_as!(
        PlayerDb,
        r#"
        INSERT INTO player (id, money, name, score)
        VALUES (gen_random_uuid(), 1000, $1, 0)
        RETURNING id, money, name, score, created_at
        "#,
        name
    )
    .fetch_one(pool)
    .await
}

