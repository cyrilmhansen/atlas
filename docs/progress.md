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

Public `search` and `show` now expose Condition entities. A fresh agent used
them to translate the heap requirement, inspect `state.spare_capacity` directly
and reproduce the same two candidates.

## Open uncertainty

One `qualify` request currently carries only one exact cost profile; conjunctions
such as logarithmic time plus no allocation have not been exercised publicly.

## Next falsifiable action

Attempt one request requiring exact time and allocation profiles under the same
condition, and determine whether the current CLI can express it without
approximating either requirement.

## Blocking structural decisions

None.
