# DEC-074 - Activate Phase 4 comparative foreign selection

## Status

Accepted on 2026-07-15 (`phase4-A`).

## Context

Phase 2 supports foreign knowledge preservation and agent consumption but does
not support public qualified selection or generic composition. Every foreign
problem still has at most one registered algorithm or implementation, so the
model has not been tested against independently sourced competitors. Phase 3
made the existing knowledge inspectable and is closed supported under DEC-073.

The Phase 2 exit audit recommended adding competing graph, dynamic-structure
and streaming strategies before considering schema 0.2. The owner now accepts
that deferred option as the next phase.

## Decision

- Activate **Phase 4 - Comparative foreign selection**.
- Make Atlas Knowledge active, keep Atlas Explorer maintained and public, and
  keep Atlas Execution Lab frozen.
- Preserve schema 0.1, the aggregate YAML registry and current CLI as the
  authoritative control at phase start.
- Freeze selection requests and expected accept/reject reasons before importing
  each competitor.
- Exercise three distinct foreign families: graph reachability, dynamic
  priority queues and exact bounded streaming top-k.
- Require candidate discovery from registered relations rather than candidate
  identifiers embedded in query code.
- Treat a schema 0.1 inability to decide as a valid result, not permission to
  infer missing facts or silently promote the private K-M5 overlay.

This decision does not authorize schema 0.2, a stable selection protocol, a
general planner, new composition behavior, AST/MIR/WASM growth or new Explorer
features.

## Consequences

- The first milestone is a frozen decision matrix and schema 0.1 baseline, not
  a new evaluator.
- Each corpus batch needs explicit source, license, transformation and
  implementation provenance.
- A phase-local experimental representation may be proposed only after the
  baseline identifies a concrete decision that schema 0.1 cannot express.
- No public schema proposal may rely on only one family; recurring needs must be
  demonstrated in at least two structurally different families.

## Alternatives considered

- Comparative foreign selection (`phase4-A`): accepted as the smallest
  reversible test of the central unsupported Knowledge hypothesis.
- Direct schema 0.2: rejected as premature before foreign competition reveals
  which structured facts change decisions.
- Further Explorer or Execution Lab growth: rejected for this phase because it
  would not test qualified selection.
