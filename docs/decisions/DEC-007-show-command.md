# DEC-007 - Show command rendering

- Status: accepted
- Class: B
- Date: 2026-07-11

## Context

The registry needs a direct way to inspect one globally identified entity and
its evidence without exposing a second serialization protocol.

## Options considered

- Print the corresponding YAML fragment.
- Print structured human-readable text.
- Print JSON intended primarily for agents.

## Decision

Use `atlas show <id>` with structured text. Relationships are printed as fields;
each claim prints its value, evidence level, and source. Effects and list values
remain expanded and visible.

## Consequences

The output is intended for inspection, not as a stable machine protocol. Its
shape can evolve during MVP 1 without changing the authoritative YAML schema.
