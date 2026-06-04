-- Add up migration script here
CREATE TABLE IF NOT EXISTS player (
    id SERIAL PRIMARY KEY,
    player_name VARCHAR(255) NOT NULL,
    score INT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW()
);
