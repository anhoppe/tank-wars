use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ComponentMountPointDb {
    pub id: Uuid,
    pub component_definition_id: Uuid,
    pub accepts_kind: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn insert_component_mount_point(pool: &sqlx::PgPool,
    component_definition_id: Uuid,
    accepts_kind: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO component_mount_point (id, component_definition_id, accepts_kind)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING id
        "#,
        component_definition_id,
        accepts_kind,
    )
    .fetch_one(pool)
    .await?;
    Ok(())
}

pub async fn get_component_mount_points_by_component_definition_id(pool: &sqlx::PgPool, 
    component_definition_id: Uuid) 
    -> Result<Vec<ComponentMountPointDb>, sqlx::Error> {
    let result = sqlx::query_as!(
        ComponentMountPointDb,
        r#"
        SELECT id, component_definition_id, accepts_kind, created_at
        FROM component_mount_point
        WHERE component_definition_id = $1
        "#,
        component_definition_id,
    )
    .fetch_all(pool)
    .await?;

    Ok(result)
}
