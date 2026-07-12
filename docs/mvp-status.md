# MVP status

- Active MVP: **MVP 2 - empirical qualification**
- Status: experimental first slice
- MVP 1: closed locally at baseline `8a2a520`
- MVP 2 activated: 2026-07-12

## Current slice

Prototype separate deterministic dataset specifications for `sequence.sort`
and `sequence.partition` without changing schema 0.1.

Current corpus progress:

- Problems: 10 / 10
- Algorithms: 15 / 15
- Implementations: 20 / 20

## MVP 1 closure

- 10 problems
- 15 algorithms
- 20 tested Rust implementations
- Git-authoritative manifests with a rebuildable SQLite projection
- `list`, `show`, `validate`, `search`, and `explain` commands

Implemented commands: `validate`, `list`, `show`, `search`, `explain`.

Runtime boundary: algorithm implementations live in the `std`-independent
`atlas-algorithms` crate. Its core subset has no dependencies; its default
features enable `alloc` and optional hash-based deduplication. Registry and CLI
remain in the `std`-based `atlas` crate.

## Exit criteria status

| Criterion | Status |
|---|---|
| 10 problems, 15 algorithms, 20 tested implementations | Complete |
| `list`, `show`, `validate`, `search`, `explain` | Complete |
| Add registry components without validator code changes | Demonstrated |
| Qualified claims with provenance | Complete for MVP 1 local schemes |
| Mandatory-property coverage at 90% or more | Complete: required fields enforced |
| Validate semantic types and evidence references | Satisfied for MVP 1 local evidence schemes |
| Git-authoritative committed source | Complete: local MVP 1 baseline established |
| Deterministic rebuild and SQLite projection | Complete |
| Schema versioning and compatibility rules | Complete before freeze |
| Single local acceptance command | Complete: `scripts/check-mvp1.sh` |

See `docs/mvp1-review.md` for evidence and recommended closure order.
