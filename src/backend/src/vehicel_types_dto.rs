use serde::Serialize;

// Represents the vehicel types that can be foundation of a blueprint
#[derive(Serialize)]
pub struct VehicelTypesDto {
    pub id: i32,
    pub name: String,
    pub image_url: String,
    pub price: i32,
}
