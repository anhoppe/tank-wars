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
