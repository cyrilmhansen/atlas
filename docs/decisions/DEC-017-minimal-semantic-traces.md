# DEC-017 - Minimal semantic traces

- Status: accepted, implementation deferred
- Class: C
- Date: 2026-07-11

## Decision

Use a small common vocabulary of semantic events for demonstrations, initially
including compare, read, write, swap, partition, recurse, allocate, copy, and
assert. Allow explicitly named algorithm-specific events without pretending to
define a universal low-level trace format.

## Consequences

Traces will reference an algorithm, implementation, dataset instance, and
execution context. They demonstrate dynamics and invariant checkpoints; they do
not constitute benchmark evidence. Stabilization waits for traces from at least
two materially different algorithms.
