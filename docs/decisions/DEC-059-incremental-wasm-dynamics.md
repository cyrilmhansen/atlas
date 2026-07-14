# DEC-059 - Incremental WASM dynamics

## Status

Accepted on 2026-07-14 (`stepper-WASM-A`).

## Decision

Use a stateful WebAssembly stepper for interactive algorithm presentation.
The browser asks the stepper to execute one semantic AST operation and reads
only its current sequence, element origins, counters and active node. It does
not receive or retain a precomputed execution trace.

Keep bounded traces as validation and analysis instruments. For insertion sort,
tests compare every incremental operation with the analytical trace and compare
the final stable result and counters with the native generic implementation.
Aggregate Scale execution continues to return only results and counters.

Backward navigation resets the WASM state and deterministically re-executes to
the requested step. The Explore bound of 64 insertion-sort elements keeps that
deliberately simple strategy bounded without storing snapshots. Its independent
analytical trace remains limited to 32 elements.

## Consequences

- Interactive memory is proportional to the current working sequence, not to
  the number of semantic operations.
- The stepper is a separate pauseable implementation of the accepted insertion
  semantics because a generic Rust function cannot yield its call frame after
  each operation. Differential tests are therefore mandatory.
- The WASM class and Web projection remain private MVP interfaces, not a stable
  runtime protocol or public trace schema.
- The existing `is_sorted` trace player was subsequently migrated to the same
  execution model. Both traces are now restricted to tests and analysis.
