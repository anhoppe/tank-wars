# Database Layout

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Document the persistent data model — tables, columns, types, and relationships — as the single reference for all backend data access. |
| **Incoming** | DTO structs in `src/backend/src/` (`player_dto.rs`, `map_dto.rs`, `blueprint_dto.rs`) and SQL migrations in `src/backend/migrations/` |
| **Outgoing** | Ch. 5 Building Block View (backend component), Ch. 6 Runtime View (data-access scenarios), Ch. 9 Architecture Decisions (schema choices) |

---

## Entity-Relationship Diagram

```plantuml
@startuml db_layout

entity player {
  * id : UUID <<PK>>
  --
  * name : VARCHAR(255)
  * score : INT
  created_at : TIMESTAMPTZ
}

entity map {
  * id : UUID <<PK>>
  --
  * player_id : UUID <<FK>>
  * map_data : TEXT
  * width : INT
  * height : INT
  created_at : TIMESTAMPTZ
}

entity blueprint {
  * id : UUID <<PK>>
  --
  * player_id : UUID <<FK>>
  * vehicel_type : UUID <<FK>>
  * name : TEXT
  * buying_price : INT
  * total_weight : INT
  created_at : TIMESTAMPTZ
}

entity vehicel_type {
  * id : UUID <<PK>>
  --
  name : TEXT
  image_id : TEXT
}

entity basic_tank {
  * id : UUID <<PK>>
  * speed : INT
  * speed_max : INT
  * speed_price_exp : INT
  * turret_speed : INT
  * turret_speed_max : INT
  * turret_speed_price_exp : INT
  * machine_gun_damage : INT
  * machine_gun_damage_max : INT
  * machine_gun_damage_price_exp : INT
  * machine_gun_reload_speed : INT
  * machine_gun_reload_speed_max : INT
  * machine_gun_reload_speed_price_exp : INT
}

player ||--o{ map       : "owns"
player ||--o{ blueprint : "researches"
vehicel_type ||--o{ blueprint : "defines"
blueprint ||--o{ basic_tank : "upgrades"

@enduml
```

## Tables

### `player`

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| `id` | UUID | PK, NOT NULL | |
| `name` | VARCHAR(255) | NOT NULL | |
| `score` | INT | NOT NULL | |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | |

### `map`

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| `id` | UUID | PK, NOT NULL | |
| `player_id` | UUID | FK → player.id, NOT NULL | Owning player |
| `map_data` | TEXT | NOT NULL | Serialised map content |
| `width` | INT | NOT NULL | |
| `height` | INT | NOT NULL | |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | |

### `blueprint`

| Column | Type | Constraints | Notes |
|--------|------|-------------|-------|
| `id` | UUID | PK, NOT NULL | |
| `player_id` | UUID | FK → player.id, NOT NULL | Owning player |
| `combat_unit` | TEXT | NOT NULL | Unit type identifier |
| `created_at` | TIMESTAMPTZ | DEFAULT NOW() | |

## DTO Mapping

| Table | DTO struct | Notable differences |
|-------|-----------|---------------------|
| `player` | `PlayerDto` | `created_at` not exposed in DTO |
| `map` | `MapDto` | `player_id` not exposed; `created_at` is `Option<String>` |
| `blueprint` | `BlueprintDto` | Maps `combat_unit` column as `name`; exposes `research_cost` (not yet in DB schema) |
