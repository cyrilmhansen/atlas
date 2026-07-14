# External project audit - post MVP 6

Date recorded: 2026-07-14  
Origin: independent agent audit supplied by the project owner  
Status: external opinion, non-normative  
Related checkpoint: MVP 1 through MVP 6 closed under DEC-064

This document preserves the substance of the external review used to prepare
the post-foundation vocabulary and roadmap. It does not override the vision,
accepted decisions, schema or tests.

## Global diagnosis

The project has not betrayed its initial vision. Its guardrails worked notably
well:

- `Problem`, `Algorithm`, `Implementation` and `Execution` remain separate;
- YAML and Git remain authoritative, with SQLite derived and rebuildable;
- declared, inferred, tested, observed and proven properties remain distinct;
- native implementations remain correction oracles;
- MIR remains outside the semantic knowledge model;
- compact 32-bit guest references remain separate from RV64ILP32;
- private formats were not stabilized prematurely;
- structural extensions were preceded by explicit decisions.

The review considers the project unusually disciplined for such rapid agentic
development. Bounded compositions are not presented as a general planner,
specialized MIR lowerings are not presented as a universal compiler, the visual
machine remains private, and browser observations are not confused with
asymptotic characteristics.

The main concern is not a violation of principles but a shift in center of
gravity. The original project was primarily a knowledge infrastructure for
describing, comparing, selecting and composing algorithms. The implemented
project has become, to a significant extent, a rigorous execution, observation
and presentation infrastructure for a small sequence-algorithm corpus. This is
a useful lateral branch, but it has developed before the main knowledge-model
hypothesis has been tested on a sufficiently broad corpus.

## Results aligned with the vision

### Trust model

The separation between theory, declaration, tests, local observations and
backend behavior is one of the strongest project outcomes. Interactive runs do
not infer complexity, local benchmarks do not establish general rankings, and
measurement context is retained. This directly supports the intended reliable
algorithm encyclopedia.

### MIR kept in its proper role

Atlas obtained a MIR interpreter, JIT probes, RV64 generation, one compact
guest-offset region, controlled imports and native comparisons without making
MIR a registry or composition dependency. Closing MVP 4 before multiple memory
regions was considered a sound boundary. `GuestOffset(u32)` is already a useful
answer to compact references on a 64-bit architecture without relying on an
immature RV64ILP32 ABI.

### Visualization answers a real initial need

The visual machine, exact AST-node links, operation counters and successive
states make structures, operations, mutations and implementation choices
inspectable at several abstraction levels. MVP 5 and MVP 6 are therefore not
off-topic; the review argues that they arrived early relative to corpus growth.

## Main drift: optimizing the instrument before widening its subject

The corpus remains at the MVP 1 minimum: 10 problems, 15 algorithms and 20 Rust
implementations. Meanwhile Atlas gained a registry, SQLite projection, dataset
model, traces, AST, pseudocode parser, MIR adapter, guest-memory runtime, JIT
observation, Capstone integration, RV64 probes, Web projection, WASM runtime,
incremental steppers, visual bytecode, derived presentation and reproducible
responsive bundle.

The resulting infrastructure density per algorithm is high. The five visual
lowerings are real and reviewed, but they still belong to one close family:
imperative scalar algorithms over indexed sequences with comparisons and local
mutation. The current visual machine therefore demonstrates factorization of
known cases, not yet natural accommodation of structurally foreign algorithms.

## Central hypotheses still weakly tested

### Selection and composition remain scenario-led

The five MVP 3 compositions expose useful and explainable plans, but candidates,
objectives and scenarios are bounded in code. A stronger test would add a new
implementation without modifying the composer and observe it becoming an
eligible or rejected candidate from contracts alone. A stronger test still
would add a new problem and implementations through manifests, then build a
pipeline not anticipated by scenario-specific code.

### Ontology remains lightly stressed

The current homogeneous corpus does not answer several important questions:

- Can independently sourced vocabularies describe the same problem without
  artificial identity fragmentation?
- Can subtle variants remain explicit without multiplying arbitrary IDs?
- Are preconditions interpreted generically or primarily by special code?
- Can dominance, effects and allocation influence selection and composition?
- Can partial, approximate, probabilistic or numerically unstable algorithms be
  represented faithfully?
- Can an external source remain recognizable without being rewritten into an
  Atlas-native dialect?

### Agent consumption has not yet been demonstrated

An agent has developed Atlas effectively, but this differs from an agent using
Atlas to design software. The decisive experiment is whether an agent denied
implementation source, but given registry contracts, alternatives, evidence
and tools, can select and compose components correctly for a new request.

