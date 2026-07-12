# DEC-034 - Minimal qualification command

## Status

Accepted reversible MVP 2 implementation, 2026-07-12.

## Decision

Provide `atlas qualify PROBLEM_ID [--stable] [--in-place] [--allocation none]` as the first
constrained selection surface. It joins implementation effects with the
referenced algorithm's stability and in-place claims and prints every selected property's
value, evidence level, and source.

Missing properties do not satisfy a constraint. The command has no expression
language, ranking, benchmark interpretation, or inferred allocation model.

## Consequences

The command demonstrates the MVP 2 query shape "stable in-place sort with no allocation"
without introducing a general planner. New constraint kinds require two real
selection uses or a separate decision.
