# Current progress

Updated: 2026-07-17

## Objective

Demonstrate union-find, run-length encoding and A* as useful local WASM
experiences, recording only Atlas limitations that materially block them.

## Demonstrated state

- Schema 0.2 is authoritative under DEC-075; all existing consumers are migrated.
- The registry contains 33 Problems, 41 Algorithms and 45 Implementations.
- Public schema 0.2 and the selection CLI remain unchanged.
- The Web Explorer derives sequence demonstrations from five generated programs.
- Union-find, RLE and A* have bounded real WASM steppers and full-width domain views.
- Catalog identity, costs and evidence handoff remain registry-derived.

## Active experiment

None. The three-demonstration phase is complete.

## Principal recent result

A* exposes an editable grid, frontier, closed set and shortest path; its stepper
matches a Petgraph oracle and keeps local observations separate from claims.

## Open uncertainty

Explorer repeats private registration, playback and catalog wiring per domain;
the three state models and renderers are genuinely different.

## Next falsifiable action

If approved, factor a private experience shell while retaining explicit domain
steppers and renderers; do not stabilize a format.

## Blocking structural decisions

Starting that consolidation as a new phase requires validation.
