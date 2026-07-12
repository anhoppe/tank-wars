# TD-002 – Frontend TypeScript Migration

| Field | Value |
|-------|-------|
| **Purpose & Intent** | Track the need for incremental migration of the frontend from JavaScript to TypeScript. |
| **Incoming** | Frontend editor/game integration work, event wiring between React and Phaser, asset handling, and registry-based state exchange. |
| **Outgoing** | Incremental migration plan, shared frontend types, typed event payload contracts, and stricter validation of scene/registry interactions. |

## Context

The current frontend uses JavaScript across React and Phaser integration points. Recent work exposed failure modes around undefined values, event payload shape assumptions, scene lifecycle timing, and implicit contracts between registry keys and consumers.

## Technical Debt

- Event payloads are untyped and can drift between emitters and listeners without early feedback.
- Scene lifecycle access and registry reads rely on runtime assumptions instead of checked contracts.
- Asset identifiers and render data shapes are not enforced across module boundaries.
- Refactoring safety is limited in files that mix React state, Phaser state, and API payloads.

## Risk

If the frontend remains untyped while complexity grows, changes to editor/game integration, vehicle rendering, and asset composition will become harder to validate and more prone to runtime-only failures.

## Proposed Mitigation Direction

- Introduce TypeScript incrementally in the frontend with allowJs during transition.
- Start with shared constants, API payload types, and event payload contracts.
- Add typed wrappers/helpers for Phaser registry access and scene event payloads.
- Enable stricter compiler options after the initial migration stabilizes.

## Trigger For Action

Begin migration before expanding frontend composition logic further, especially before introducing multi-part vehicle rendering or additional scene-to-React coordination.
