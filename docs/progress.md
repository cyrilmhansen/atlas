# Current progress

Updated: 2026-07-17

## Objective

Demonstrate union-find, run-length encoding and A* as useful local WASM
experiences, recording only Atlas limitations that materially block them.

## Demonstrated state

- Schema 0.2 is authoritative under DEC-075; all existing consumers are migrated.
- The registry contains 31 Problems, 39 Algorithms and 43 Implementations.
- Public schema 0.2 and the selection CLI remain unchanged.
- The Web Explorer derives sequence demonstrations from five generated programs.
- Union-find now has a bounded real WASM stepper and full-width component view.
- Catalog identity, costs and evidence handoff remain registry-derived.

## Active experiment

Run-length encoding with visible incremental input and encoded output.

## Principal recent result

Union-find exposes representative inspection before mutation, supports repeated
and redundant unions, and requires no public schema or visual-bytecode change.

## Open uncertainty

It is not yet known whether three unrelated experiences share enough real
structure to justify a private generated experience description.

## Next falsifiable action

Implement incremental RLE in WASM and verify that input consumption and output
emission remain inspectable without stored presentation traces.

## Blocking structural decisions

None.
