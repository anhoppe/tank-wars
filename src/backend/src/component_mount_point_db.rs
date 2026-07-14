use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct ComponentMountPointDb {
    pub id: Uuid,
    pub component_definition_id: Uuid,
    pub accepts_kind: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn insert_component_mount_point(ppol: &sqlx::PgPool,
    component_definition_id: Uuid,
    accepts_kind: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO component_mount_point (id, component_definition_id, accepts_kind)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING id
    )
    .execute(pool)
    .await?;
    Ok(())
}
