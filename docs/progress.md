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

None. The bounded public selection slice is complete.

## Principal recent result

The CLI selects the heap time and allocation profiles separately, but cannot
express their conjunction. The legacy implementation-effect allocation filter
is not a valid substitute and correctly yields no candidate.

## Open uncertainty

The schema contains the necessary facts; only the public query cardinality is
insufficient.

## Next falsifiable action

If approved, allow repeated exact `--cost` groups with AND semantics and
per-profile conditions, then rerun the heap conjunction.

## Blocking structural decisions

Extending the public `qualify` syntax to repeated cost groups requires human
validation.
