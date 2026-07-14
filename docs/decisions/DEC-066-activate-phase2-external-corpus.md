# DEC-066 - Activate Phase 2 external corpus trial

## Status

Accepted on 2026-07-14 (`phase2-A`).

## Decision

Activate **Phase 2 - External corpus trial** in Atlas Knowledge. During the
phase:

- Atlas Knowledge is `active`;
- Atlas Execution Lab is `frozen` except for preservation and blocking fixes;
- Atlas Explorer is `maintained` without capability expansion.

The phase tests whether Atlas can preserve and normalize independent algorithm
sources well enough for generic selection, composition and agent consumption.
Total corpus size is a secondary indicator, not the primary exit criterion.

## Mandatory pilot

The first dual-agent experiment imports BFS and Dijkstra for a rapid shared-
vocabulary comparison, then immediately imports one structurally different
case: union-find. No conclusion about ontology adequacy may be drawn from the
two graph algorithms alone.

Before either agent starts, the comparison rubric must distinguish:

- identity equivalence;
- semantic equivalence of requirements, outputs and guarantees;
- taxonomic equivalence;
- operational equivalence for selection, substitution and composition;
- acceptable documentary divergence.

The primary comparison question is whether remaining differences change a
selection, substitution or composition decision, not whether YAML is textually
identical.

## Source fidelity

Every imported item distinguishes:

- bibliographic fidelity: author, edition, page/section or commit and license;
- algorithmic fidelity: strategy, invariants and validity conditions;
- representational fidelity: source structure and vocabulary;
- executable fidelity: tested behavioral correspondence where executable;
- declared transformations: translation, specialization, type adaptation, bug
  correction or pedagogical simplification.

Source fact, Atlas interpretation and executable validation remain separate.

## Schema restraint

No public schema extension may be accepted from one algorithm family alone. A
candidate field or relation requires at least two materially independent cases
from two structural families, plus a separate schema decision. Until then,
source-specific information may remain in a private experimental annotation,
non-stabilized relation or import report.

AST or visual-machine representability is not an import acceptance criterion.
An inability to represent a source cleanly is phase evidence, not a trigger for
immediate runtime expansion.

## Qualitative exit floor

Phase synthesis requires at least:

- three structurally different algorithm families;
- five problems with competing algorithms;
- three algorithms with multiple implementations;
- two external source types;
- two cases that cannot be represented cleanly without experimental annotation;
- one real semantic divergence detected between independent imports;
- one new candidate discovered without implementation-specific planner logic;
- one blind agent-consumer experiment with a control comparison.

A working target of 30 to 50 algorithm entries remains useful for sampling, but
cannot compensate for missing qualitative evidence.

## Anti-instrumentation boundary

- Start with review documents and existing CLI tools.
- Do not build a general importer, ontology merger, metrics service, dashboard or
  new agent API before the manual BFS/Dijkstra/union-find pilot is reviewed.
- Add instrumentation only when a named phase question cannot be answered with
  the current worksheet, comparison matrix and executable gates.
- MIR, JIT, RISC-V, guest-memory, visual-machine, bytecode and Explorer feature
  growth remain excluded.

## Consequences

- The first active milestone is `K-M0`, import protocol and heterogeneous pilot.
- Corpus growth pauses if imports expose ontology ambiguity that would make
  further entries systematically lossy.
- Negative and blocked mappings are retained as valuable phase results.
- No schema 0.2, runtime extension or publication work is authorized.
- `docs/phase2-external-corpus.md` is the maintained phase plan.

## Alternatives considered

- Atlas Explorer first: deferred because it would optimize presentation before
  testing source fidelity and knowledge consumption.
- Atlas Execution Lab first: deferred because existing instruments are already
  sufficient to falsify the next knowledge hypothesis.
