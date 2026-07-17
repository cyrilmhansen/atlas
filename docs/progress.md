# Current progress

Updated: 2026-07-17

## Objective

Demonstrate union-find, run-length encoding and A* as useful local WASM
experiences, recording only Atlas limitations that materially block them.

## Demonstrated state

- Schema 0.2 is authoritative under DEC-075; all existing consumers are migrated.
- The registry contains 32 Problems, 40 Algorithms and 44 Implementations.
- Public schema 0.2 and the selection CLI remain unchanged.
- The Web Explorer derives sequence demonstrations from five generated programs.
- Union-find and RLE have bounded real WASM steppers and full-width domain views.
- Catalog identity, costs and evidence handoff remain registry-derived.

## Active experiment

A* frontier and path evolution on a small editable grid.

## Principal recent result

RLE exposes input consumption, its pending run and emitted output without a
stored trace or public schema/visual-bytecode change.

## Open uncertainty

It is not yet known whether three unrelated experiences share enough real
structure to justify a private generated experience description.

## Next falsifiable action

Implement incremental A* in WASM and verify frontier/path state against a direct
oracle on deterministic grids.

## Blocking structural decisions

None.
