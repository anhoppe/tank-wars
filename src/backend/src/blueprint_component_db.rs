use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
pub struct BlueprintComponentDb {
    pub id: Uuid,
    pub blueprint_id: Uuid,
    pub component_definition_id: Uuid,
}

pub async fn create_blueprint_component(
    pool: &sqlx::PgPool,
    blueprint_id: Uuid,
    component_definition_id: Uuid,
) -> Result<BlueprintComponentDb, sqlx::Error> {
    let blueprint_component = sqlx::query_as!(
        BlueprintComponentDb,
        r#"
        INSERT INTO blueprint_component (id, blueprint_id, component_definition_id)
        VALUES (gen_random_uuid(), $1, $2)
        RETURNING id, blueprint_id, component_definition_id
        "#,
        blueprint_id,
        component_definition_id
    )
    .fetch_one(pool)
    .await?;

    Ok(blueprint_component)
}
