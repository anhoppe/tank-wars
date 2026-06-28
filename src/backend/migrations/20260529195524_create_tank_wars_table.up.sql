-- Add up migration script here

CREATE TABLE IF NOT EXISTS component_definition (
    id UUID PRIMARY KEY NOT NULL,
    kind TEXT NOT NULL,
    name TEXT NOT NULL,
    image_url TEXT NOT NULL,
    price INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS player (
    id UUID PRIMARY KEY NOT NULL,

    name VARCHAR(255) NOT NULL,
    score INT NOT NULL,
    money INT NOT NULL,
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

CREATE TABLE IF NOT EXISTS map (
    id UUID PRIMARY KEY NOT NULL,
    player_id UUID NOT NULL REFERENCES player(id),

    map_data TEXT NOT NULL,
    width INT NOT NULL,
    height INT NOT NULL,

    created_at TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS blueprint_component (
    id UUID PRIMARY KEY NOT NULL,
  
    blueprint_id UUID NOT NULL REFERENCES blueprint(id),
    component_definition_id UUID NOT NULL REFERENCES component_definition(id),

    created_at TIMESTAMPTZ DEFAULT NOW()
);
