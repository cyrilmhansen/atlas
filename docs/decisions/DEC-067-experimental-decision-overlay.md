# DEC-067 - Test ontology facts in a disposable decision overlay

## Status

Accepted on 2026-07-15 (`ontology-A`).

## Context

K-M1 through K-M4-W expose recurring decision-changing losses in schema 0.1:
directional contract relations, qualified costs and guarantees, persistent state
flow, partial finalizers and arithmetic/randomness assumptions. The K-M4 gate
requires an ontology review before corpus growth or K-M5.

Direct schema 0.2 design would stabilize these concepts before Atlas has
measured whether they improve generic decisions or how much authoring structure
they require.

## Decision

Keep schema 0.1 and `registry/atlas.yaml` authoritative. For K-M5, compare the
unchanged schema 0.1 behavior with a private, versioned and disposable decision
overlay containing only facts needed by the accepted discriminants.

The overlay:

- is not a public schema, registry extension, migration format or compatibility
  promise;
- may reference registry entities and frozen import worksheets but does not
  create authoritative Problem, Algorithm or Implementation entities;
- uses closed relation, guarantee, cost-regime and state-flow vocabularies;
- supports only conjunction of explicit facts and directional relations;
- must not parse prose, infer facts from execution, rank candidates, prove
  theorems or branch on implementation/source-family IDs;
- is evaluated against a schema 0.1 control before any promotion proposal.

## Experiment boundary

The first overlay covers at most eight candidates and the seven discriminants in
`docs/phase2/ontology-review.md`. If correct evaluation requires candidate-
specific fields, a general expression language or planner search, stop and
record the representation as unsupported.

Record authoring cost, unique controlled atoms, validation rules and evaluator
code size. Delete the overlay if it does not improve decisions over the control.
Any public promotion requires a separate class C decision, evidence from at
least two structural families and a deterministic schema migration plan.

## Consequences

- K-M5 may implement a small private parser, validator and evaluator.
- The aggregate YAML, SQLite projection, CLI compatibility and public query
  semantics remain unchanged.
- Temporary duplication is accepted only for the duration of the experiment.
- K-M5 results, including negative results, precede any schema 0.2 proposal.

## Alternatives considered

- Direct public schema 0.2: deferred because its migration and compatibility
  costs would precede evidence of decision value.
- Keep schema 0.1 throughout Phase 2: rejected as the only experiment because it
  repeats known failures without testing a bounded remedy.
