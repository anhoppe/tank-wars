-- Add up migration script here
CREATE TABLE IF NOT EXISTS player (
    id UUID PRIMARY KEY NOT NULL,
    name VARCHAR(255) NOT NULL,
    score INT NOT NULL,
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
    combat_unit TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);


