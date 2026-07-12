# DEC-016 - Separate dataset specifications

- Status: accepted, implementation deferred to MVP 2
- Class: C
- Date: 2026-07-11

## Decision

Introduce dataset descriptions as separately identified, referenced objects.
The intended model distinguishes deterministic generation specifications, small
materialized demonstration cases, and generated instances identified by
parameters, seed, and content digest.

## Consequences

Problem, algorithm, implementation, test, benchmark, and execution records may
reference datasets without embedding or duplicating them. Typical, boundary,
degenerate, adversarial, randomized, and regression classifications are planned.
Tests and benchmarks remain separate uses of the same specification.
