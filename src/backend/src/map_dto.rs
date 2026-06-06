use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct MapDto {
    pub id: String,
    pub map_data: String,
    pub width: i32,
    pub height: i32,
    pub created_at: Option<String>,
}
