# DEC-038 - Close MVP 3 composition scope

## Status

Accepted on 2026-07-13 (`close-mvp3-A`).

## Decision

Close MVP 3 with five bounded, internal composition scenarios drawn from the
existing sequence corpus: cleanup, find, partition-sort, unique-sort, and
merge-sorted. Together they demonstrate linear pipelines, an explicit
declared-cost trade-off, produced preconditions, a structured intermediate,
and a two-input fan-in.

Every scenario renders inputs, outputs, mutations, copies, allocations, a
selected candidate, and a rejected compatible candidate. Its unconstrained
Rust orchestration is a committed Cargo example and is independently compiled
and run. Explicit force/forbid constraints remain limited to the two reviewed
candidates of each scenario.

MVP 3 does not add a public plan schema, persistent plan format, general
planner, general code generator, plugin interface, or MIR coupling. The
internal Rust composition model remains an experiment rather than registry
authority.

## Consequences

- Schema 0.1 and the Git-authoritative registry remain unchanged by composition
  planning.
- `atlas compose` remains a bounded demonstrator, not a stable planning API.
- Rendered Rust remains verification material only; Atlas does not compile or
  execute it at runtime.
- A future phase that persists plans, expands candidate search, or generates
  arbitrary source must define public semantics, versioning, evidence, and
  validation before activation.
- MVP 4 is not activated by this closure. Any MIR adapter requires its own
  accepted scope and reproducible ABI and memory-model experiments.
