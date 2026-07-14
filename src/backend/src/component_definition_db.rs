use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ComponentDefinitionDb {
     pub id: Uuid,
     pub kind: String,
     pub name: String,
     pub game_image_url: String,
     pub menu_image_url: String,
     pub price: i32,
     pub created_at: Option<chrono::DateTime<chrono::Utc>>,
 }
 
pub async fn insert_component_definition(
    pool: &sqlx::PgPool,
    kind: &str,
    name: &str,
    game_image_url: &str,
    menu_image_url: &str,
    price: i32,
) -> Result<Uuid, sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO component_definition (id, kind, name, game_image_url, menu_image_url, price)
        VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)
        RETURNING id
        "#,
        kind, name, game_image_url, menu_image_url, price
    )
    .fetch_one(pool)
    .await?.id;
}

pub async fn get_all_chassis_component_definitions(pool: &sqlx::PgPool) -> Result<Vec<ComponentDefinitionDb>, sqlx::Error> {
    let components = sqlx::query_as!(
        ComponentDefinitionDb,
        r#"
        SELECT *
        FROM component_definition
        WHERE kind = 'chassis'
        "#
    )
    .fetch_all(pool)
    .await?;

    for component in &components {
        println!("Component: {:?}", component);
    }

    Ok(components)
}

pub async fn get_component_definition_by_id(pool: &sqlx::PgPool, component_definition_id: Uuid) -> Result<ComponentDefinitionDb, sqlx::Error> {
    let component = sqlx::query_as!(
        ComponentDefinitionDb,
        r#"
        SELECT *
        FROM component_definition
        WHERE id = $1
        "#,
        component_definition_id
    )
    .fetch_one(pool)
    .await?;

    Ok(component)
}