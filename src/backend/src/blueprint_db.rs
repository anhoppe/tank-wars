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
