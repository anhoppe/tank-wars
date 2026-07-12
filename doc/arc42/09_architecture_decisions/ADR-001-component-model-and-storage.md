# ADR-001 - Component Model and Storage for Combat Vehicles

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Decide how combat vehicle customization is represented in code and persisted in the database, balancing flexibility, validation, and balancing velocity. |
| **Incoming** | Ch. 1 gameplay flexibility goals, Ch. 2 Rust/PostgreSQL constraints, v1 tech-tree documents in doc/v1. |
| **Outgoing** | Ch. 5 building blocks (blueprint and rule engine), Ch. 6 runtime validation/stats pipeline, Ch. 8 data modeling concept. |

## Status

Accepted (Opt C); amended 2026-07-12 (mount points; simplified v1 scope)

## Date

2026-06-27 (original); 2026-07-12 (mount-point amendment, simplified)

## Context

The current schema stores fixed combat stats in `basic_tank`, which is suitable for one vehicle archetype but does not scale to composable builds (`chassis -> mount -> turret -> weapon -> utility`) and compatibility rules defined in v1 documentation.

We need:
- Data-driven expansion of components without code rewrites for every new part.
- Dedicated properties for specific component kinds (for example turret rotation speed).
- Fast and explainable validation for requirements, incompatibilities, and balancing rules.

## Decision

Use a hybrid component model (Opt C):
- Generic component envelope for shared fields and rule graph behavior.
- Typed component payload per component kind for dedicated properties.
- Data-driven compatibility and dependency rules (tags + explicit rules), not inheritance trees.

For storage, use a primarily normalized schema with optional JSONB extension fields for low-frequency experimental attributes.

## Options Considered

### Option A - Deep class hierarchy with subclass-specific properties

Description:
- Represent components mainly through class inheritance (for example `Component -> Mount -> Turret`).

Pros:
- Strong compile-time typing in code.
- Simple when component set is small and stable.

Cons:
- Rigid and expensive to evolve for content-heavy systems.
- Encourages logic spread across subclasses instead of centralized rule evaluation.
- Harder to let designers/balance work mostly through data.

Decision:
- Rejected.

### Option B - Fully generic JSON document for all components

Description:
- Keep all component fields in JSONB with minimal relational structure.

Pros:
- Very flexible and quick for experimentation.
- Few migrations required for new fields.

Cons:
- Weak constraints and referential integrity.
- Harder query performance and indexing for gameplay-critical paths.
- Increased risk of runtime validation bugs.

Decision:
- Rejected as primary model.

### Option C - Normalized core + typed details + optional JSONB extensions

Description:
- Shared fields in relational tables.
- Kind-specific detail tables for dedicated properties.
- Optional `extra_json` for experimental attributes.

Pros:
- Strong integrity for ids, slots, tags, compatibility, and progression queries.
- Clear separation between shared and dedicated fields.
- Extensible without large inheritance refactors.

Cons:
- More tables and migrations than pure JSON.
- Slightly higher upfront modeling effort.

Decision:
- Accepted.

## Recommended Schema (v1)

### Scope: implement now vs. later

**Implement now** (v1 mount ticket): four tables — `component_definition`, `component_mount_point`, `blueprint`, `blueprint_component`. Mount compatibility is a single rule: a child may attach to a parent host only if the host offers a mount point whose `accepts_kind` equals the child's `kind`.

