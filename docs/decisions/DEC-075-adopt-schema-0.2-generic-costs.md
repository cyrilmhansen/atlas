# DEC-075 - Adopt schema 0.2 generic costs

Status: accepted and implemented 2026-07-17

## Context

Phases 6 and 7 showed that schema 0.1 could not associate time bounds with
capacity and workload conditions across heap and hash-map families. Phase 8
validated generic time, memory and allocation profiles. Phase 9 proved a typed,
deterministic whole-registry migration and negative validation cases.

## Decision

Adopt schema 0.2 with first-class condition identities and generic qualified
Algorithm cost profiles. Migrate the authoritative aggregate registry, CLI,
SQLite projection and private Web projection together. Keep bounds opaque and
condition matching exact; do not add an expression language or complexity
ordering.

## Consequences

- `time_worst`, `time_expected` and `auxiliary_memory` are replaced by `costs`.
- SQLite projection version 2 and Web projection `atlas-web-private-v1` are
  incompatible derived formats and must be rebuilt.
- The private decision overlay reuses registry cost metric and regime types;
  Atlas Knowledge does not depend on that evaluator.
- Schema 0.1 remains documented as historical. Git commit `8106f55` retains the
  full-corpus dry-run migrator and its conservation tests.
- Condition truth remains caller-supplied and bounds remain exact strings.
