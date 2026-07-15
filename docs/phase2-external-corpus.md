# Phase 2 - External corpus trial

Status: active under DEC-066  
Trajectory: Atlas Knowledge first, accepted by DEC-066  
Inputs: project vision, DEC-064 closure evidence and the external audit recorded
in `docs/audits/2026-07-14-external-project-audit.md`

## Current position

MVP 1 through MVP 6 form a closed-supported **Phase 1 - Technical foundation**.
They established a trustworthy registry, evidence levels, deterministic derived
indexes and datasets, bounded composition experiments, native/MIR/RISC-V
execution probes and a reproducible interactive artifact. Those assets remain
maintained evidence, not the automatic center of future work.

The primary unresolved product hypothesis is:

> Can Atlas preserve and normalize knowledge from independent algorithm sources
> well enough for generic selection, composition and agent consumption?

The phase began with a 10-problem, 15-algorithm and 20-implementation sequence
corpus that was too small and homogeneous to answer that question. K-M1 through
K-M3 added graph, dynamic-structure and streaming families. K-M4 has now tested
independent normalization over six frozen subjects; its mixed gate result pauses
corpus growth for a bounded ontology review.

## Accepted strategic trajectory

### Atlas Knowledge first

Run an external-corpus trial while freezing Execution Lab capability growth and
maintaining Explorer. This maximizes new information about ontology, source
fidelity, generic selection and agent use. It is reversible because no existing
runtime or artifact is removed.

Alternatives considered by DEC-066:

- Explorer-first was deferred because it would adapt corpus work to presentation
  before validating the knowledge model;
- Execution-Lab-first was deferred because the existing instruments are already
  sufficient to falsify the next hypothesis.

## Active phase

### Phase question

Can independent sources be represented without systematic semantic loss, and
can Atlas use their contracts to select and compose components without
source-specific planner logic?

### Work-program statuses during the phase

| Program | Status | Allowed work |
|---|---|---|
| Atlas Knowledge | `active` | corpus, import, ontology, contracts, generic query/selection/composition |
| Atlas Execution Lab | `frozen` | CI preservation, correction fixes and diagnostic use only |
| Atlas Explorer | `maintained` | compatibility, accessibility and blocking fixes only |

No MIR feature, new visual instruction, new presentation kind or Web feature is
added merely to make an imported algorithm look complete. Modeling failures are
phase results.

### Corpus sampling target

Use 30 to 50 algorithms as a secondary sampling target across at least three
structurally foreign families and independent source classes. Phase success is
determined by qualitative coverage and decision behavior, not entry count.

| Family | Initial candidates | Modeling pressure |
|---|---|---|
| Graphs | BFS, DFS, Dijkstra, topological sort, connected components, minimum spanning tree variants | node identity, adjacency representation, frontier state, structural preconditions |
| Dynamic structures | union-find, binary heap operations, hash-table lookup/insertion/deletion, collision strategies | persistent state, allocation, amortization, identity and representation invariants |
| Streaming | online mean/variance, bounded top-k, reservoir sampling, exact bounded deduplication, approximate membership | bounded memory, approximation, randomness, numerical stability and persistent state |

Candidate source classes:

- one established reference book or standards-quality educational source;
- one or more recognized Rust or C libraries;
- peer-reviewed papers or stable author publications for specialized variants.

Source selection must respect copyright and license boundaries. Atlas records
bibliographic identity, contracts and independently written structured
descriptions; it does not copy unlicensed prose or code into the repository.

At least two concrete implementations must originate outside Atlas. Prefer
unchanged upstream source plus a thin test adapter. Any semantic rewrite becomes
a separate Atlas implementation with explicit derivation provenance.

### Source fidelity model

Each import report evaluates five independent dimensions:

| Dimension | Required record |
|---|---|
| Bibliographic fidelity | author, title/project, edition or version, page/section or commit, license and retrieval context |
| Algorithmic fidelity | strategy, invariants, validity conditions and guarantees preserved or changed |
| Representational fidelity | source structure, vocabulary and decomposition preserved or normalized |
| Executable fidelity | behavior checked against source examples, tests or implementation where available |
| Declared transformations | translation, specialization, type adaptation, bug correction or pedagogical simplification |

The dimensions are not interchangeable. An Atlas entry may be algorithmically
faithful and executable while deliberately changing representation. It must say
so rather than claiming undifferentiated source fidelity.

## Milestones

### K-M0 - Import protocol and baseline

Status: completed with a mixed, informative result. Six isolated worksheets,
the equivalence matrix and protocol revision `k-m0.2` are recorded. Effort
self-estimates were not comparable and are explicitly unavailable as phase
evidence.

K-M0 working documents:

