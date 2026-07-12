# DEC-010 - Explain implementation chain

- Status: accepted
- Class: B
- Date: 2026-07-11

## Context

`show` displays one entity, but users and agents also need to inspect how a
concrete artifact connects to its strategy and problem contract. MVP 1 does not
yet perform automatic selection, so `explain` must not imply selection reasons.

## Options considered

- Make `explain` a more verbose alias for `show`.
- Resolve and display the `Implementation -> Algorithm -> Problem` chain.
- Defer explanation until constrained selection exists.

## Decision

Use `atlas explain <implementation-id>`. Print the resolved chain first, then
the complete structured views of the implementation, algorithm, and problem.
This includes effects, requirements, claim levels, and provenance.

## Consequences

Only implementation IDs are accepted because they uniquely provide the complete
three-level chain. The command explains registry relationships and evidence; it
does not claim that the implementation was selected or preferred.
