# ADR-001 - Component Model and Storage for Combat Vehicles

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Decide how combat vehicle customization is represented in code and persisted in the database, balancing flexibility, validation, and balancing velocity. |
| **Incoming** | Ch. 1 gameplay flexibility goals, Ch. 2 Rust/PostgreSQL constraints, v1 tech-tree documents in doc/v1. |
| **Outgoing** | Ch. 5 building blocks (blueprint and rule engine), Ch. 6 runtime validation/stats pipeline, Ch. 8 data modeling concept. |

## Status

Accepted (Opt C)

## Date

2026-06-27

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

### Component Catalog (researchable parts)

- `component_definition`
  - `id UUID PK`
  - `code TEXT UNIQUE`
  - `name TEXT`
  - `kind TEXT` (`chassis`, `mount`, `turret`, `weapon`, `mobility`, `utility`, `armor`, `power`)
  - `slot_type TEXT`
  - `base_cost INT`
  - `base_weight INT`
  - `power_draw INT`
  - `power_supply INT`
  - `extra_json JSONB NULL`

- `component_tag`
  - `component_id UUID FK -> component_definition.id`
  - `tag TEXT`
  - PK (`component_id`, `tag`)

- `component_requirement`
  - `id UUID PK`
  - `component_id UUID FK`
  - `requirement_type TEXT` (`component`, `tag`)
  - `required_component_id UUID NULL FK`
  - `required_tag TEXT NULL`

- `component_incompatibility`
  - `id UUID PK`
  - `component_id UUID FK`
  - `incompatibility_type TEXT` (`component`, `tag`)
  - `blocked_component_id UUID NULL FK`
  - `blocked_tag TEXT NULL`

- `component_stat_modifier`
  - `id UUID PK`
  - `component_id UUID FK`
  - `stat_key TEXT`
  - `modifier_type TEXT` (`flat`, `percent`)
  - `value NUMERIC`

### Typed Detail Tables (dedicated properties)

- `turret_component_detail`
  - `component_id UUID PK FK -> component_definition.id`
  - `rotation_speed_deg_s NUMERIC`
  - `stabilization_bonus NUMERIC`

- `weapon_component_detail`
  - `component_id UUID PK FK`
  - `reload_s NUMERIC`
  - `penetration INT`
  - `alpha_damage INT`
  - `dispersion NUMERIC`

- `mobility_component_detail`
  - `component_id UUID PK FK`
  - `max_speed_bonus NUMERIC`
  - `turn_rate_bonus NUMERIC`
  - `terrain_penalty_factor NUMERIC`

### Player Blueprint Assembly

- `blueprint`
  - keep existing id/name/player link
  - add computed summary fields as needed (`buying_price`, `total_weight`, `power_balance`)

- `blueprint_component`
  - `id UUID PK`
  - `blueprint_id UUID FK -> blueprint.id`
  - `component_id UUID FK -> component_definition.id`
  - `parent_blueprint_component_id UUID NULL FK -> blueprint_component.id`
  - `mount_index INT NULL` (for multiple same slot positions)

### Rule Evaluation (optional explicit rule ids)

- `blueprint_rule_evaluation`
  - `id UUID PK`
  - `blueprint_id UUID FK`
  - `rule_code TEXT`
  - `rule_type TEXT` (`hard`, `soft`, `negative`)
  - `status TEXT` (`pass`, `fail`, `penalty`, `bonus`)
  - `value NUMERIC NULL`

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
3. Build a validator + stat aggregator service in backend.
4. Backfill existing blueprint data into component assemblies.
5. Deprecate and remove `basic_tank` after parity and verification.

## Validation and Testing Impact

- Add unit tests for requirement/incompatibility evaluation.
- Add integration tests for blueprint assembly persistence and stat aggregation.
- Add fixture-based balance tests for representative archetypes (raider, brawler, sniper).

## Open Questions

- Which stats should remain persisted as denormalized cache versus computed at runtime?
- Do we need versioned component definitions for live-balance patches?
- Should soft and negative rules always be precomputed or partially runtime-evaluated?
