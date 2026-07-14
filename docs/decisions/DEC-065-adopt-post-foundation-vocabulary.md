# DEC-065 - Adopt the post-foundation project vocabulary

## Status

Accepted on 2026-07-14 (`vocab-A`).

## Decision

Adopt three orthogonal work programs:

- **Atlas Knowledge** for schema, corpus, provenance, ontology, contracts,
  selection, composition and imports;
- **Atlas Execution Lab** for AST, traces, MIR, compact references, RISC-V,
  WASM and visual-machine experiments;
- **Atlas Explorer** for catalog, comparison, interactive execution and
  human-facing explanation.

Use the following planning vocabulary for new work:

- **Phase**: one major product hypothesis;
- **Milestone**: one coherent demonstrable deliverable;
- **Experiment**: one bounded falsifiable question and protocol;
- **Decision**: one accepted architectural, semantic or governance choice;
- **Gate**: one executable or auditable acceptance check.

Keep the historical MVP 1 through MVP 6 names unchanged. Interpret them
together as **Phase 1 - Technical foundation**, `closed-supported` under their
existing decisions. Do not continue automatic `MVP 7`, `MVP 8`, ... numbering.

## Consequences

- Work-program boundaries are planning and authority boundaries, not new crates,
  repositories or deployment units.
- Milestones become the default progress, commit and push unit.
- Decisions are reserved for genuine class B/C choices rather than routine
  implementation or every experiment result.
- A phase closes when its hypothesis is supported, falsified or abandoned, not
  merely when its task list is exhausted.
- Atlas Knowledge semantics never derive silently from Execution Lab or Explorer
  implementation convenience.
- `docs/project-vocabulary.md` is the maintained vocabulary reference.

## Alternatives considered

- Adopt the terms without the three work programs: rejected because it would not
  resolve the observed center-of-gravity ambiguity.
- Continue MVP numbering: rejected because it conflates product hypotheses,
  technical experiments and deliverable milestones.
