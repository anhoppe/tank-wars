# TW-1 — Allow buying turret component

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Implementation plan for mounting turret and light machine gun components onto existing player blueprints, and using them in combat. |
| **Incoming** | [Roadmap TW-1](../v1/roadmap.md), [ADR-001](../arc42/09_architecture_decisions/ADR-001-component-model-and-storage.md) (simplified mount-point model), [blueprints.md](../v1/blueprints.md) |
| **Outgoing** | DB migration, backend mount API, Blueprint Workshop UI, game rendering/controls; schema documented in [08_01](../arc42/08_crosscutting_concepts/08_01_database_layout.md) |

## Status

Planned

## Goal

Player can customize **Tank** and **Truck** blueprints differently:

- **Tank** — mount a **turret** (rotates toward mouse), then a **main gun** on the turret.
- **Truck** — mount a **light machine gun** directly on the chassis (no turret); MG fires only in the truck’s facing direction.

## Acceptance criteria

- [ ] Catalog contains Scout turret, Light MG, and Main Gun `component_definition` rows (plus existing chassis types).
- [ ] **Tank** chassis catalog offers `accepts_kind = turret`; **Truck** offers `accepts_kind = light_gun` (no turret).
- [ ] Scout turret catalog offers `accepts_kind = heavy_gun`.
- [ ] Player selects a blueprint in Research Blueprints, sees chassis-appropriate instance slots (Tank: turret → heavy_gun; Truck: light_gun only), and **buys & mounts** compatible components.
- [ ] Mounting validates: `child.kind = instance_slot.accepts_kind`, slot not already occupied, player has enough money.
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

Per [ADR-001](../arc42/09_architecture_decisions/ADR-001-component-model-and-storage.md) and [08_01 database layout](../arc42/08_crosscutting_concepts/08_01_database_layout.md): **catalog** mount points + **instance** mount points on each installed host.

### Catalog vs. instance

| Layer | Table | Role |
|-------|--------|------|
| Catalog | `component_mount_point` | Tank offers `turret` slot (global seed) |
| Instance | `blueprint_component_mount_point` | Copied onto installed Tank row on a blueprint |
| Child | `blueprint_component` | `mounted_on_slot_id` → parent's instance slot |

### Mount rule

```
child.kind == blueprint_component_mount_point.accepts_kind
AND slot is empty (no other child references that slot id)
```

### On install

| Source | Stored on |
|--------|-----------|
| Installed part | `blueprint_component.component_definition_id` + snapshots (`kind`, images) |
| Parent slot | `blueprint_component.mounted_on_slot_id` → `blueprint_component_mount_point.id` |
| Host's new slots | `blueprint_component_mount_point` rows copied from catalog when host is installed |

Root chassis: `mounted_on_slot_id = NULL`.

### Chassis archetypes (gameplay difference)

| Chassis | Mount points on chassis | Assembly depth | Combat behaviour |
|---------|-------------------------|----------------|----------------|
| **Tank** | `accepts_kind = turret` (catalog) | chassis → turret → heavy_gun | Turret rotates; main gun along turret |
| **Truck** | `accepts_kind = light_gun` (catalog) | chassis → light_gun | MG along hull only |

This is the main reason mount points are per-`component_definition`: Truck and Tank chassis definitions differ in which slots they offer.

### Target assembly trees

**Tank blueprint**

```text
bc1  blueprint_component (Tank)              mounted_on_slot_id: NULL
  bcmp1  instance slot (accepts turret)
    bc2  blueprint_component (Scout)        mounted_on_slot_id → bcmp1
      bcmp2  instance slot (accepts heavy_gun)
        bc3  blueprint_component (Main Gun) mounted_on_slot_id → bcmp2
```

**Truck blueprint**

```text
bc1  blueprint_component (Truck)           mounted_on_slot_id: NULL
  bcmp1  instance slot (accepts light_gun)
    bc2  blueprint_component (Light MG)    mounted_on_slot_id → bcmp1
```

### Seed data (v1)

| `component_definition` | `kind` | Mount points offered |
|------------------------|--------|----------------------|
| Tank | `chassis` | `turret` |
| Truck | `chassis` | `light_gun` |
| Scout turret | `turret` | `heavy_gun` |
| Light MG | `light_gun` | — |
| Main Gun | `heavy_gun` | — |

---

