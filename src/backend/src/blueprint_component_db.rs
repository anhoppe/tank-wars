use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BlueprintComponentDb {
    pub id: Uuid,
    pub blueprint_id: Uuid,
    pub component_definition_id: Uuid,

    pub kind: String,
    pub image_url: String,
    
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_blueprint_component(
    pool: &sqlx::PgPool,
    blueprint_id: Uuid,
    component_definition_id: Uuid,
    kind: String,
    image_url: String,
) -> Result<BlueprintComponentDb, sqlx::Error> {
    let blueprint_component = sqlx::query_as!(
        BlueprintComponentDb,
        r#"
        INSERT INTO blueprint_component (id, blueprint_id, component_definition_id, kind, image_url)
        VALUES (gen_random_uuid(), $1, $2, $3, $4)
        RETURNING id, blueprint_id, component_definition_id, kind, image_url, created_at
        "#,
        blueprint_id,
        component_definition_id,
        kind,
        image_url
    )
    .fetch_one(pool)
    .await?;

    Ok(blueprint_component)
}

pub async fn get_blueprint_component_by_id_and_kind(
    pool: &sqlx::PgPool,
    blueprint_id: Uuid,
    kind: &str,
) -> Result<Option<BlueprintComponentDb>, sqlx::Error> {
    let blueprint_component = sqlx::query_as!(
        BlueprintComponentDb,
        r#"
        SELECT *
        FROM blueprint_component
        WHERE blueprint_id = $1 AND kind = $2
        "#,
        blueprint_id,
        kind
    )
    .fetch_optional(pool)
    .await?;

    Ok(blueprint_component)
}
