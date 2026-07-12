# DEC-012 - Operationally distinct implementations

- Status: accepted
- Class: B
- Date: 2026-07-11

## Decision

Count two implementations of one algorithm only when they expose materially
different operational contracts, such as internal allocation versus a
caller-provided output or scratch buffer.

Thin scalar-type wrappers and aliases do not count as separate implementations.

## Consequences

The twenty-implementation target will include five meaningful storage-policy
variants. Their signatures, mutations, and possible allocations must remain
visible in the registry.
