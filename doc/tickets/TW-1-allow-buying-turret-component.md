# TW-1 — Allow buying turret component

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Implementation plan for mounting turret and light machine gun components onto existing player blueprints, and using them in combat. |
| **Incoming** | [Roadmap TW-1](../v1/roadmap.md), [ADR-001](../arc42/09_architecture_decisions/ADR-001-component-model-and-storage.md) (simplified mount-point model), [blueprints.md](../v1/blueprints.md) |
| **Outgoing** | DB migration, backend mount API, Blueprint Workshop UI, game rendering/controls; follow-up updates to `08_01_database_layout.md` |

## Status

Planned

## Goal

Player can customize **Tank** and **Truck** blueprints differently:

- **Tank** — mount a **turret** (rotates toward mouse), then a **main gun** on the turret.
- **Truck** — mount a **light machine gun** directly on the chassis (no turret); MG fires only in the truck’s facing direction.

## Acceptance criteria

- [ ] Catalog contains turret and weapon `component_definition` rows (plus existing chassis types).
- [ ] **Tank** chassis offers `accepts_kind = turret`; **Truck** chassis offers `accepts_kind = weapon` (no turret slot).
- [ ] Turret definition offers `accepts_kind = weapon` (main gun on turret).
- [ ] Player selects a blueprint in Research Blueprints, sees chassis-appropriate slots (Tank: turret → weapon; Truck: weapon only), and **buys & mounts** compatible components.
- [ ] Mounting validates: `child.kind = mount_point.accepts_kind`, slot not already occupied, player has enough money.
- [ ] Blueprint `buying_price` increases when components are mounted.
- [ ] Fleet can still buy a vehicle from the updated blueprint.
- [ ] In `GameScene`, **Tank** shows chassis + rotatable turret + weapon; **Truck** shows chassis + fixed MG.
- [ ] **Tank:** turret rotates toward mouse; main gun fires along turret bearing.
- [ ] **Truck:** MG fires along chassis bearing only (no independent turret rotation).
- [ ] Minimal combat feedback for firing (damage/destruction is [TW-2](../v1/roadmap.md)).

## Out of scope (this ticket)

- Hit points, damage, explosions ([TW-2](../v1/roadmap.md)).
- Enemy AI / skills ([TW-3](../v1/roadmap.md)).
- Tags, requirements, incompatibilities, stat detail tables (ADR-001 deferred schema).
- Remove/swap mounted components.
- Full asset composition pipeline ([TD-001](../arc42/11_risks_and_technical_debt/TD-001-vehicle-asset-composition.md)) — apply minimal layering only.

---

## Architecture (agreed)

Per [ADR-001 amendment](../arc42/09_architecture_decisions/ADR-001-component-model-and-storage.md): **four tables**, one mount rule.

### Mount rule

```
child.component_definition.kind == component_mount_point.accepts_kind
```

### `blueprint_component` on install

| Source | Stored on `blueprint_component` |
|--------|----------------------------------|
| Installed part (`component_definition`) | `component_definition_id` + snapshot copy: `kind`, `game_image_url`, `menu_image_url` |
| Slot used (`component_mount_point`) | `mount_point_id` (FK only) |
| Assembly tree | `parent_blueprint_component_id` |

Root chassis row: `parent_blueprint_component_id` and `mount_point_id` are `NULL`.

### Chassis archetypes (gameplay difference)

| Chassis | Mount points on chassis | Assembly depth | Combat behaviour |
|---------|-------------------------|----------------|----------------|
| **Tank** | `accepts_kind = turret` | chassis → turret → weapon | Turret rotates independently; main gun fires along turret angle |
| **Truck** | `accepts_kind = weapon` | chassis → weapon (MG) | No turret; MG fires along truck facing only |

This is the main reason mount points are per-`component_definition`: Truck and Tank chassis definitions differ in which slots they offer.

### Target assembly trees

**Tank blueprint**

```text
blueprint "Tank #1"
└─ blueprint_component (chassis: Tank)
   └─ blueprint_component (turret: Scout)       mount_point_id → Tank.turret slot
      └─ blueprint_component (weapon: Main gun) mount_point_id → Scout.weapon slot
```

**Truck blueprint**

```text
blueprint "Truck #1"
└─ blueprint_component (chassis: Truck)
   └─ blueprint_component (weapon: Light MG)    mount_point_id → Truck.weapon slot
```

### Seed data (v1)

| `component_definition` | `kind` | Mount points offered |
|------------------------|--------|----------------------|
| Tank | `chassis` | `accepts_kind = turret` |
| Truck | `chassis` | `accepts_kind = weapon` |
| Scout turret | `turret` | `accepts_kind = weapon` |
| Light MG | `weapon` | — (used on Truck chassis or Tank turret) |

---

## Implementation plan

### Phase 1 — Database

1. Migration: create `component_mount_point` (`id`, `host_component_id`, `accepts_kind`, unique on `(host_component_id, accepts_kind)`).
2. Migration: extend `blueprint_component` with `parent_blueprint_component_id`, `mount_point_id` (nullable FKs).
3. Backfill existing chassis rows: `parent_blueprint_component_id = NULL`, `mount_point_id = NULL`.
4. Seed mount points: Tank `accepts_kind=turret`; Truck `accepts_kind=weapon`; Scout turret `accepts_kind=weapon`.
5. Seed `component_definition` rows for Scout turret, Light MG, and (if needed) a distinct main-gun weapon for the Tank turret.

### Phase 2 — Backend

