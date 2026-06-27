# ADR-002 - Rule Evaluation Strategy for Blueprint Validation and Balance (Draft)

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Define the goal and scope for how blueprint rules are evaluated (hard, soft, negative) across validation, stat aggregation, and gameplay feedback. |
| **Incoming** | ADR-001 component model decision, Ch. 1 gameplay clarity and balancing goals, v1 compatibility/rule graphs. |
| **Outgoing** | Ch. 5 validator and stat pipeline building blocks, Ch. 6 runtime build/resolve flow, Ch. 8 cross-cutting rule evaluation concept. |

## Status

Draft

## Date

2026-06-27

## Goal

Establish a consistent rule evaluation strategy so that:
- invalid builds are rejected deterministically,
- valid builds receive predictable bonuses and penalties,
- UI and logs can explain exactly why a rule passed, failed, or applied a trade-off,
- balancing changes can be made through data with minimal backend rewrites.

## Context

The blueprint system uses composable components and rule relations (requirements, incompatibilities, synergies, negative interactions). We now need a clear strategy for when and where rules are evaluated:
- at assembly time (save/build validation),
- at runtime (combat state and dynamic modifiers),
- or as precomputed cached outputs.

Without this decision, behavior can drift between backend validation, gameplay calculations, and frontend explanation.

## Scope

In scope:
- Evaluation lifecycle for hard, soft, and negative rule types.
- Responsibility boundaries between persistence layer, validator service, and runtime systems.
- Explainability format for rule outcomes (`pass`, `fail`, `bonus`, `penalty`).
- Caching/precompute boundaries for rule results.

Out of scope:
- Exact numeric balancing values for individual components.
- UI layout details for presenting rule outcomes.
- Full migration plan for all existing gameplay systems.

## Decision Drivers

- Determinism and fairness in build validation.
- Runtime performance under frequent blueprint checks.
- Maintainability of rules as content volume grows.
- Explainability for players and for debugging.
- Ease of automated testing.

## Non-Goals

- Finalizing all optimization details in this ADR.
- Replacing ADR-001 data model decisions.
- Defining every possible rule category beyond hard/soft/negative.

## Constraints and Assumptions

- Backend stack remains Rust + PostgreSQL.
- Component and rule data is primarily data-driven per ADR-001.
- Rule outcomes may need persistence for analytics and debugging.

## Deferred Section

Options and final recommendation are intentionally deferred and will be added in a follow-up iteration of this ADR.

## Open Questions

- Which rule types must always be evaluated synchronously before save?
- Which rule outcomes can be cached and for how long?
- How should runtime-only context (for example temporary buffs) interact with precomputed outcomes?
- Should rule evaluation produce a stable, versioned explanation payload for frontend rendering?
