use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BlueprintComponentMountPointDb {
    pub id: Uuid,
    pub blueprint_component_id: Uuid,
    pub source_mount_point_id: Uuid,
    pub accepts_kind: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn create_blueprint_component_mount_point(
    pool: &sqlx::PgPool,
    blueprint_component_id: Uuid,
    source_mount_point_id: Uuid,
    accepts_kind: String,
) -> Result<BlueprintComponentMountPointDb, sqlx::Error> {
    let blueprint_component_mount_point = sqlx::query_as!(
        BlueprintComponentMountPointDb,
        r#"
        INSERT INTO blueprint_component_mount_point (id, blueprint_component_id, source_mount_point_id, accepts_kind)
        VALUES (gen_random_uuid(), $1, $2, $3)
        RETURNING id, blueprint_component_id, source_mount_point_id, accepts_kind, created_at
        "#,
        blueprint_component_id, source_mount_point_id, accepts_kind
    )
    .fetch_one(pool)
    .await?;
    Ok(blueprint_component_mount_point)
}

pub async fn get_blueprint_component_mount_points_by_blueprint_component_id(
    pool: &sqlx::PgPool,
    blueprint_component_id: Uuid,
) -> Result<Vec<BlueprintComponentMountPointDb>, sqlx::Error> {
    let blueprint_component_mount_points = sqlx::query_as!(
        BlueprintComponentMountPointDb,
        r#"
        SELECT * FROM blueprint_component_mount_point WHERE blueprint_component_id = $1
        "#,
        blueprint_component_id
    ).fetch_all(pool).await?;
    Ok(blueprint_component_mount_points)
}
