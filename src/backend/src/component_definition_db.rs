use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ComponentDefinitionDb {
     pub id: Uuid,
     pub kind: String,
     pub name: String,
     pub image_url: String,
     pub price: i32,
     pub created_at: Option<chrono::DateTime<chrono::Utc>>,
 }
 
pub async fn insert_component_definition(
    pool: &sqlx::PgPool,
    kind: &str,
    name: &str,
    image_url: &str,
    price: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO component_definition (id, kind, name, image_url, price)
        VALUES (gen_random_uuid(), $1, $2, $3, $4)
        "#,
        kind, name, image_url, price
    )
    .execute(pool)
    .await?;

    Ok(())
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

    Ok(components)
}