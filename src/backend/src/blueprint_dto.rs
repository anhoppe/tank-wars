use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BlueprintDto {
    #[serde(default)]
    pub id: String,
    pub player_id: String,
    
    pub name: String,
    #[serde(default)]
    pub buying_price: i32,
    #[serde(default)]
    pub total_weight: i32
}
