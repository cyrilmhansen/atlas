# DEC-006 - Optional domain-specific algorithm properties

- Status: accepted
- Class: C
- Date: 2026-07-11

## Context

Schema 0.1 initially required `stable` and `in_place` for every algorithm.
Those properties describe sorting strategies but are not meaningful for every
problem family, such as sequence search.

## Options considered

- Store arbitrary properties in an untyped map.
- Make known domain-specific properties optional while retaining typed claims.
- Introduce property groups for each domain immediately.

## Decision

Make `stable` and `in_place` optional typed claims. Their absence means the
property is not asserted for that algorithm; it does not mean `false`.

## Consequences

The schema remains small and typed. New properties will only be introduced when
real corpus cases justify them. Adding or restructuring public properties still
requires a class C decision.
