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

An independent agent using only `atlas qualify` reproduced the frozen heap,
hash-map and top-k candidate sets and their provenance. It did not infer an
ordering between distinct opaque top-k bounds.

## Open uncertainty

The successful requests already supplied Atlas IDs and exact bound strings;
translation from an ordinary human requirement remains untested.

## Next falsifiable action

Give an independent agent a natural-language heap requirement and only the
public `search`, `show`, `explain` and `qualify` commands; require it to discover
the relevant Atlas vocabulary and reproduce the known selection.

## Blocking structural decisions

None.
