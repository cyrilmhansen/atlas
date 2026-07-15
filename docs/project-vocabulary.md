# Atlas project vocabulary

Status: accepted under DEC-065  
Basis: post-MVP 6 external audit and current project evidence  
Scope: project planning and governance only; no public schema change

## Why change the vocabulary

`MVP 1` through `MVP 6` were useful bounded checkpoints, but the term now mixes
product hypotheses, backend experiments and deliverable milestones. Continuing
that sequence would make progress easy to count while obscuring which central
hypothesis is being tested.

The proposed vocabulary separates the long-lived work program, the hypothesis
under test, the deliverable, the technical experiment and the accepted choice.
Historical MVP names remain unchanged in decisions and audit records.

## Three orthogonal work programs

### Atlas Knowledge

The authority-bearing core:

- public schema and semantic model;
- corpus, source fidelity and provenance;
- problem/algorithm/implementation/execution identity;
- contracts, requirements, effects and confidence levels;
- query, selection, comparison and composition;
- import and normalization procedures.

Knowledge may consume evidence produced elsewhere, but its semantics do not
depend on an execution backend or user interface.

### Atlas Execution Lab

The experimental laboratory:

- structured pseudocode and typed AST experiments;
- semantic traces and differential checks;
- native adapters, MIR, JIT and RISC-V probes;
- guest references and private memory models;
- WASM and the private visual machine;
- performance-fingerprint experiments.

Execution Lab artifacts are non-normative unless a separate decision promotes a
result into Atlas Knowledge. Its current Phase 3 status is **frozen**: correction
and CI maintenance continue, capability expansion does not.

### Atlas Explorer

The human-facing product surface:

- catalog browsing and search;
- comparison and provenance display;
- interactive execution and visualization;
- explanations and agent-assisted exploration;
- static distribution artifacts.

Explorer derives facts from Knowledge and may invoke bounded Execution Lab
capabilities. Presentation choices do not become knowledge semantics by use.
Its current Phase 3 status is **active** under DEC-071.

These programs are ownership and planning boundaries, not new repositories,
crates or deployment units. A code split requires separate evidence.

## Planning terms

### Phase

A **Phase** tests one major product hypothesis across one or more work programs.
It has an explicit question, excluded scope and exit evidence. Only one phase is
active by default.

Example:

> Phase 2 - External corpus trial: can independent algorithm sources be
> represented faithfully enough for generic selection and agent consumption?

A phase is not closed because its planned tasks are finished; it is closed when
the hypothesis is supported, falsified or deliberately abandoned.

### Milestone

A **Milestone** is a coherent, demonstrable deliverable inside a phase. It may
combine corpus, tools, documentation and tests. Milestones should be large
enough to change what can be demonstrated.

Examples:

- one source-faithful graph import batch;
- a measured dual-agent normalization study;
- one manifest-driven selection with no scenario code change.

Milestones are the default unit for commits, progress summaries and external
pushes. They do not each require an architectural decision.

### Experiment

An **Experiment** answers one bounded technical or epistemic question with a
reproducible protocol and an interpretable negative result.

Every experiment states:

- question and competing hypotheses;
- controlled inputs and procedure;
- measurements or comparison rubric;
- stop condition;
- result and limits;
- consequence for the current phase.

Examples include two independent imports of the same source, an AST coverage
assessment or an agent selection blind to implementation source. An experiment
is not a capability claim and need not leave production code behind.

### Decision

A **Decision** is an accepted architectural, semantic or governance choice with
alternatives and consequences. Existing `DEC-NNN` records remain the format.

Create a decision for class B/C choices that affect authority, compatibility,
scope or several modules. Do not create one for local names, routine tests,
progress checkpoints or every experiment result.

### Gate

A **Gate** is an executable or auditable acceptance check. It answers whether a
criterion holds; it does not by itself prove that the phase hypothesis matters.

Examples: manifest validation, clean-archive reproduction, differential native
equality and a blind-agent rubric threshold.

### Evidence

**Evidence** is the material supporting a claim: source citation, test result,
observation, proof or audit. Registry evidence keeps the existing confidence
levels. Project-planning evidence does not automatically become registry data.

### Audit

An **Audit** assesses a phase, milestone or project boundary against declared
questions and risks. It may be internal or external and must state provenance
and authority. An audit recommends; a Decision accepts.

### Artifact

An **Artifact** is a reproducible output intended for inspection or use, such as
a static bundle, report, generated plan or corpus snapshot. An artifact is not
necessarily authoritative or persistent.

### Corpus batch

A **Corpus batch** is a reviewable import unit from a named source family. It
records source scope, selection rules, importer, normalization effort, unresolved
mapping questions and accepted entries. Batches prevent aggregate corpus counts
from hiding source-specific modeling failures.

## Status vocabulary

Use these status words consistently:

| Status | Meaning |
|---|---|
| `proposed` | documented for review; no implementation authority |
| `active` | explicitly accepted work with current exit criteria |
| `maintained` | supported and corrected, without planned capability growth |
| `frozen` | no capability growth; only preservation and blocking fixes |
| `blocked` | accepted work cannot progress without an external decision or dependency |
| `closed-supported` | hypothesis checkpoint complete; compatibility and maintenance continue |
| `closed-archived` | retained as evidence; no compatibility promise |

Avoid using `complete` for a phase without saying whether its hypothesis was
supported, falsified or abandoned.

## Lightweight identifiers

Names are primary; identifiers exist only where cross-references need them:

- phases: `PHASE-02`;
- milestones: `K-M1`, `K-M2` within Atlas Knowledge;
- experiments: `EXP-K-001`, `EXP-X-001` only when a durable protocol exists;
- decisions: existing `DEC-NNN` sequence;
- corpus batches: source-oriented names such as `graph-textbook-batch-01`.

Do not number routine tasks or progress entries.

## Historical translation

The existing history remains immutable, but it can be understood as:

| Historical work | New interpretation |
|---|---|
| MVP 1 | Knowledge foundation milestone: registry and pilot corpus |
| MVP 2 | Execution Lab/Knowledge evidence milestone |
| MVP 3 | Knowledge composition experiments |
| MVP 4 | Execution Lab MIR/RISC-V experiment series |
| MVP 5 | Explorer static-artifact milestone |
| MVP 6 | Execution Lab/Explorer generated-presentation milestone |

Together these form **Phase 1 - Technical foundation**, closed-supported under
the existing decisions. This reinterpretation does not rewrite those decisions.

## Governance cadence

- Maintain one short phase status document and one milestone-oriented progress
  log; avoid narrating every local edit.
- Require a Decision only for genuine class B/C choices.
- Review phase evidence at meaningful milestones, not after every experiment.
- Track corpus growth by source diversity and modeling disagreement, not only by
  entity count.
- Keep negative results and unsupported representations visible.
- Push at milestones, consistent with the established repository policy.

## Current program statuses

| Program | Status | Reason |
|---|---|---|
| Atlas Knowledge | `maintained` through Phase 3 | preserve schema 0.1, provenance and correctness without active ontology growth |
| Atlas Execution Lab | `frozen` | the five accepted visual programs are sufficient for the initial product workflows |
| Atlas Explorer | `active` through Phase 3 | test whether existing knowledge and execution evidence form a useful human-facing product |
