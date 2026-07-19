-- Add up migration script here


CREATE TABLE IF NOT EXISTS component_definition (
    id UUID PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL,
    name TEXT NOT NULL,
    game_image_url TEXT NOT NULL,
    menu_image_url TEXT NOT NULL,
    price INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS component_mount_point (
    id UUID PRIMARY KEY NOT NULL,
    component_definition_id UUID NOT NULL REFERENCES component_definition(id),
    accepts_kind TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS player (
    id UUID PRIMARY KEY NOT NULL,

    name VARCHAR(255) NOT NULL,
    score INT NOT NULL,
    money INT NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS map (
    id UUID PRIMARY KEY NOT NULL,
    player_id UUID NOT NULL REFERENCES player(id),

    map_data TEXT NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blueprint (
    id UUID PRIMARY KEY NOT NULL,
    player_id UUID NOT NULL REFERENCES player(id),

    name TEXT NOT NULL,
    buying_price INT NOT NULL,
    total_weight INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blueprint_component (
    id UUID PRIMARY KEY NOT NULL,
    blueprint_id UUID NOT NULL REFERENCES blueprint(id),
    
    component_definition_id UUID NOT NULL REFERENCES component_definition(id),

    blueprint_component_mount_point_id UUID,

    kind TEXT NOT NULL,
    game_image_url TEXT NOT NULL,
    menu_image_url TEXT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blueprint_component_mount_point (
    id UUID PRIMARY KEY NOT NULL,
    blueprint_component_id UUID NOT NULL REFERENCES blueprint_component(id),
    source_mount_point_id UUID NOT NULL REFERENCES component_mount_point(id),
    accepts_kind TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

ALTER TABLE blueprint_component
    ADD CONSTRAINT fk_blueprint_component_blueprint_component_mount_point
    FOREIGN KEY (blueprint_component_mount_point_id) REFERENCES blueprint_component_mount_point(id);

CREATE TABLE IF NOT EXISTS vehicle (
    id UUID PRIMARY KEY NOT NULL,
  
    player_id UUID NOT NULL REFERENCES player(id),
    blueprint_id UUID NOT NULL REFERENCES blueprint(id),

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS vehicle_on_map (
    id UUID PRIMARY KEY NOT NULL,
  
    player_id UUID NOT NULL REFERENCES player(id),
    vehicle_id UUID NOT NULL REFERENCES vehicle(id),

    x INT NOT NULL,
    y INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);