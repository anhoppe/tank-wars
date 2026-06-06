use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct PlayerDb {
    pub id: uuid::Uuid,
    pub name: String,
    pub score: i32,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
