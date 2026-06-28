use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BlueprintDb {
    pub id: Uuid,
    pub player_id: Uuid,
    pub name: String,
    pub buying_price: i32,
    pub total_weight: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_blueprint(
    pool: &sqlx::PgPool,
    player_id: Uuid,
    name: String,
) -> Result<BlueprintDb, sqlx::Error> {
    sqlx::query_as!(
        BlueprintDb,
        r#"
        INSERT INTO blueprint (id, player_id, name, buying_price, total_weight)
        VALUES (gen_random_uuid(), $1, $2, 0, 0)
        RETURNING id, player_id, name, buying_price, total_weight, created_at
        "#,
        player_id,
        name
    )
    .fetch_one(pool)
    .await
}
