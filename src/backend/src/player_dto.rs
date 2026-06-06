use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerDto {
    pub id: String,
    pub name: String,
    pub score: i32,
}
