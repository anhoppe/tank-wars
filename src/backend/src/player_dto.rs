use serde::{Deserialize, Serialize};
use crate::player_db::PlayerDb;

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

impl From<PlayerDb> for PlayerDto {
    fn from(player_db: PlayerDb) -> Self {
        PlayerDto {
            id: player_db.id.to_string(), // Convert Uuid to String
            money: player_db.money,
            name: player_db.name,
            score: player_db.score,
        }
    }
}