**Defer** (iterate later): `component_tag`, `component_requirement`, `component_incompatibility`, `component_stat_modifier`, typed detail tables (`turret_component_detail`, …), `blueprint_rule_evaluation`, and extra catalog fields (`code`, `parent_slot_code`, `power_draw`, …). See [Deferred schema](#deferred-schema) at the end of this section.

### Schema dependency diagram (v1 — implement now)

```plantuml
@startuml component_schema_dependencies

skinparam linetype ortho
skinparam packageStyle rectangle

title Component schema — v1 tables (implement now)

package "Component catalog (global, seeded)" #E8F4FD {

  entity component_definition {
    * id : UUID <<PK>>
    --
    * kind : TEXT
    * name : TEXT
    game_image_url : TEXT
    menu_image_url : TEXT
    price : INT
    ..
  }

  entity component_mount_point {
    * id : UUID <<PK>>
    --
    * host_component_id : UUID <<FK>>
    * accepts_kind : TEXT
  }
}

package "Player blueprint assembly" #FDF6E8 {

  entity blueprint {
    * id : UUID <<PK>>
    --
    * player_id : UUID <<FK>>
    name : TEXT
    buying_price : INT
    total_weight : INT
    ..
  }

  entity blueprint_component {
    * id : UUID <<PK>>
    --
    * blueprint_id : UUID <<FK>>
    * component_definition_id : UUID <<FK>>
    parent_blueprint_component_id : UUID <<FK>>
    mount_point_id : UUID <<FK>>
    kind : TEXT
    game_image_url : TEXT
    menu_image_url : TEXT
    ..
  }
}

component_definition ||--o{ component_mount_point : "host_component_id"

component_definition ||--o{ blueprint_component : "component_definition_id"

component_mount_point ||--o{ blueprint_component : "mount_point_id"

blueprint ||--o{ blueprint_component : "blueprint_id"

blueprint_component ||--o{ blueprint_component : "parent_blueprint_component_id"

note right of component_mount_point
  **Mount rule (v1)**
  Child mounts on parent host when:
  child.kind = accepts_kind
  --
  Example: chassis mount point
  accepts_kind = turret
  Scout turret has kind = turret → fits
end note

note bottom of blueprint_component
  Root chassis row:
  parent and mount_point_id are NULL.
  Child rows point at parent row
  and the mount_point_id used.
end note

@enduml
```

### Component catalog (implement now)

- `component_definition` — global parts catalog (exists today; extend `kind` usage)
  - `id UUID PK`
  - `kind TEXT` — what this part **is** (`chassis`, `turret`, `weapon`, …)
  - `name TEXT`
  - `game_image_url TEXT`
  - `menu_image_url TEXT`
  - `price INT`

- `component_mount_point` — slots a host part **offers** (new)
  - `id UUID PK`
  - `host_component_id UUID FK -> component_definition.id` — host that offers the slot (for example Tank chassis)
  - `accepts_kind TEXT` — `kind` value a child must have to mount here (for example `turret`, `weapon`)
  - UNIQUE (`host_component_id`, `accepts_kind`) — one slot per accepted kind per host for v1

**Mount rule:** child `component_definition.kind` must equal `component_mount_point.accepts_kind` on the parent's catalog definition.

### Player blueprint assembly (implement now)

- `blueprint` — unchanged (player-owned design; `buying_price`, `total_weight`, …)

- `blueprint_component` — installed parts on a blueprint (extend existing table)
  - `id UUID PK`
  - `blueprint_id UUID FK -> blueprint.id`
  - `component_definition_id UUID FK -> component_definition.id`
  - `parent_blueprint_component_id UUID NULL FK -> blueprint_component.id` — assembly tree (NULL for root chassis)
  - `mount_point_id UUID NULL FK -> component_mount_point.id` — which host slot was used (NULL for root chassis)
  - denormalized snapshots as today: `kind`, `game_image_url`, `menu_image_url`

### Schema references (v1)

| From | To | Purpose |
|------|-----|---------|
| `component_mount_point.host_component_id` | `component_definition.id` | Host offers the slot |
| `blueprint_component.component_definition_id` | `component_definition.id` | Installed part |
| `blueprint_component.mount_point_id` | `component_mount_point.id` | Slot used on parent |
| `blueprint_component.parent_blueprint_component_id` | `blueprint_component.id` | Parent in assembly tree |

**Install validation (v1):**

1. Resolve parent `blueprint_component` (chassis for turret or truck MG; turret for tank main gun).
2. Load parent's `component_definition` → its `component_mount_point` rows.
3. Find a mount point where `accepts_kind` = child's `kind`.
4. Fail if that `mount_point_id` is already used by another child of the same parent.
5. Insert `blueprint_component` with `parent_blueprint_component_id` and `mount_point_id`.

**Empty-slot UI:** list `component_mount_point` rows for the blueprint's chassis (or selected parent), minus mount points already referenced by sibling `blueprint_component` rows.

**v1 seed target:**

| Host | `accepts_kind` | Notes |
|------|----------------|-------|
| Tank chassis | `turret` | Then turret hosts `weapon` |
| Truck chassis | `weapon` | Fixed MG only; no turret slot |
| Scout turret | `weapon` | Main gun on turret |

```text
Tank:
  component_definition (Tank, kind=chassis)
    └── component_mount_point (accepts_kind=turret)
          └── blueprint_component (Scout turret)
                └── component_mount_point on turret (accepts_kind=weapon)

Truck:
  component_definition (Truck, kind=chassis)
    └── component_mount_point (accepts_kind=weapon)
          └── blueprint_component (Light MG)
```

### Deferred schema

Tables and fields planned per the full Opt C model; not part of the first mount-point migration:

- `component_tag`, `component_requirement`, `component_incompatibility`, `component_stat_modifier`
- `turret_component_detail`, `weapon_component_detail`, `mobility_component_detail`
- `blueprint_rule_evaluation`
- Extra `component_definition` fields: `code`, `parent_slot_code`, `slot_type`, `base_cost`, `base_weight`, `power_draw`, `power_supply`, `extra_json`
- Extra `component_mount_point` fields: `slot_code`, `accepts_tags`, `max_children`, `required`
- Extra `blueprint_component` fields: `mount_point_code`, `mount_index`

### Full target schema (deferred — Opt C vision)

The sections below describe the longer-term model. **Do not implement in the first mount-point migration.**

#### Component catalog (full target)

- `component_definition`
  - `id UUID PK`
  - `code TEXT UNIQUE`
  - `name TEXT`
  - `kind TEXT` (`chassis`, `mount`, `turret`, `weapon`, `mobility`, `utility`, `armor`, `power`)
  - `parent_slot_code TEXT NULL`
  - `slot_type TEXT`
  - `base_cost INT`, `base_weight INT`, `power_draw INT`, `power_supply INT`
  - `extra_json JSONB NULL`
  - plus display fields as in v1 (`game_image_url`, `menu_image_url`, …)

- `component_mount_point` (extended)
  - v1 fields plus `slot_code`, `accepts_tags`, `max_children`, `required`
  - UNIQUE (`host_component_id`, `slot_code`)

- `component_tag`, `component_requirement`, `component_incompatibility`, `component_stat_modifier`

#### Typed detail tables (full target)

- `turret_component_detail`, `weapon_component_detail`, `mobility_component_detail`

#### Player blueprint assembly (full target)

- `blueprint_component` extended with `mount_point_code`, `mount_index` snapshots
- `blueprint_rule_evaluation`

## Amendment: Mount Points (2026-07-12, simplified)

### Context

The v1 roadmap story ([TW-1](../../tickets/TW-1-allow-buying-turret-component.md): mount components on Tank/Truck blueprints) needs hosts to declare which child `kind` values they accept. Tank offers a turret slot; Truck offers a weapon slot only.

### Decision

Introduce `component_mount_point` with only:

- `host_component_id` → `component_definition.id`
- `accepts_kind` — must match the child's `component_definition.kind`

No `parent_slot_code` on children, no tags, no `max_children` column — slot fullness is enforced by uniqueness: at most one child per `(parent_blueprint_component_id, mount_point_id)`.

### Options considered (summary)

| Option | Decision |
|--------|----------|
| Implicit kind matching only (no mount point table) | Rejected — no data-driven empty-slot UI |
| Explicit mount points with `accepts_kind` only | **Accepted for v1** |
| Full slot model (`slot_code`, tags, `max_children`, child `parent_slot_code`) | Deferred — see [Deferred schema](#deferred-schema) |

### Assembly example (v1)

```
blueprint: "Tank #1"
└─ blueprint_component (chassis: Tank)           parent: NULL, mount_point_id: NULL
   └─ blueprint_component (turret: Scout)        parent: chassis row, mount_point_id: → Tank turret slot
      └─ blueprint_component (weapon: main gun)  parent: turret row, mount_point_id: → turret weapon slot

blueprint: "Truck #1"
└─ blueprint_component (chassis: Truck)          parent: NULL, mount_point_id: NULL
   └─ blueprint_component (weapon: Light MG)     parent: chassis row, mount_point_id: → Truck weapon slot
```

Rule **evaluation timing** remains in [ADR-002](ADR-002-rule-evaluation-strategy-draft.md).

## Why This Fits Tank Wars

- Matches v1 graph-based assembly and tag-based compatibility.
- Supports domain-specific stats like turret rotation speed without class explosion.
- Keeps balancing mostly in data while preserving SQL constraints and queryability.

## Consequences

Positive:
- Faster content expansion and balancing iterations.
- Better testability: validator and stat aggregation can be deterministic over table data.
- Easier UI explainability for invalid builds and trade-offs.

Negative:
- More migration and data-seeding work in early development.
- Requires disciplined rule validation service boundaries.

## Migration Notes from Current Schema

Current state contains fixed columns in `basic_tank` for speed, turret speed, and machine-gun fields.

Incremental migration path:
1. Introduce `component_definition` and `blueprint_component` tables.
2. Create seed components equivalent to existing `basic_tank` capabilities.
3. **Add `component_mount_point` and assembly fields on `blueprint_component` (`parent_blueprint_component_id`, `mount_point_id`) — simplified v1.**
4. Seed mount points: Tank `accepts_kind=turret`; Truck `accepts_kind=weapon`; Scout turret `accepts_kind=weapon`; seed turret and weapon `component_definition` rows.
5. Build a validator + stat aggregator service in backend.
6. Backfill existing blueprint data into component assemblies (chassis rows get `parent_blueprint_component_id = NULL`).
7. Deprecate and remove `basic_tank` after parity and verification.

## Validation and Testing Impact

- Add unit tests for mount validation (`accepts_kind` match, slot already occupied).
- Add integration tests for blueprint assembly persistence and stat aggregation.
- Add fixture-based balance tests for representative archetypes (raider, brawler, sniper).

## Open Questions

- Which stats should remain persisted as denormalized cache versus computed at runtime?
- Do we need versioned component definitions for live-balance patches?
- When to add `slot_code`, tags, and `max_children` to mount points (see [Deferred schema](#deferred-schema))?
