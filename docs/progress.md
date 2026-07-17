# Current progress

Updated: 2026-07-17

## Objective

Determine whether Atlas can discover and conservatively adjudicate foreign
algorithm implementations across unrelated families without candidate-specific
selection code or a premature public schema change.

## Demonstrated state

- Phase 8 has a bounded qualified-time schema design awaiting adoption choice.
- The registry contains 31 Problems, 39 Algorithms and 43 Implementations.
- Graph reachability, priority-queue push and exact bounded top-k each have two
  candidates discovered through `solves` and `implements` alone.
- The unchanged private evaluator handles guarantees, forbidden effects,
  conditioned costs, retained memory and concrete state across all families.
- Missing or incomparable knowledge produces an unsupported result rather than
  an invented selection.

## Active experiment

Phase 8 design is complete; no schema change has been made.

## Principal recent result

Qualified time profiles plus first-class condition identities express heap,
hash-map and unconditional sorting costs without an expression language. A
generic resource-cost model is not justified by the current experiment.

## Open uncertainty

Condition vocabulary may still hide ambiguous semantics. The deterministic
schema 0.2 migration and downstream projection impact remain unexecuted.

## Next falsifiable action

If Option A is accepted, verify its five discriminants in a disposable fixture
before any authoritative manifest migration.

## Blocking structural decisions

Adopting qualified time profiles as schema 0.2 requires human validation.
