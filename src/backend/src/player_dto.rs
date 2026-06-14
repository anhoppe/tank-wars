use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub money: i32,
    pub name: String,
    #[serde(default)]
    pub score: i32,
}
