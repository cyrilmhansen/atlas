# Current progress

Updated: 2026-07-17

## Objective

Determine whether Atlas can discover and conservatively adjudicate foreign
algorithm implementations across unrelated families without candidate-specific
selection code or a premature public schema change.

## Demonstrated state

- Schema 0.2 is authoritative under DEC-075; all existing consumers are migrated.
- The registry contains 31 Problems, 39 Algorithms and 43 Implementations.
- Graph reachability, priority-queue push and exact bounded top-k each have two
  candidates discovered through `solves` and `implements` alone.
- The unchanged private evaluator handles guarantees, forbidden effects,
  conditioned costs, retained memory and concrete state across all families.
- Missing or incomparable knowledge produces an unsupported result rather than
  an invented selection.

## Active experiment

None. The schema 0.2 adoption slice is complete.

## Principal recent result

Heap and hash-map conditioned requests now succeed from public facts. CLI,
SQLite projection v2 and Web projection v1 preserve qualified profiles and
condition provenance across the full corpus.

## Open uncertainty

Condition statements remain declarative, and cost bounds remain exact opaque
strings without asymptotic ordering.

## Next falsifiable action

Expose one bounded public selection request over the adopted facts without
stabilizing the private overlay format.

## Blocking structural decisions

A new public selection interface remains subject to human validation.
