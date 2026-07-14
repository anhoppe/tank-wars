use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BlueprintComponentDb {
    pub id: Uuid,
    pub blueprint_id: Uuid,
    pub component_definition_id: Uuid,

    pub mount_point_id: Uuid,
    pub parent_component_mount_point_id: Uuid,

    pub kind: String,
    pub game_image_url: String,
    pub menu_image_url: String,
    
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_blueprint_component(
    pool: &sqlx::PgPool,
    blueprint_id: Uuid,
    component_definition_id: Uuid,
    mount_point_id: Uuid,
    parent_component_mount_point_id: Uuid,
    kind: String,
    game_image_url: String,
    menu_image_url: String,
) -> Result<BlueprintComponentDb, sqlx::Error> {
    let blueprint_component = sqlx::query_as!(
        BlueprintComponentDb,
        r#"
        INSERT INTO blueprint_component (id, blueprint_id, component_definition_id, mount_point_id, parent_component_mount_point_id, kind, game_image_url, menu_image_url)
        VALUES (gen_random_uuid(), $1, $2, $3, $4, $5)
        RETURNING id, blueprint_id, component_definition_id, mount_point_id, parent_component_mount_point_id, kind, game_image_url, menu_image_url, created_at
        "#,
        blueprint_id,
        component_definition_id,
        mount_point_id,
        parent_component_mount_point_id,
        kind,
        game_image_url,
        menu_image_url
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