- `docs/phase2/k-m0-source-selection.md`;
- `docs/phase2/import-worksheet.md`;
- `docs/phase2/import-equivalence-rubric.md`;
- `docs/phase2/current-model-baseline.md`;
- `docs/phase2/k-m0-comparison.md`;
- `docs/phase2/imports/importer-a/` and `importer-b/` (raw submissions).

Deliverables:

- one bounded source-fidelity worksheet separating bibliographic, algorithmic,
  representational and executable fidelity plus declared transformations;
- a time/effort log at corpus-batch granularity;
- an import-equivalence rubric distinguishing identity, semantic, taxonomic,
  operational and acceptable documentary differences;
- a frozen snapshot of current schema/AST coverage before imports.

Pilot cases:

1. BFS and Dijkstra provide a rapid comparison over shared graph vocabulary.
2. Union-find immediately provides a structurally different persistent-state
   case before any conclusion about ontology adequacy.

The primary equivalence question is whether normalized differences change a
selection, substitution or composition decision. Textual YAML equality is not
an objective.

Acceptance:

- BFS, Dijkstra and union-find can be reviewed without a public schema or
  runtime change;
- unresolved mappings remain representable as explicit findings rather than
  forced values;
- the protocol distinguishes quoted source fact, Atlas interpretation and
  executable validation;
- no general importer, metrics service, dashboard or new agent API is created.

### K-M1 - Graph corpus batch

Status: complete. Four exact BFS/Dijkstra contracts and two petgraph 0.8.3
implementation records are in the authoritative registry. The tested adapter,
source transformations, experimental projections and mismatch severities are
documented in `docs/phase2/k-m1-graph-corpus.md`.

Import a coherent graph batch from at least two independent sources. Include
both traversal and weighted/ordering problems. Do not extend the AST or visual
machine during the batch.

Acceptance:

- source terminology and Atlas identity mappings are reviewable;
- representation-dependent preconditions are explicit;
- at least one external implementation is tested through a thin adapter;
- every schema/AST mismatch is recorded with severity and lossiness.

### K-M2 - Dynamic-structure corpus batch

Status: complete. Eleven operation-specific contracts and twelve tested
implementation records cover petgraph union-find, standard-library binary heaps,
hashbrown collision handling and generic candidate discovery. Cost-scope and
persistent-state losses are recorded in
`docs/phase2/k-m2-dynamic-structures.md`; schema 0.1 remains unchanged.

Import union-find, heap and collision-aware hash-table operations. Treat a data
structure with persistent invariants as a family of problems and algorithms,
not automatically as one opaque implementation.

Acceptance:

- construction, mutation, query and lifetime effects remain distinguishable;
- worst-case, amortized and expected costs are not collapsed;
- allocation and caller-provided storage affect qualification where declared;
- at least one new implementation can enter an existing query candidate set
  without modifying query code.

### K-M3 - Streaming and approximation corpus batch

Status: complete. Exact bounded top-k, Welford online moments, Vitter reservoir
sampling and Bloom approximate membership add six contracts and six tested
adapters. Numerical, random and probabilistic modeling losses are recorded in
`docs/phase2/k-m3-streaming-approximation.md`; schema 0.1 remains unchanged.

Import exact and approximate bounded-state algorithms, including at least one
randomized and one numerically sensitive example.

Acceptance:

- deterministic, randomized and seed-conditioned guarantees are distinct;
- accuracy/error and memory bounds retain provenance;
- numerical assumptions and failure regimes remain visible;
- inability of schema 0.1 to express a property is recorded before proposing a
  schema extension.

### K-M4 - Independent dual-import experiment

Status: **complete**. The frozen packet, twelve raw worksheets and adjudicated
matrix are preserved in `docs/phase2/k-m4-source-packet.md`,
`docs/phase2/imports/k-m4/` and
`docs/phase2/k-m4-dual-import-comparison.md`.

Two agents independently import the same six to ten source-selected algorithms
using only the import protocol and current Atlas tools. Neither sees the other's
work before submission.

Compare:

- **identity equivalence**: same problem and algorithm identity;
- **semantic equivalence**: compatible preconditions, outputs and guarantees;
- **taxonomic equivalence**: compatible categories and variant boundaries;
- **operational equivalence**: same selection, substitution and composition
  outcomes;
- **documentary divergence**: wording, reference decomposition and examples that
  do not alter a knowledge decision;
- unresolved ambiguity, authoring time and required human interventions.

Acceptance is not perfect agreement. The experiment must produce a reproducible
agreement matrix, explain disagreements and separate protocol ambiguity from
source ambiguity and model insufficiency.

Pause corpus growth and review the ontology when remaining import differences
change a selection, substitution or composition outcome, or when the same lossy
mapping recurs in two structural families. Do not invent a numeric agreement
score unless the manual pilot demonstrates that one answers a phase question.

