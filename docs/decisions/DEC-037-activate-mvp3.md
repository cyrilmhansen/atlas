# DEC-037 - Activate MVP 3

## Status

Accepted on 2026-07-13 (`mvp3-A`).

## Decision

Activate MVP 3 as a narrow experimental phase for explainable constrained
selection and linear composition. The first slice is limited to one real
pipeline drawn from the existing sequence corpus, internal structural data
types, and a human-readable plan representation that is neither a public schema
nor a persistent interchange format.

The first slice must make inputs, outputs, preconditions, mutations,
allocations, copies, and rejected alternatives explicit. It must not add a
general planner, a public plan format, a plugin interface, or MIR coupling.

## Consequences

- MVP 3 can add narrowly scoped internal Rust modules and deterministic tests.
- The aggregate registry schema 0.1 remains unchanged until real scenarios
  justify public plan data.
- A later decision is required before serializing plans as a stable format or
  generating executable Rust orchestration outside an experiment.
- MVP 4, including any MIR adapter, remains out of scope.
