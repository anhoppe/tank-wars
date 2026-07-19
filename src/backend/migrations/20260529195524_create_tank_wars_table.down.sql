-- Add down migration script here
DROP TABLE IF EXISTS vehicle_on_map;
DROP TABLE IF EXISTS vehicle;

ALTER TABLE blueprint_component
    DROP CONSTRAINT IF EXISTS fk_blueprint_component_blueprint_component_mount_point;

DROP TABLE IF EXISTS blueprint_component_mount_point;
DROP TABLE IF EXISTS blueprint_component;
DROP TABLE IF EXISTS blueprint;

DROP TABLE IF EXISTS map;
DROP TABLE IF EXISTS player;

DROP TABLE IF EXISTS component_mount_point;
DROP TABLE IF EXISTS component_definition;
