# ADR-003 - Application Layer vs Fat Handlers (Pragmatic Hexagon)

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Decide how HTTP handlers relate to business use cases as blueprint buy/mount flows grow beyond simple CRUD. |
| **Incoming** | Ch. 2 Rust/Axum backend, ADR-001 assembly model, TW-1 buy/mount flows in `handler.rs`. |
| **Outgoing** | Ch. 5 building blocks (API vs application services), future refactors of buy/mount endpoints, optional follow-up on transactions. |

## Status

Accepted

## Date

2026-07-19

## Context

The Axum handlers (especially `buy_blueprint_for_player` and related blueprint routes) already mix:

- HTTP concerns (path/body parsing, status codes, JSON errors)
- Business orchestration (money checks, create blueprint, install chassis, copy catalog mount points to instance slots)
- Persistence calls (`*_db` modules)

As TW-1 grows (mount child components, validation, transactions), fat handlers become hard to overview, test, and change. A full hexagonal (“ports & adapters”) layout is attractive but easy to over-engineer for a hobby learning project.

## Decision

Adopt a **pragmatic hexagon**: extract **application use cases** (services) for multi-step business flows; keep handlers as a thin HTTP adapter. Do **not** introduce a full ports/adapters package tree or trait-per-repository layer until a second adapter (e.g. tests with fakes, CLI, second transport) actually needs it.

### Target shape

```text
HTTP (Axum handler)     →  parse request, map errors to status codes
        │
        ▼
Application use case    →  buy chassis blueprint, mount component, …
        │
        ▼
Persistence (*_db)      →  sqlx queries (adapters in practice)
```

Dependencies point **inward** toward the use case: handlers depend on services; services depend on DB modules (or later on traits). Domain rules (mount kind matching, insufficient funds) live in or next to the use case, not in route wiring.

### Concrete conventions (v1)

| Layer | Responsibility | Example |
|-------|----------------|---------|
| Handler | Path/body → typed inputs; call use case; map `UseCaseError` → `(StatusCode, Json)` | `buy_blueprint_for_player` |
| Application service | Orchestrate one business action; own transaction boundary when introduced | `buy_chassis_blueprint(player_id, component_definition_id)` |
| `*_db` / repos | Single-table or narrow persistence operations | `create_blueprint_component`, `create_blueprint_component_mount_point` |

### Errors

Prefer a small use-case error enum (e.g. `InsufficientFunds`, `NotFound`, `Persistence`) that the handler maps to HTTP. Avoid leaking `sqlx::Error` as the handler’s primary API.

### Transactions

Multi-step writes that must be atomic (deduct money + create blueprint + chassis + instance slots) belong in the **use case**, via a DB transaction (`BEGIN`/`COMMIT` through sqlx), not as one giant SQL statement and not as “hope the handler never fails mid-way.” Timing: introduce when partial-failure risk matters; the boundary is already the use case.

### What we explicitly defer

- Trait ports for every repository
- Separate crates for domain vs infrastructure
- DDD aggregate purism
- Rewriting all existing handlers in one go

Migrate **use case by use case** (start with buy blueprint / mount component) as those endpoints change.

## Options Considered

### Option A - Keep business logic in handlers

Pros: few files; fast for tiny CRUD.  
Cons: dependency magnet; hard to test; duplicates across routes.  
**Rejected** as the long-term default for buy/mount flows.

### Option B - Full hexagonal / clean architecture upfront

Pros: strong boundaries; easy to swap DB or add test doubles.  
Cons: high ceremony for current size; slows learning delivery of TW-1.  
**Rejected** for now; revisit if a second adapter or heavy testing needs ports.

### Option C - Thin handlers + application services + sqlx modules (pragmatic)

Pros: clears the handler; keeps sqlx modules as practical adapters; matches current code shape; room to grow into real ports later.  
**Accepted.**

## Consequences

Positive:

- Buy/mount flows become readable “scripts” of domain steps.
- Handlers stay overviewable.
- Natural place for transactions and domain errors.
- Path open to real ports later without a big-bang rewrite.

Negative:

- One more layer to navigate for new contributors.
- Temporary inconsistency while only some flows are extracted.
- Risk of anemic “service” wrappers if people move code without moving *decisions*.

## Migration notes

1. Extract `buy_chassis_blueprint` (and later `mount_component`) from `handler.rs` into an application module (e.g. `blueprint_service.rs` or `app/blueprint.rs`).
2. Handler only parses `componentDefinitionId` / authz-style checks and maps errors.
3. When adding atomicity, open a sqlx transaction inside the use case and pass it into DB helpers (or accept `&mut Transaction` in those helpers).
4. Leave simple read endpoints in handlers until they grow orchestration.

## Related

- [ADR-001](ADR-001-component-model-and-storage.md) — what is being bought/mounted  
- [08_01_database_layout.md](../08_crosscutting_concepts/08_01_database_layout.md) — install/copy mount-point flow  
- [TW-1](../../tickets/TW-1-allow-buying-turret-component.md) — first flows to extract  
