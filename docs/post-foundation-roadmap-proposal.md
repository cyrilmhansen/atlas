# Post-foundation roadmap proposal

Status: proposal; no phase activated  
Recommended trajectory: Atlas Knowledge first  
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

The current 10-problem, 15-algorithm and 20-implementation sequence corpus is
too small and homogeneous to answer that question.

## Strategic trajectory

### Option A - Atlas Knowledge first (recommended)

Run an external-corpus trial while freezing Execution Lab capability growth and
maintaining Explorer. This maximizes new information about ontology, source
fidelity, generic selection and agent use. It is reversible because no existing
runtime or artifact is removed.

### Option B - Atlas Explorer first

Expand the interactive educational product across visually different families.
This offers the clearest near-term public demonstration, but it risks adapting
new corpus entries to presentation constraints before the knowledge model is
validated.

### Option C - Atlas Execution Lab first

Advance memory regions, compact identities, MIR/RISC-V and execution
instrumentation. This deepens a coherent technical laboratory, but yields the
least evidence about the original encyclopedia and agent-consumer hypothesis.

**Recommendation:** accept A for one phase, then use its evidence to decide
whether the next investment belongs in Knowledge, Explorer or Execution Lab.

## Proposed Phase 2 - External corpus trial

### Phase question

Can independent sources be represented without systematic semantic loss, and
can Atlas use their contracts to select and compose components without
source-specific planner logic?

### Work-program statuses during the phase

| Program | Proposed status | Allowed work |
|---|---|---|
| Atlas Knowledge | `active` | corpus, import, ontology, contracts, generic query/selection/composition |
| Atlas Execution Lab | `frozen` | CI preservation, correction fixes and diagnostic use only |
| Atlas Explorer | `maintained` | compatibility, accessibility and blocking fixes only |

No MIR feature, new visual instruction, new presentation kind or Web feature is
added merely to make an imported algorithm look complete. Modeling failures are
phase results.

### Corpus target

Import 30 to 50 algorithms across at least three structurally foreign families
and three independent source classes:

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

## Milestones

### K-M0 - Import protocol and baseline

Deliverables:

- a source-fidelity worksheet covering locator, edition/version, terminology,
  original assumptions, Atlas mapping and unresolved differences;
- a time/effort log at corpus-batch granularity;
- a normalization rubric for identity, problem boundaries, preconditions,
  effects, costs and evidence;
- a frozen snapshot of current schema/AST coverage before imports.

Acceptance:

- two pilot entries can be reviewed without a schema or runtime change;
- unresolved mappings remain representable as explicit findings rather than
  forced values;
- the protocol distinguishes quoted source fact, Atlas interpretation and
  executable validation.

### K-M1 - Graph corpus batch

Import a coherent graph batch from at least two independent sources. Include
both traversal and weighted/ordering problems. Do not extend the AST or visual
machine during the batch.

Acceptance:

- source terminology and Atlas identity mappings are reviewable;
- representation-dependent preconditions are explicit;
- at least one external implementation is tested through a thin adapter;
- every schema/AST mismatch is recorded with severity and lossiness.

### K-M2 - Dynamic-structure corpus batch

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

Import exact and approximate bounded-state algorithms, including at least one
randomized and one numerically sensitive example.

Acceptance:

- deterministic, randomized and seed-conditioned guarantees are distinct;
- accuracy/error and memory bounds retain provenance;
- numerical assumptions and failure regimes remain visible;
- inability of schema 0.1 to express a property is recorded before proposing a
  schema extension.

### K-M4 - Independent dual-import experiment

Two agents independently import the same six to ten source-selected algorithms
using only the import protocol and current Atlas tools. Neither sees the other's
work before submission.

Compare:

- problem and algorithm identity;
- variant boundaries and terminology;
- preconditions, outputs and effects;
- complexity/evidence classification;
- unresolved ambiguity;
- authoring time and required human interventions.

Acceptance is not perfect agreement. The experiment must produce a reproducible
agreement matrix, explain disagreements and separate protocol ambiguity from
source ambiguity and model insufficiency.

Decision trigger:

- below 70% semantic agreement on core contracts, pause corpus growth and revise
  normalization guidance;
- above 25% lossy or blocked mappings in a batch, investigate the ontology before
  extending execution infrastructure.

Thresholds are proposed operational tripwires, not quality claims, and should be
confirmed when the phase is activated.

### K-M5 - Manifest-driven candidate discovery

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

### K-M6 - Blind agent-consumer experiment

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

- 30 or more reviewed algorithm entries from the three families and independent
  source classes;
- at least two externally authored implementations with preserved provenance
  and executable correction tests;
- source-fidelity and unresolved-mapping records for every corpus batch;
- a dual-agent import agreement report;
- one generic candidate-discovery experiment with no new implementation branch;
- one blind agent-consumer experiment and control comparison;
- an evidence-based list of model changes, with no extension justified by only
  one isolated case.

The numeric corpus target is a sampling floor, not the phase hypothesis. Source
diversity, modeling failures and agent outcomes carry more weight than totals.

## Explicitly excluded during Phase 2

- MIR, JIT, RISC-V or guest-memory capability growth;
- visual-machine, bytecode or browser-presentation expansion;
- publication/deployment work beyond preserving the existing artifact;
- a universal source importer or automated ontology merger;
- copying copyrighted books or incompatible upstream code;
- public schema changes before corpus evidence and a separate decision;
- a general planner hidden behind source-specific rules;
- treating AST representability as an import acceptance requirement.

## Strategic decisions anticipated

### P2-C1 - Activate the trajectory

| Option | Consequence |
|---|---|
| A. Atlas Knowledge first | Highest information gain on the original hypothesis; freezes visible/runtime expansion temporarily. |
| B. Atlas Explorer first | Strongest public demonstration; weaker ontology falsification. |
| C. Atlas Execution Lab first | Deepest runtime research; largest risk of continued center-of-gravity drift. |

Recommendation: **A**.

### P2-C2 - Preserve source fidelity

| Option | Consequence |
|---|---|
| A. Separate source record from Atlas normalization | Keeps original terminology and ambiguity visible; may require a later schema decision. |
| B. Normalize directly into current entities | Lowest initial machinery; risks erasing source distinctions. |
| C. Store source snapshots as primary authority | High fidelity but creates copyright, versioning and multi-authority problems. |

Recommendation: **A**, initially as review documentation rather than a public
schema change.

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

## First two weeks of work after activation

1. Select sources and licensing boundaries for the graph pilot.
2. Draft the import worksheet outside the public schema.
3. Import BFS and one structurally contrasting weighted algorithm twice,
   independently.
4. Measure disagreement and authoring friction.
5. Adjust the protocol once, then freeze it for the first graph batch.

This small experiment decides how to conduct the larger phase without first
building a general importer.