That pause condition is met. The two imports diverged on a generic problem
boundary, version-specific cost evidence and Algorithm R's zero-size boundary;
Welford remained unresolved because its article body was not readable from the
packet. Recurrent conditioned-cost, persistent-state, randomness/error and
staged-output losses justify an ontology review, but no public schema choice has
been made.

K-M4-W is a closed source-repair addendum. Two new isolated imports use the
open Pébay 2008 and Chan-Golub-LeVeque 1979 reports to normalize incremental
second central moments under a source-neutral identity. They strongly agree on
the recurrence, `M2` state, guarded variance finalizers and distinct pairwise
combination. They diverge on totalizing empty input and on whether a published
proof is enough for Atlas level `proven`; both questions enter the ontology
review. The frozen packet and comparison are in
`docs/phase2/k-m4-w-source-packet.md` and
`docs/phase2/k-m4-w-comparison.md`.

For future packets, prefer official openly readable primary pages, including
the ACM Digital Library when full text is available. Freeze a content digest
and a readable archival alternative when practical. Open access permits
inspection; it does not silently grant a software license or permission to copy
an implementation.

The required bounded ontology review is now documented in
`docs/phase2/ontology-review.md`. It separates import-protocol defects from
knowledge semantics and query execution, defines seven K-M5 discriminants and
presents two independent human decisions: representation strategy and the
meaning of evidence level `proven`. Options A/A are accepted under DEC-067 and
DEC-068. Protocol `k-m0.3` and the private K-M5 overlay boundary are now frozen;
schema 0.1 remains unchanged.

### K-M5 - Manifest-driven candidate discovery

Status: **closed mixed under DEC-070**.

Add at least one implementation whose compatibility is not named in a composer
scenario. The existing query/composition path must discover, accept or reject it
from manifests and generic constraints.

Acceptance:

- no implementation ID or source-family branch is added to candidate discovery;
- the explanation identifies satisfied and violated constraints;
- effects, allocation and relevant preconditions influence the outcome;
- a negative candidate is as explainable as the selected candidate.

If the current composer cannot perform this without scenario code, record that
as a supported falsification of generic composition rather than hiding it behind
a new hard-coded scenario.

The first private evaluator checkpoint is recorded in
`docs/phase2/k-m5-overlay-result.md`. It covers all seven discriminants through
ten requests without source-family branches and leaves schema 0.1 unchanged.
This does not close K-M5: an independently authored candidate/request pair must
still test whether the overlay vocabulary converges operationally rather than
only fitting the cases used to design it.

That test is frozen in `docs/phase2/k-m5-independent-authoring-packet.md` around
one bounded top-k implementation and three pre-adjudicated requests. The author
receives neither the reference facts nor evaluator tests. A standalone overlay
avoids exceeding the reference experiment's eight-candidate ceiling.

The isolated submission is adjudicated. All three operational decisions agree
without evaluator changes, but taxonomy is mixed: exactness was fused into the
output capability and allocation was represented as an effect rather than a
cost. DEC-069 subsequently tests explicit bounded equivalences rather than
canonicalizing either representation.

DEC-069 accepts `normalization-B`. Two non-recursive equivalences reconcile both
top-k encodings for four requests, including the allocation false acceptance in
the no-equivalence control. The private Rust experiment grows from 826 to 1,146
non-test lines. At that checkpoint, conditional cost equivalence remains
explicitly unsupported pending the conditioned heap falsifier.

The heap falsifier now passes in both directions and preserves rejection without
spare capacity. Condition transport adds 23 lines, bringing the private Rust
total to 1,169. `docs/phase2/k-m5-review.md` recommends closing K-M5 as mixed:
the private evaluator is supported, but schema 0.1 and the existing CLI still
cannot consume the overlay facts. DEC-070 accepts that recommendation, retains
the private condition-aware trial and freezes K-M5.

### K-M6 - Blind agent-consumer experiment

Status: **active packet checkpoint; dual-agent execution pending**.

Give an agent registry/query/compose tools, task requirements and evidence, but
withhold implementation source during selection. Use a new task drawn from the
imported families and not represented by an existing scenario.

Protocol:

1. Record the task and acceptance oracle before the run.
2. Let the agent query alternatives, constraints and evidence.
3. Require a selected plan, rejected alternatives and uncertainty report.
4. Reveal source only for integration and correction testing.
5. Compare against a control agent with project documentation but without Atlas
   query results.

Acceptance:

- the Atlas-assisted agent does not invent unsupported properties;
- selected components satisfy machine-checkable contracts;
- rejected alternatives cite actual constraints;
- integration tests pass or failure is correctly attributed;
- the report measures whether Atlas changed correctness, explanation quality or
  human intervention rather than merely whether a plan was produced.

