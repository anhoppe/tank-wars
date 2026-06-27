use serde::Serialize;

use crate::component_db::ComponentDb;

// Represents the vehicel types that can be foundation of a blueprint
#[derive(Serialize)]
pub struct ComponentDto {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub image_url: String,
    pub price: i32,
}

impl From<ComponentDb> for ComponentDto {
    fn from(component_db: ComponentDb) -> Self {
        ComponentDto {
            id: component_db.id.to_string(), // Convert Uuid to String
            kind: component_db.kind,
            name: component_db.name,
            image_url: component_db.image_url,
            price: component_db.price,
        }
    }
}