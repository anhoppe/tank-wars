use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlueprintDto {
    pub id: String,
    pub name: String,
    pub research_cost: i32,
    pub player_id: i32,
}
