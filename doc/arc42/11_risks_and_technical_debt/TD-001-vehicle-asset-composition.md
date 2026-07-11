# TD-001 – Vehicle Asset Composition Pipeline

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Track technical debt around frontend asset handling for composed vehicles (base + turret + future components). |
| **Incoming** | Chapter 5 (Building Blocks), gameplay/editor rendering implementation in frontend scene logic. |
| **Outgoing** | Rendering component model draft, asset key registry, validation checks for missing textures, and follow-up ADR if composition strategy impacts architecture. |

## Context

Current rendering paths still rely on direct image references and fallback behavior. This works for single-sprite vehicles but will become brittle when vehicles are assembled from multiple visual components.

## Technical Debt

- Asset identity is not yet standardized across backend image paths, frontend texture keys, and Phaser preload keys.
- Multi-part rendering order (base, turret, barrel, effects) is not defined as a formal layering contract.
- Missing-asset behavior is currently ad-hoc and can hide configuration or content pipeline errors.
- Scene logic is at risk of accumulating composition-specific rules that should live in a dedicated rendering adapter.

## Risk

If left unresolved, adding component-based vehicles can cause visual inconsistencies, hard-to-debug fallback rendering, and tight coupling between gameplay data and scene-specific asset wiring.

## Proposed Mitigation Direction

- Introduce a canonical asset key registry that maps backend-provided identifiers to preload keys.
- Define a stable per-component draw order and transform contract.
- Add runtime validation for required component textures when spawning/rendering vehicles.
- Move vehicle composition into a dedicated renderer module instead of embedding it in scene event handlers.

## Trigger For Action

Start implementation before introducing the first vehicle with more than one independently rendered component.
