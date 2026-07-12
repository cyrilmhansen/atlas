# DEC-008 - Algorithm requirements

- Status: accepted
- Class: C
- Date: 2026-07-11

## Context

Multiple algorithms can solve the same problem under different assumptions.
Binary search, for example, requires an input sequence sorted according to its
comparison order, while linear search does not.

## Options considered

- Add an optional qualified requirement list to `Algorithm`.
- Attach requirements to each concrete implementation.
- Keep requirements in unstructured documentation.

## Decision

Add `requires: Claim<Vec<String>>` as an optional algorithm field. Absence means
that no requirement beyond the problem contract is declared. It does not prove
that the algorithm is universally applicable.

## Consequences

Requirements remain semantic properties of algorithms, independent of execution
backends. Their provenance is validated and `show` exposes them. Requirement
expressions remain human-readable strings in schema 0.1; no expression language
or solver is introduced.
