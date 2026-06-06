use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    #[serde(default)]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub score: i32,
}
