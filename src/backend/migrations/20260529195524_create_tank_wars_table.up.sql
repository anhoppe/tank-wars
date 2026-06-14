-- Add up migration script here
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

CREATE TABLE IF NOT EXISTS basic_tank (
    id UUID PRIMARY KEY NOT NULL,
    blueprint_id UUID NOT NULL REFERENCES blueprint(id),

    speed INT NOT NULL,
    speed_max INT NOT NULL,
    speed_price_exp INT NOT NULL,
    turret_speed INT NOT NULL,
    turret_speed_max INT NOT NULL,
    turret_speed_price_exp INT NOT NULL,
    machine_gun_damage INT NOT NULL,
    machine_gun_damage_max INT NOT NULL,
    machine_gun_damage_price_exp INT NOT NULL,
    machine_gun_reload_speed INT NOT NULL,
    machine_gun_reload_speed_max INT NOT NULL,
    machine_gun_reload_speed_price_exp INT NOT NULL,
    
    created_at TIMESTAMPTZ DEFAULT NOW()
);
