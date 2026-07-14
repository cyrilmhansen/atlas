# DEC-063 - Keep presentation descriptions private and derived

## Status

Accepted on 2026-07-14 (`presentation-A`).

## Decision

Generate a private presentation description from validated algorithm, AST and
DatasetSpec information. It selects a small visual primitive and maps named
machine-state channels to labels, highlights and controls.

The first primitive is a sequence. Later experiments may add matrix, graph,
tree and scalar/table primitives only when real corpus cases require them.
Explicit private overrides are permitted where inference from types and effects
is ambiguous.

## Consequences

- Presentation metadata is not added to schema 0.1 and is not registry
  authority.
- The browser page and controls are shared rather than generated separately for
  every algorithm.
- A public `PresentationSpec` requires evidence from at least two domains other
  than sequences and a separate class C schema decision.