## Implementation plan

### Phase 1 — Database

1. Migration: `component_mount_point` (catalog; exists).
2. Migration: add `blueprint_component_mount_point` (`blueprint_component_id`, `accepts_kind`, `source_mount_point_id`).
3. Migration: `blueprint_component.mounted_on_slot_id` (replace `mount_point_id` / `parent_component_mount_point_id`).
4. On host install: copy catalog slots → instance slots on that `blueprint_component`.
5. Seed catalog: Tank→turret, Truck→light_gun, Scout→heavy_gun; seed Scout, Light MG, Main Gun definitions.

### Phase 2 — Backend

1. **`component_mount_point_db`** — read catalog slots by `component_definition_id`.
2. **`blueprint_component_mount_point_db`** — create instance slots on host install; list slots for a host.
3. **Mount validator**
   - Find empty `blueprint_component_mount_point` on parent host where `accepts_kind = child.kind`.
   - Reject if slot already has a child (`mounted_on_slot_id` in use).
   - Insert child; copy child's catalog slots to instance rows on child.
3. **New endpoint** — e.g. `POST /api/blueprints/{player_id}/components` body: `{ blueprintId, componentDefinitionId, parentInstanceSlotId }`.
4. **Extend GET blueprint** — return assembly: installed parts, instance slots per host, which slots are empty.
5. **Extend `get_vehicle_types`** or add **`GET /api/component-definitions?kind=turret|light_gun|heavy_gun`** for the component shop.
6. Update **`vehicle_dto` / `vehicle_on_map_dto`** — resolve display from assembly (chassis + turret at minimum) instead of chassis-only.

### Phase 3 — Frontend (Blueprint Workshop)

Evolve [ResearchBlueprints.js](../../src/frontend/tank-wars/src/Game/ResearchBlueprints.js):

1. Selecting a blueprint loads assembly with **instance slots** on each installed host.
2. Slot list from `blueprint_component_mount_point` on selected host (empty/filled).
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
  "parentInstanceSlotId": "uuid"
}
```

`parentInstanceSlotId` is the `blueprint_component_mount_point.id` on the host (empty slot).

**Success:** updated player (money), blueprint summary, new `blueprint_component` id.

**Errors (4xx):** `slot_occupied`, `kind_mismatch`, `insufficient_funds`, `parent_not_found`.

### `GET /api/blueprints/{player_id}/{blueprint_id}` (or extend list response)

Returns blueprint with `components[]` and per-host `instanceMountPoints[]` (empty slots have no child referencing them).

---

## UX sketch

**Tank blueprint selected**

```
┌─────────────────┬──────────────────────────┬─────────────────────┐
│ My blueprints   │ Assembly                 │ Shop (filtered)     │
│ > Tank #1       │ [chassis] Tank     ✓     │ Scout turret  120   │
│   Truck #1      │ [turret]  (empty)  ←sel │ [Buy & Mount]       │
│                 │ [heavy gun] (locked)   │                     │
└─────────────────┴──────────────────────────┴─────────────────────┘
```

Weapon slot on turret unlocks after turret is mounted (`accepts_kind = heavy_gun`).

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

- Mount turret on **Tank** → child `mounted_on_slot_id` points at chassis instance slot.
- Mount Light MG on **Truck** → valid.
- Reject turret on **Truck** (no instance slot with `accepts_kind = turret`).
- Reject second child on same instance slot.
- Money and `buying_price` updated correctly.

### Frontend

- **Tank:** empty turret instance slot, then heavy_gun slot on turret after mount.
- **Truck:** single machine-gun slot only; no turret row.
- After mount, slot shows installed part name/image.
- Buy vehicle from Fleet still works.

### Game

- **Tank:** turret sprite rotates with mouse; gun fires along turret bearing.
- **Truck:** MG fires along hull bearing; no turret rotation.
- Fire inputs produce visible feedback (damage waits for TW-2).

---

## Documentation

- Authoritative schema: [08_01_database_layout.md](../arc42/08_crosscutting_concepts/08_01_database_layout.md).
- Mark TW-1 done in [roadmap.md](../v1/roadmap.md) when acceptance criteria are met.

## Open decisions

- Asset keys for turret, main gun, and truck MG sprites (placeholders OK for first slice).
- Whether gameplay phase is part of the same PR or a follow-up PR after workshop + API land.