1. **`component_mount_point_db`** — CRUD/read by `host_component_id`.
2. **Mount validator** (inline in handler or small module):
   - Resolve parent `blueprint_component` for the target blueprint.
   - Load parent's `component_definition` and its mount points.
   - Match `accepts_kind` to child `kind`.
   - Reject if `(parent_blueprint_component_id, mount_point_id)` already exists.
   - Deduct `component_definition.price` from player; add to `blueprint.buying_price`.
3. **New endpoint** — e.g. `POST /api/blueprints/{player_id}/components` body: `{ blueprintId, componentDefinitionId, parentBlueprintComponentId }`.
4. **Extend GET blueprint** (or new DTO) — return assembly tree: installed components, empty mount points per parent, prices.
5. **Extend `get_vehicle_types`** or add **`GET /api/component-definitions?kind=turret|weapon`** for the component shop.
6. Update **`vehicle_dto` / `vehicle_on_map_dto`** — resolve display from assembly (chassis + turret at minimum) instead of chassis-only.

### Phase 3 — Frontend (Blueprint Workshop)

Evolve [ResearchBlueprints.js](../../src/frontend/tank-wars/src/Game/ResearchBlueprints.js):

1. Selecting a blueprint loads its assembly (slots depend on chassis type).
2. Show slot list driven by mount points on the blueprint’s chassis:
   - **Tank:** Chassis ✓ → Turret (empty/filled) → Weapon (empty/filled, on turret)
   - **Truck:** Chassis ✓ → Machine gun (empty/filled) — no turret row
3. Clicking an empty slot filters the shop to components whose `kind` matches the slot's `accepts_kind`.
4. **Buy & Mount** calls the new API; refresh blueprint price and slot state.
5. Disable incompatible or unaffordable options with a short reason.

### Phase 4 — Gameplay (minimal)

In [GameScene.js](../../src/frontend/tank-wars/src/Game/GameScene.js):

1. Load player vehicle assembly from API (chassis type + mounted parts).
2. **Tank:** layered sprites — chassis + turret child; turret rotation follows mouse; weapon fires along turret angle.
3. **Truck:** chassis + MG sprite fixed to hull; MG fires along `playerBase.angle` (no turret rotation).
4. Fire input per weapon (projectile stub OK until TW-2).

Enemy vehicles on opponent map: render from assembly when available; chassis-only fallback acceptable initially.

---

## API sketch

### `POST /api/blueprints/{player_id}/components`

```json
{
  "blueprintId": "uuid",
  "componentDefinitionId": "uuid",
  "parentBlueprintComponentId": "uuid"
}
```

**Success:** updated player (money), blueprint summary, new `blueprint_component` id.

**Errors (4xx):** `slot_occupied`, `kind_mismatch`, `insufficient_funds`, `parent_not_found`.

### `GET /api/blueprints/{player_id}/{blueprint_id}` (or extend list response)

Returns blueprint with nested `components[]` and per-parent `availableMountPoints[]` / `occupiedMountPointIds[]`.

---

## UX sketch

**Tank blueprint selected**

```
┌─────────────────┬──────────────────────────┬─────────────────────┐
│ My blueprints   │ Assembly                 │ Shop (filtered)     │
│ > Tank #1       │ [chassis] Tank     ✓     │ Scout turret  120   │
│   Truck #1      │ [turret]  (empty)  ←sel │ [Buy & Mount]       │
│                 │ [weapon]  (locked)       │                     │
└─────────────────┴──────────────────────────┴─────────────────────┘
```

Weapon slot on turret unlocks after turret is mounted.

**Truck blueprint selected**

```
┌─────────────────┬──────────────────────────┬─────────────────────┐
│ My blueprints   │ Assembly                 │ Shop (filtered)     │
│   Tank #1       │ [chassis] Truck    ✓     │ Light MG       40   │
│ > Truck #1      │ [machine gun] (empty)←sel│ [Buy & Mount]       │
└─────────────────┴──────────────────────────┴─────────────────────┘
```

No turret slot on Truck.

---

## Test plan

### Backend

- Mount turret on **Tank** blueprint → valid; correct `mount_point_id` and parent FK.
- Mount Light MG on **Truck** chassis → valid (direct child of chassis).
- Reject turret on **Truck** blueprint (no `accepts_kind = turret` on Truck chassis).
- Reject weapon on **Tank** chassis without turret parent (weapon must mount on turret’s weapon slot).
- Reject second component in the same occupied slot.
- Money and `buying_price` updated correctly.

### Frontend

- **Tank:** empty turret slot, then weapon slot on turret after mount.
- **Truck:** single machine-gun slot only; no turret row.
- After mount, slot shows installed part name/image.
- Buy vehicle from Fleet still works.

### Game

- **Tank:** turret sprite rotates with mouse; gun fires along turret bearing.
- **Truck:** MG fires along hull bearing; no turret rotation.
- Fire inputs produce visible feedback (damage waits for TW-2).

---

## Documentation follow-up

- Update [08_01_database_layout.md](../arc42/08_crosscutting_concepts/08_01_database_layout.md) ER diagram with `component_mount_point` and new `blueprint_component` columns.
- Mark TW-1 done in [roadmap.md](../v1/roadmap.md) when acceptance criteria are met.

## Open decisions

- Asset keys for turret, main gun, and truck MG sprites (placeholders OK for first slice).
- Separate `component_definition` for Tank main gun vs. Truck Light MG, or one shared `weapon` row (mount point still differs by chassis).
- Whether gameplay phase is part of the same PR or a follow-up PR after workshop + API land.
