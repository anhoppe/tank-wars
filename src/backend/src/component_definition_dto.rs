use serde::Serialize;

use crate::component_definition_db::ComponentDefinitionDb;

// Represents the vehicel types that can be foundation of a blueprint
#[derive(Serialize)]
pub struct ComponentDefinitionDto {
    pub id: String,
    pub kind: String,
    pub name: String,
    pub game_image_url: String,
    pub menu_image_url: String,
    pub price: i32,
}

impl From<ComponentDefinitionDb> for ComponentDefinitionDto {
    fn from(component_db: ComponentDefinitionDb) -> Self {
        ComponentDefinitionDto {
            id: component_db.id.to_string(), // Convert Uuid to String
            kind: component_db.kind,
            name: component_db.name,
            game_image_url: component_db.game_image_url,
            game_image_url: component_db.menu_image_url,
            price: component_db.price,
        }
    }
}