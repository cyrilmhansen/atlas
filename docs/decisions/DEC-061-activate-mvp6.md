# DEC-061 - Activate MVP 6 for generic executable presentation

## Status

Accepted on 2026-07-14 (`mvp6-A`).

## Decision

Activate MVP 6 to replace the current per-algorithm Web execution and
presentation paths with one bounded generated execution path. The MVP covers:

- a viewport-width execution visualization with mobile-first state priority;
- build-time lowering from the private typed algorithm AST to a private compact
  program under DEC-062;
- one bounded WASM visual machine executing those programs;
- a private derived presentation description under DEC-063;
- differential migration of adjacent `is_sorted`, stable insertion and
  symmetric reverse;
- `sequence.minimum` and even partition as independent generality tests.

## Exit criteria

- Five sequence algorithms execute through the same visual machine and match
  their native Rust correction fixtures.
- Exact current operations retain their AST node identifiers.
- Adding one supported algorithm requires no new WASM class, export or
  algorithm branch in `web/app.js`.
- Algorithm-specific presentation data remains declarative and bounded to less
  than 50 non-test lines per algorithm.
- The execution visualization uses the available viewport width and remains
  operable at the accepted mobile, tablet and desktop viewports.
- The clean static-bundle reproducibility gate remains green.

## Excluded scope

MVP 6 does not stabilize bytecode, AST, presentation or WASM interfaces. It
does not introduce arbitrary calls, recursion, allocation, objects, multiple
memory regions, concurrency, plugins, MIR-in-browser execution, publication or
a general algorithm runtime. Performance-fingerprint implementation remains a
subsequent scope informed by `docs/performance-model-research.md`.

## Consequences

- The existing hand-written steppers remain differential references until each
  generated path is accepted; removing them requires an explicit checkpoint.
- Native Rust remains the correction authority.
- Any widening of the visual machine memory or call model is a class C
  decision.
