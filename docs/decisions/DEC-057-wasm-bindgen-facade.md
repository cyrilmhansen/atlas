# DEC-057 - Private wasm-bindgen facade for MVP 5

## Status

Accepted on 2026-07-14 (`wasm-bridge-A`).

## Decision

Use `wasm-bindgen` 0.2.100 for the private MVP 5 boundary between plain browser
JavaScript and a curated Rust/WebAssembly crate. Keep the crate and CLI versions
exactly aligned. Generate browser and Node.js bindings during the build; do not
commit generated glue or WebAssembly binaries.

The first exported operation accepts a copied `Int32Array` and returns a typed
`IsSortedObservation` containing the result, exact adjacent-comparison count
and optional first-inversion index. Inputs are limited to 4096 elements.

## Consequences

- Atlas relies on the binding generator for memory transfer and object lifetime
  instead of defining a raw pointer/length ABI.
- The facade is deliberately private and may change without compatibility. It
  is not the general algorithm ABI excluded by DEC-056.
- `atlas-algorithms` remains independent of WebAssembly and `wasm-bindgen`.
- CI must build the actual `wasm32-unknown-unknown` module and exercise the
  generated JavaScript binding, not only compile the Rust wrapper natively.
- Replacing `wasm-bindgen` remains possible while the facade is private, but
  requires equivalent correction and resource-limit tests.
