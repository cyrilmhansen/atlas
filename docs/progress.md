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

`atlas qualify` now selects exact schema 0.2 cost profiles and prints their
evidence. It distinguishes conditioned heap and hash-map facts and keeps the two
opaque top-k memory bounds separate.

## Open uncertainty

Condition statements remain declarative, and cost bounds remain exact opaque
strings without asymptotic ordering.

## Next falsifiable action

Give an independent agent only the public Atlas CLI and three frozen requests
across heap, hash-map and top-k, then compare its candidate choices with the
known exact-profile results.

## Blocking structural decisions

None.