## Process risk: proof bureaucracy

The 64 accepted decisions, six closed MVPs and detailed progress journal show
strong governance. They also create a risk of a self-validating process where a
small extension is bounded, implemented, tested, audited and closed without
substantially testing the fundamental product hypothesis.

The audit recommends reserving `MVP` for an actual product-validation unit and
using a clearer vocabulary:

- **Phase** for a major product hypothesis;
- **Milestone** for a coherent deliverable;
- **Experiment** for a bounded technical question;
- **Decision** for an architectural or governance choice.

## AST risk: becoming the product by accumulation

The AST now unifies pseudocode, traces, partial MIR lowering, visual programs,
presentation and differential validation. This is promising, but it gives the
AST the de facto role of a universal semantic intermediate language before it
has encountered independent corpora.

The risk is epistemological rather than code-quality related: Atlas may become
excellent at representing algorithms that resemble its current AST. Stress
families include recursive trees, graph frontiers, union-find, collision-aware
hash tables, probabilistic algorithms, parsing, matrix dynamic programming,
bounded streaming, floating-point numerical methods and concurrency.

The recommendation is not to extend the visual machine immediately. First
confront the knowledge model and AST with two or three foreign families and
record what cannot be expressed faithfully.

## Three possible trajectories

### A - Return to the encyclopedic core

Freeze MIR, RV64, Web, visual-machine and private bytecode/presentation
extensions. Use them as existing test instruments. Start a phase focused on an
external corpus and knowledge-model robustness.

Candidate target: 30 to 50 algorithms from independent sources, including a
reference book, established Rust or C libraries, papers and possibly a teaching
collection. Preserve exact provenance, avoid forced normalization, measure
authoring cost, detect synonyms and variants, and let observed failures drive
schema evolution.

Key experiment: two independent agents import the same algorithms, then compare
identity, decomposition, preconditions, effects and classification.

### B - Adopt an interactive educational product

Treat the original result as an interactive atlas combining pseudocode, state,
operations, invariants and characteristics. Expand to visually different
algorithm families, multi-scale narration, side-by-side comparison, memory and
cost visualization, pedagogy and agent explanations.

This would be an immediately demonstrable product, but closer to an executable
algorithm museum than to the original metaprogrammatic encyclopedia.

### C - Recenter on the fantasy computer

Advance explicit memory regions, compact references, runtime devices, code
generation and native execution visualization. This could form a coherent
educational fantasy computer, but is the trajectory most likely to hide the
knowledge product behind its machine.

## Recommended organization: three work programs

The audit recommends keeping one repository initially while distinguishing:

- **Atlas Knowledge**: schema, corpus, provenance, ontology, contracts,
  selection, composition and imports;
- **Atlas Execution Lab**: AST, traces, MIR, compact references, RISC-V, WASM
  and the visual machine;
- **Atlas Explorer**: catalog, search, comparison, interactive execution and
  human-facing explanation.

The intended authority direction is:

```text
Atlas Knowledge says what is known and why.
Atlas Execution Lab tests how selected knowledge can execute.
Atlas Explorer chooses how to expose it to humans.
```

Execution Lab and Explorer must not silently define Knowledge semantics.

## Recommended next phase

Suggested name: **Phase 2 - Foreign corpus trial**.

Five objectives:

1. Import at least three algorithm families foreign to the current corpus.
2. Import at least two implementations not written for Atlas.
3. Preserve the link between original sources and Atlas representations.
4. Identify schema and AST failures without immediately repairing all of them.
5. Have an agent perform a new selection and composition without a specially
   coded scenario.

Suggested families:

- graphs: BFS, DFS, Dijkstra and topological sort;
- dynamic structures: union-find, heap and hash table;
- streaming: online mean, bounded top-k and exact or approximate bounded
  deduplication.

These cases stress identity, references, non-contiguous structures, allocation,
persistent state, structural preconditions, memory/time tradeoffs, separate
outputs and scratch, and approximation without first extending runtime memory.

## Verdict

The project has not drifted in principles. It has drifted in effort allocation
toward execution, visualization, Web reproducibility and backends while the
central question - whether a rich, composable algorithm knowledge library can
be built and consumed - remains tested on a small homogeneous corpus.

The resulting infrastructure is not wasted. It provides strong falsification
instruments. The audit's recommendation is to freeze those instruments, apply
them to foreign material and test Atlas with an agent acting as a consumer.