The frozen top-k task, assisted/control packet boundary, source-reveal procedure
and qualitative oracle are in `docs/phase2/k-m6-agent-consumer-protocol.md`.
Neither arm receives the private K-M5 overlay. Execution requires explicit
dual-agent authorization after packet verification.

### K-M7 - Phase synthesis

Produce a phase audit containing:

- source diversity and imported-entity counts;
- authoring effort distribution;
- agreement and modeling-friction matrices;
- generic candidate-discovery and agent-consumer results;
- proposed schema/ontology deltas, each tied to multiple observed cases;
- AST coverage findings without an automatic AST extension plan;
- recommendation for the next active program.

The phase may conclude **supported**, **mixed**, or **falsified**. A mixed or
negative result is valuable if it identifies where the knowledge hypothesis
fails.

## Phase exit evidence

Phase 2 can close when all of the following are available:

- at least three structurally different algorithm families;
- at least five problems with competing algorithms;
- at least three algorithms with multiple implementations;
- at least two external source types;
- at least two externally authored implementations with preserved provenance
  and executable correction tests;
- at least two cases that require an experimental annotation because schema 0.1
  cannot represent them cleanly;
- at least one substantive semantic divergence detected between independent
  imports;
- source-fidelity and unresolved-mapping records for every corpus batch;
- a dual-agent import agreement report;
- one generic candidate-discovery experiment with no new implementation branch;
- one blind agent-consumer experiment and control comparison;
- an evidence-based list of model changes, with no extension justified by only
  one isolated case.

The 30-to-50 corpus target is a secondary progress indicator. Source diversity,
competing choices, modeling failures and agent outcomes are the exit evidence.

## Explicitly excluded during Phase 2

- MIR, JIT, RISC-V or guest-memory capability growth;
- visual-machine, bytecode or browser-presentation expansion;
- publication/deployment work beyond preserving the existing artifact;
- a universal source importer or automated ontology merger;
- copying copyrighted books or incompatible upstream code;
- public schema changes before corpus evidence and a separate decision;
- a general planner hidden behind source-specific rules;
- treating AST representability as an import acceptance requirement.

No public field or relation can be justified by one family alone. A schema
proposal requires at least two materially independent cases from two structural
families. Until then, use a private experimental annotation, non-stabilized
relation or import report.

## Anti-instrumentation rule

The manual worksheet, comparison matrix, current CLI and existing executable
gates are the default tools. New instrumentation requires a named phase question
that cannot be answered with them. Convenience, aggregate counts or presentation
quality alone do not justify an importer framework, dashboard, metrics service
or new agent API.

## Strategic decisions anticipated

### Resolved by DEC-066 - Activate the trajectory

| Option | Consequence |
|---|---|
| A. Atlas Knowledge first | Highest information gain on the original hypothesis; freezes visible/runtime expansion temporarily. |
| B. Atlas Explorer first | Strongest public demonstration; weaker ontology falsification. |
| C. Atlas Execution Lab first | Deepest runtime research; largest risk of continued center-of-gravity drift. |

Accepted: **A**.

### Resolved by DEC-066 - Preserve source fidelity

| Option | Consequence |
|---|---|
| A. Separate source record from Atlas normalization | Keeps original terminology and ambiguity visible; may require a later schema decision. |
| B. Normalize directly into current entities | Lowest initial machinery; risks erasing source distinctions. |
| C. Store source snapshots as primary authority | High fidelity but creates copyright, versioning and multi-authority problems. |

Accepted: **A**, initially as review documentation and private experimental
annotations rather than a public schema change.

### P2-B1 - Agent interface for the first consumer experiment

| Option | Consequence |
|---|---|
| A. Existing CLI and textual outputs | Tests Atlas as it exists and minimizes experiment tooling. |
| B. New structured agent API | Easier automation but risks optimizing the interface before learning what agents need. |

Recommendation: **A**. Record friction before proposing an API.

### Later decisions, evidence first

- algorithm/variant identity normalization;
- approximation and numerical-error vocabulary;
- persistent-state and data-structure contracts;
- schema 0.2 scope;
- whether the AST should remain specialized, become layered or be replaced;
- whether Explorer or Execution Lab becomes the next active program.

None should be decided solely from the current sequence corpus.

## First active milestone

Current gate: K-M0 is complete. K-M1 must preserve exact source contracts and
experimental projection notes; it must not turn the Dijkstra divergence into a
public schema relation.

1. Select sources and licensing boundaries for the graph pilot.
2. Draft the import worksheet outside the public schema.
3. Import BFS and Dijkstra twice, independently.
4. Immediately import union-find through the same independent protocol.
5. Compare decision-relevant divergence and authoring friction.
6. Adjust the protocol once, then freeze it for the first corpus batches.

This small experiment decides how to conduct the larger phase without first
building a general importer.
