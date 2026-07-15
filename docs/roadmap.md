# Project roadmap

This roadmap refines the vision without silently changing public schema or
accepted scope. Decisions remain authoritative when this document and a
decision record differ.

## Current position

MVP 1 through MVP 6 are closed. No subsequent MVP is active. GitHub CI runs the
reproducible MVP 2 acceptance gate, all workspace targets, the RV64 LP64 probe
and the MIR RV64 generator probe on pushes and pull requests. The project has a
Git-authoritative YAML registry (31 problems, 36 algorithms and 40
implementations), a rebuildable SQLite projection, deterministic datasets and
reproducible local observations, plus five bounded composition scenarios with
compiled Rust orchestration. The current counts include the 10/15/20 MVP 1
sequence baseline and the first external graph batch.

MVP 5 delivered the reproducible static interactive artifact. MVP 6 replaced
its specialized browser execution paths with five AST-derived private programs,
one bounded WASM visual machine and derived presentation data. DEC-064 closes
that scope while retaining private test-only exports as differential oracles.

An external post-foundation audit recommended shifting new work back to corpus,
ontology, generic selection and agent consumption while freezing execution and
presentation capability growth. DEC-065 and DEC-066 accept the vocabulary and
activate Phase 2. The audit and maintained planning documents are:

- `docs/audits/2026-07-14-external-project-audit.md`;
- `docs/project-vocabulary.md`;
- `docs/phase2-external-corpus.md`.

Atlas Knowledge is active. Atlas Execution Lab is frozen and Atlas Explorer is
maintained for the duration of Phase 2.

K-M0 is complete with a mixed, informative result. Its six isolated imports and
comparison report are under `docs/phase2/`. BFS and union-find converged at the
decision level; Dijkstra exposed a real difference between exact source
contracts and a common all-distances projection. Protocol `k-m0.2` freezes
mandatory source pages and external batch timing after optional source coverage
and self-estimated effort proved non-comparable.

K-M1 is complete. It imports separate petgraph/algs4 BFS traversal/path and
Dijkstra distance/path-tree contracts, plus two real petgraph 0.8.3
implementations exercised through a thin test adapter. Output projections,
numeric domains and graph constraints remain recorded limitations rather than
new schema fields or graph-specific query flags.

K-M2 is complete. It represents union-find, binary-heap and collision-aware
hash-table construction, mutation and queries as explicit state transitions.
The batch demonstrates generic candidate discovery and allocation qualification
without query changes. It also independently reproduces two model pressures:
amortized sequence costs and typed persistent-state continuity. Both remain
documented rather than silently forced into schema 0.1. K-M3 streaming and
approximation is complete.

K-M3 adds exact bounded top-k, numerically sensitive online moments,
seed-reproducible reservoir sampling and one-sided Bloom membership. It confirms
that randomness identity, seed-conditioned determinism, numerical error and
probabilistic accuracy are decision-relevant properties absent from schema 0.1.

K-M4 independent dual import is complete. Two isolated importers produced twelve
worksheets from one frozen six-subject packet. Three of five source-readable
subjects converged operationally; heap identity, hashbrown cost evidence and the
zero-size reservoir boundary exposed decision-changing differences. Welford
remained unresolved because the packet lacked a readable article body. The
accepted phase pause condition is therefore met.

K-M4-W closes the algorithmic source-access gap with two open primary reports
without rewriting K-M4 or claiming fidelity to unread Welford text. Independent
imports converge on a neutral incremental second-moment identity, distinct
pairwise combination and guarded variance finalizers. Empty-state totalization
and the meaning of evidence level `proven` remain decision-relevant ontology
questions.

Next is a bounded ontology review, not more corpus or instrumentation. It must
clarify generic problem identity and contextual evidence, synthesize recurrent
model losses across families and present any public schema alternative as a
class C decision. K-M5 manifest-driven candidate discovery remains the next
falsification experiment after that review.

The review in `docs/phase2/ontology-review.md` is accepted under DEC-067 and
DEC-068. A disposable experimental decision overlay will be measured against
unchanged schema 0.1 before any schema 0.2 proposal. `proven` is reserved for an
auditable claim-to-proof mapping. Protocol `k-m0.3` and the private overlay
experiment are specified. Its closed Rust parser and validator reject unknown
fields, broken atom references, duplicate candidates and unaudited `proven`
facts. A bounded YAML overlay now exercises all seven discriminants through ten
requests, and a 230-line generic evaluator produces five positive and five
negative adjudications without candidate-specific branches. The full 826-line
non-test Rust cost is part of the experiment result. K-M5 remains open for the
independent-authoring test; no public schema promotion is implied.

MVP 4 closed under DEC-052. It established a pinned upstream MIR
interpreter boundary, a standard RV64 LP64 compiler/QEMU-user probe, an
independent comparison of three compact guest-reference candidates, and a
complete interpreter-only capability ladder over one bounded offset region.
The demonstrated native/MIR pairs now cover partition, `is_sorted`,
minimum/maximum, reverse and stable insertion, in addition to scalar probes.
None changes the public registry schema, execution-record format or native
reference backend. On x86-64, a narrowly configured embedded Capstone now
decodes the observed scalar, read-only guest and mutating guest JIT functions
without external tools. Generated RV64 scalar, read-only and mutating probes run
under QEMU user mode. These remain diagnostics, not persistent backend
artifacts.

The DOCX snapshot is preserved at `doc/Vision_Atlas_Executable_MVP1-4.docx`.
`docs/vision.md` is its verified, diffable Markdown conversion and the
maintained source of vision under DEC-020.

## Close MVP 1

### 1. Documentation and schema authority

Status: complete for MVP 1 pre-freeze.

- Maintain the vision in diffable Markdown under DEC-020 while retaining the
  DOCX snapshot.
- Decide whether schema 0.1 must add implementation version, license, target,
  dependencies, and ABI before stabilization.
- Define additive, breaking, and migration rules for schema versions.

Required decisions: class C.

### 2. Evidence and coverage

Status: local evidence resolution complete under DEC-026; required-field
coverage enforced by schema.

- Resolve local `file:` sources and entity references. Complete.
- Verify registered Rust test evidence against source symbols. Complete for the
  MVP 1 Rust corpus.
- Report mandatory-property coverage by entity kind. Complete through required
  schema fields and committed-registry acceptance tests.
- Reject stale or malformed local provenance references with actionable errors.
  Complete.

The first implementation should validate only source schemes already present in
the corpus. It must not become a universal URI framework.

### 3. Deterministic derived index

Status: complete under DEC-022 through DEC-024. Projection version 1 is
documented in `docs/sqlite-projection-v1.md`.

- Rebuild exclusively from the aggregate YAML source. Complete.
- Compute and verify a canonical logical digest. Complete.
- Keep generated database files outside Git authority. Complete.

Required decisions: SQLite dependency, projection schema, and digest format.

### 4. Acceptance gate and repository baseline

- Provide one local command covering format, Clippy, core/alloc/hash feature
  profiles, workspace tests, registry validation, coverage, and index rebuild.
  Complete under DEC-027 with `scripts/check-mvp1.sh`.
- Add CI only after that command is stable locally.
- Create the initial Git commit when the responsible human approves the baseline.
  Complete locally.

## MVP 2 empirical qualification

Status: closed locally under DEC-036. The work below records the delivered
scope; it does not activate MVP 3.

- Implement the accepted separate `DatasetSpec`, `DatasetCase`, and generated
  instance model. Experimental Rust model implemented for the first slice.
- Begin with deterministic typical, boundary, degenerate, adversarial, and
  regression cases for sorting and partitioning. Complete for the first slice.
- Exercise sorting independently of benchmark execution with a deterministic
  matrix of 12 instances: 3 sizes and 4 input distributions.
- Exercise partitioning with a matching deterministic matrix covering uniform,
  alternating, fully selected, and fully rejected inputs.
- Generate execution observations outside Git authority with compiler, target,
  parameters, seed, commit, environment, and result provenance. A deterministic
  correction recipe is complete for sorting and partitioning under DEC-033. A
  qualified sorting benchmark recipe records raw evidence without ranking.
- Keep correction tests and benchmarks as separate execution modes.

No benchmark conclusion is accepted before environment and dispersion are
recorded.

The first constrained selection command is accepted under DEC-034. It remains
a property filter, not the MVP 3 composition planner.

Status: the first release-only comparison harness records raw samples, robust
dispersion, complete local context, and dataset identity under DEC-031. Results
remain ephemeral and non-normative.

`docs/mvp2-review.md` records the closure criteria and explicit limits.
`scripts/check-mvp2.sh` validates the non-timing acceptance slice.

Generated comparison reports are implemented under DEC-035. They remain local,
dataset- and context-bounded observations rather than a ranking service.

DEC-036 defers numeric resource constraints, algorithm-only allocation and
traversal measurements, and cross-evidence queries. Any later work in those
areas must define its measurement boundary and provenance before implementation.

## Algorithm representation research

After MVP 1 closure, prototype the accepted structured algorithm AST on two
materially different cases: top-down merge sort and in-place partition.

The experiment must demonstrate:

- human-readable structured pseudocode;
- explicit reads, writes, comparisons, swaps, allocations, and effects;
- invariant checkpoints;
- rendering independent of Rust;
- minimal semantic traces over small dataset cases.

Status: deterministic semantic traces and experimental structured pseudocode
ASTs are implemented for merge sort and in-place partition under DEC-029. Every
trace step is validated against its exact AST operation node. Structural
expressions and parameter access modes are typed under DEC-030.

DEC-044 adds a separate test-only textual editing experiment for adjacent
`is_sorted` and two-pointer partition. Each private source must parse to an AST
structurally equal to its Rust builder; it is not yet a schema or source of
truth.

The AST, dataset, trace and textual pseudocode shapes remain non-public. The
current experiments show useful common structure, but the test-only parser also
shows algorithm-specific expression costs. Promotion requires a separate
decision informed by real imported sources, not only the local sequence corpus.

## MVP 3 and MVP 4

MVP 3 is closed locally under DEC-038 after a narrow experiment in explainable
constrained selection and linear composition. Its first slice uses one real sequence
pipeline, internal structural types, and a non-public readable plan. The first
scenario, `atlas compose cleanup`, selects in-place filter, insertion sort, and
quadratic deduplication for declared intermediate allocation minimization; it
also explains a copying merge/hash alternative rejection. The structured AST may
support readable orchestration but must not expand MVP 3 into a general compiler.

The selected scenario can render verified Rust source under `--rust`; the source
is compiled as the `cleanup_generated` example. It remains an internal
single-scenario generation experiment, not a plan format or compiler service.

The same scenario also demonstrates a second `expected-time` objective from
declared complexity claims, selecting the copying merge/hash candidate under its
`i32: Eq + Hash` condition. This demonstrates an explicit trade-off, not a
benchmark-derived ranking. Both selected candidates render Rust source compiled
as a matching example; this still does not create a general compiler service.

A second scenario, `atlas compose find`, demonstrates an explicit produced
precondition: insertion sort establishes the ordering required by binary search.
The plan and compiled Rust example make that mutation and the rejected
allocation-heavy merge alternative visible.

Three further bounded scenarios exercise complementary plan shapes without
introducing a planner: `partition-sort` makes a structured intermediate and its
projection/reassembly explicit, `unique-sort` separates the required unique
output allocation from rejected intermediate merge/hash storage, and
`merge-sorted` forms a two-input fan-in after establishing both sorted-input
preconditions. All have compiled Rust orchestration examples.

The first explicit override surface is also implemented: `--force` and
`--forbid` select only between each scenario's reviewed candidates, retaining a
reason or rejecting an empty candidate set. This satisfies the forcing/forbid
experiment without turning MVP 3 into general search or mutable registry state.

MVP 4 closed under DEC-052 as a narrow LP64 MIR interpreter, host-JIT and
QEMU-user probe. DEC-049 validates a scalar MIR RV64 generator artifact,
DEC-050 adds a read-only checked guest import and DEC-051 adds mutation.
The `atlas-algorithms` core APIs remain the native reference backend; MIR
remains an adapter and never defines registry semantics, compact references, or
evidence formats. Timed JIT measurement, multiple guest regions, RV64ILP32 and
a fantasy computer remain separate experiments.

## MVP 4 execution path

The closed work progressed through the following gates. A failed or
inconclusive gate records a limit; it does not silently widen the runtime.

### 1. Select and specify compact guest references

Status: complete for the single-region checkpoint under DEC-040 and DEC-041.

- `GuestOffset(u32)` is the byte offset in one fixed-capacity guest region.
- Offset zero is valid; MVP 4 has no null reference or guest-visible growth.
- Typed accesses must declare alignment and reject overflow and out-of-bounds
  offsets before reaching the host buffer.
- Bounds-checked little-endian 8-byte loads and stores are exercised by several
  interpreter programs; overflow, alignment and bounds failures are tested.

Exit evidence: a documented memory model and a reproducible test that never
passes a host pointer as a guest reference.

### 2. Lower one existing algorithm AST to private MIR

Status: complete for two AST-backed subsets under DEC-041 and DEC-043.

- Keep the native algorithm as the correction oracle.
- Reuse an existing typed AST rather than inventing a second semantic model.
- The partition read, predicate, swap and boundary subset now lowers to private
  MIR over the selected offset region.
- Result and typed semantic trace links are checked against native Rust and the
  existing AST on deterministic boundary and mixed cases.
- Adjacent `is_sorted` adds a read-only lowering with first-inversion and early
  stop checks.

Exit evidence: one AST-backed algorithm runs through native Rust and MIR with
the same declared semantics. No public backend trait or persistent plan format
is introduced by this gate.

### 3. Exercise guest references in that lowered algorithm

Status: complete for one fixed-capacity region.

- Pass compact references as MIR scalar values, never as host pointers.
- Make every load, store, bounds check and imported runtime operation visible
  in the lowered program and trace.
- Ordinary and boundary algorithm cases match native Rust. Invalid, unaligned
  and overflowing references are rejected by the memory-model tests.

Exit evidence: a minimal sequence operation reads or writes guest memory through
the selected model and matches the native reference result.

### 4. Compare interpreter and optional JIT behavior

Status: correction and structural inspection complete under DEC-046 through
stable insertion; performance measurement excluded from MVP 4 by DEC-052.

- Measure startup latency, code size and correction equivalence separately.
- Keep JIT results local observations with environment and protocol provenance.
- Do not promote latency or throughput results to general claims from one host.

Exit evidence: a reproducible local comparison that justifies retaining or
rejecting the MIR generator for this project.

### 5. Reassess RISC-V code generation and the fantasy-computer profile

Status: scalar generation and single-region read/write imports complete under
DEC-049 through DEC-051; multi-region runtime and machine profile deferred.

- Keep the generated probes independent of registry semantics and persistent
  artifacts.
- Keep QEMU user mode as the LP64 Linux ABI probe; do not imply a machine model.
- Extend additional runtime imports only through a separate target-boundary
  decision.
- Define devices, console and clock only if a separate MVP decision activates a
  system-emulation experiment.

Exit evidence: target-specific probes remain independent of registry semantics
while reproducing the selected single-region offset checks where guest memory
is exercised.

## Dual-backend rollout

Status: native Rust and MIR are retained indefinitely under DEC-042. Native
Rust remains the correction and qualification authority; MIR support is
additive, bounded and never required for registry or composition operation.

| Step | MIR capability | First candidates | Acceptance evidence |
|---|---|---|---|
| Complete | scalar operations and private trace imports | addition, minimum | result and exact trace checks |
| Complete | one guest offset region, reads, writes and swaps | even partition | native output/boundary equality and typed AST trace links |
| Complete | adjacent reads and comparisons | `sequence.is_sorted` | boolean result, first inversion behavior and typed AST trace links |
| Complete | scalar selection scan | `sequence.minimum`, `sequence.maximum` | value/index and first-occurrence tie policy |
| Complete | swap-only sequence mutation | `sequence.reverse` | exact reversal and double-reversal property |
| Complete | shifted writes over private 16-byte pairs | insertion sort | exact native equality, sortedness, stability and permutation |
| Deferred | additional regions and outputs | merge sort, filter, merge-sorted, deduplicate | explicit allocation/copy/region semantics |

For every row, preserve the native implementation and its tests. Execute native
and MIR on the same deterministic cases; compare return values, mutated/output
data and applicable invariants. Require trace-to-AST links only where the
existing experimental AST represents the lowered operations. A missing MIR
counterpart means only that the experimental adapter does not support the
algorithm.

Do not compare interpreter timings with native benchmark results. Interpreter
cost, JIT startup, generated-code size and target execution remain different
observation protocols. No backend is chosen automatically from those results.

## MVP 4 single-region checkpoint

Status: closed under DEC-052 after interpreter, host-JIT and RV64 read/write
correction checkpoints.

The checkpoint demonstrates the interpreter and guest-offset boundary across
read-only scans, selection, swaps and shifted writes. It also demonstrates
exact AST trace links for two materially different control-flow shapes. It does
not demonstrate a general AST compiler, timed JIT behavior, multi-region memory
or a persistent backend artifact. Exact host-code spans and instruction shapes
plus scalar and read/write guest RV64 generation are observed locally, but no
timed JIT or executable-allocation protocol has been introduced.

Deferred work after MVP 4 closure:

1. Add a bounded construction/execution latency or executable-allocation probe
   only if a concrete backend-retention question requires it; retain
   interpreter traces as the observability reference.
2. Introduce multiple regions only when an output/scratch algorithm is selected
   and region identity, lifetime and copy visibility have been accepted.

Neither item is active. A new MVP must define its exit criteria before widening
the memory model or measurement surface.

## MVP 5 public interactive artifact

Status: closed under DEC-060 at the extended DEC-058/DEC-059 boundary. No public
projection format, deployment target or general browser runtime is active.

The recommended distribution is a static, reproducible website that can be
hosted on GitHub Pages or opened from a release bundle. It would combine a
read-only projection of the Atlas registry with a small Rust/WebAssembly
runtime for curated local executions. It would require no application server,
account, database service or remote code execution.

The authority chain should remain explicit:

```text
registry YAML + sources + decisions
  -> atlas validate
  -> derived web projection + WASM + build metadata
  -> static public site
```

The derived projection must carry the source commit and logical registry
digest. It is disposable build output, never registry authority. If its shape
becomes a supported public interface, it needs an independently versioned schema
decision before publication.

### Proposed public experience

1. Browse and search problems, algorithms and implementations with provenance,
   requirements, effects and known limitations.
2. Show theoretical time and auxiliary-space complexity as sourced Atlas
   claims. Never infer `O(...)` from a short interactive run.
3. Select a versioned `DatasetSpec` case or enter a bounded custom sequence,
   then run one of a deliberately curated set of native Rust algorithms locally
   through WebAssembly.
4. Display input, output, checked invariants and deterministic operation counts
   such as comparisons, reads, writes, swaps, copies and requested auxiliary
   storage.
5. Offer step/run/reset controls through a stateful incremental WASM executor.
   Its current operation must identify the exact algorithm or AST node. Keep
   bounded traces as validation/analysis instruments rather than UI timelines.
6. Optionally display wall-clock duration as a local browser observation with
   dataset, repetitions, browser, target, timer resolution and warnings. It
   must remain visually and semantically separate from complexity claims and
   native benchmark evidence.

The first executable slice should cover three materially different cases from
the existing corpus: read-only `is_sorted`, mutating stable insertion sort, and
in-place `reverse` or partition. This is enough to test results, mutation,
stability and operation counters without exposing a general plugin system.

### Proposed implementation stages

| Stage | Deliverable | Acceptance boundary |
|---|---|---|
| 0. Distribution contract | accepted scope, audience, derived-data policy and browser support | MVP activation and exit criteria accepted before code |
| 1. Static catalog | generated entity pages/search data from the validated registry | exact entity counts, links, provenance and commit digest |
| 2. Local execution | one small `wasm32` crate exposing curated typed functions | browser results equal native correction fixtures and dataset digests |
| 3. Characteristics | sourced `O(...)` claims plus deterministic operation counters | theoretical, counted and timed properties remain distinct |
| 4. Semantic dynamics | bounded incremental WASM execution for algorithms with validated operations | replay is deterministic, current operations map to declared AST nodes, and analytical traces remain optional test oracles |
| 5. Distribution | reproducible static bundle and optional GitHub Pages workflow | clean checkout rebuilds the same logical content; deployment is explicit |

Current status:

- Stage 0 is complete under DEC-053 through DEC-057.
- Stage 1 has a private deterministic full-corpus projection and searchable
  catalog carrying the source commit and logical registry digest.
- Stages 2 and 3 have passed read-only `is_sorted`, mutating stable-insertion
  and symmetric-reverse gates with native/WASM equivalence, exact semantic
  operation counts, visible original indices, sourced complexity and qualified
  local timing. Reverse also checks that a second application restores input.
- The selector is derived from all five cases of
  `dataset.sequence.sort.m2.v0`, retaining spec/problem attribution, class,
  seed and content digest. Compatible reuse for `is_sorted` and `reverse` is
  visible and does not imply new problem-specific specs.
- The complete bundle gate passes from a Git archive without `.git`, `target` or
  prior Web output. All eight generated files match the normal build bit for
  bit, and effective build/runtime environments are exposed.
- DEC-058 extends the active scope with Stage 4 before closure. Its first gate is
  bounded adjacent-`is_sorted` dynamics linked to exact AST nodes, alongside
  deterministic editable-data generation and a separate scale regime.
- That first gate is complete: Explore provides pseudocode-linked playback up to
  64 elements, while Scale plots exact comparisons/swaps through 4096 elements
  without treating browser timing as complexity evidence. DEC-059 subsequently
  moved the presentation to incremental WASM state; its analytical trace is now
  a test oracle only.
- Insertion adds a typed AST, parser-equivalent pseudocode and a stateful WASM
  stepper under DEC-059. It retains only the current tagged sequence and is
  checked operation-by-operation against the analytical trace and finally
  against native stable insertion. Reverse adds a typed symmetric-pair AST,
  parser-equivalent pseudocode and a stepper checked against native mutation,
  exact structural counts and involution. The three curated dynamics adapters
  are complete. The extended clean-archive gate reproduces all ten files byte
  for byte at `75ceb69`.
- DEC-060 formally closes MVP 5 without promoting a public trace API, stable Web
  contract or deployment target. External publication remains excluded and no
  subsequent MVP is active.

Resource limits must be part of the local execution boundary: maximum input
length, analytical-trace cap, step budget, cancellation and no network access from
algorithm execution. Custom input is ephemeral and must not be presented as
registry evidence. Accessibility, keyboard operation and small-screen layout
are acceptance requirements for a public artifact, not deferred decoration.

MIR should initially be shown as documented backend evidence: supported probes,
instruction shapes and target limits. Running MIR or an RV64 emulator inside
the browser is not required for the first artifact and would need a separate
cost/benefit decision. Native Rust/WASM remains the correction path.

MVP 5 exit evidence:

- a static bundle builds from a clean checkout without a server;
- every displayed registry fact resolves to source provenance;
- three curated algorithms execute locally and match native fixtures;
- complexity claims, operation counts and local timings cannot be confused;
- the bundle exposes commit, registry digest and build/runtime environment;
- CI validates the bundle, but publication occurs only through an explicitly
  accepted deployment decision.

## MVP 6 generic executable presentation

Status: closed under DEC-064, with private generated bytecode selected by
DEC-062 and a private derived presentation description selected by DEC-063.

MVP 6 addresses the scaling limit demonstrated by the three hand-written MVP 5
Web paths. It does not make the browser a general algorithm runtime. The visual
machine is initially limited to one bounded `i32` sequence, bounded scalar
state, structured control and exact AST-linked read, write, compare and swap
operations.

Planned stages:

| Stage | Deliverable | Acceptance boundary |
|---|---|---|
| 0. Responsive execution band | **Complete:** viewport-width state visualization with mobile state priority | no page overflow at 390, 768, 1440 and 1920 pixel viewports |
| 1. Program contract | **Complete for the current read-only and mutating subsets:** private validated instruction and state model | unsupported AST shapes fail before bundle generation |
| 2. First generated execution | **Complete:** one `sequence.minimum` program and shared sequence renderer | exact AST nodes and native result agree |
| 3. Mutating generality | **Complete:** even partition through the same program and machine | mutation, counters and partition invariant agree with native Rust |
| 4. Differential migration | **Complete (3/3):** `is_sorted`, stable insertion and reverse generated | step and final-state equivalence with retained hand-written references |
| 5. Consolidation | **Complete under DEC-064:** no specialized import or dispatch remains in `app.js`; private test-only WASM exports remain differential oracles | five algorithms, no per-algorithm production-browser export or `app.js` dispatch branch |

Exit evidence must include a clean reproducible static bundle, native
equivalence for all five algorithms, exact AST identity, bounded declarative
algorithm data and multi-viewport browser inspection. The generated program,
visual machine and presentation description remain private. Performance
fingerprint implementation is deliberately deferred to a later scope; its
research basis is `docs/performance-model-research.md`.

The implemented contract and its bounds are recorded in
`docs/mvp6-visual-machine.md`. Stage 3 adds explicit swap mutation and origin
tracking. Its provisional `predicate_even` intrinsic is not a general call
model; widening predicates or calls remains a separate decision.

## Strategic decisions to prepare

These are intentionally visible before implementation. Each item records its
accepted or open status; open class C items must not be implemented by default.

### C1. First compact-reference model

Context: all three candidates have distinct overflow, bounds and identity tests.
The first executable guest runtime now uses one fixed-capacity offset region.

| Option | Consequence |
|---|---|
| A. `u32` offset in one bounded region | Smallest runtime and lowest call overhead; cannot express separate object identity without extra conventions. |
| B. `u32` handle through an object table | Explicit identity and lifetime checks; adds table indirection and allocator policy. |
| C. region ID plus offset | Clear separation between regions; uses more representation and requires region lifecycle rules. |

Accepted and demonstrated: **A** under DEC-040 through partition, `is_sorted`,
selection, reverse and insertion experiments. Any replacement or extension to
multiple addressable regions still requires a new decision because it changes
private runtime semantics.

### C2. Boundary between existing AST and MIR lowering

Context: partition and `is_sorted` now prove that exact existing AST node IDs
can constrain and validate private MIR experiments. A second semantic authority
would still make trace validation weaker.

| Option | Consequence |
|---|---|
| A. Handwritten MIR programs per experiment | Fastest probes; no demonstration that the existing AST can drive a backend. |
| B. Private lowering of one existing typed AST | Tests the intended adapter direction while retaining a narrow, reversible implementation. |
| C. Public generic backend trait and plan format | Reusable on paper, but premature and schema-shaping. |

Accepted and demonstrated: **B** under DEC-041 and DEC-043 for partition and
`is_sorted`. Minimum/maximum, reverse and insertion remain specialized MIR
programs rather than AST lowerings. No generic lowering or public backend trait
is implied.

### C3. Status of MIR semantic traces

Context: `MinimumTrace` is intentionally private and cannot be mistaken for
observed registry evidence. A cross-backend trace is useful only once there is
a stable semantic event vocabulary and provenance policy.

| Option | Consequence |
|---|---|
| A. Keep traces private through MVP4 | Lowest risk; correction tests inspect traces only inside adapter tests. |
| B. Define a versioned internal trace artifact | Enables replay tooling, but requires identifiers, provenance and retention rules. |
| C. Add traces to public execution evidence | Makes traces queryable, but changes schema and archival policy. |

Current policy: **A**. Partition and `is_sorted` now have matching typed AST
links without serialization, and that has been sufficient for correction and
diagnosis. Revisit B only if replay across processes becomes a concrete need.
B or C remains class **C** because either introduces a durable format.

### B1. Interpreter versus JIT evaluation protocol

Context: the interpreter is valuable for observability; MIR generation may be
valuable for startup latency and compact native code. The completed
single-region matrix now provides workloads suitable for a controlled probe.

Recommendation: retain the interpreter as the correction and trace backend and
define a local protocol comparing JIT construction latency, generated-code size
and execution separately on the same deterministic inputs. Protocol details are
class **B**. Compiling and enabling MIR generator sources was class **C** and
is now accepted by DEC-046/C6 below.

Preparation complete: Atlas now selects level 2 explicitly and verifies
correction at MIR optimization levels 0 through 3. The public MIR API does not
expose exact generated-code length. The size protocol must therefore choose
between parsing pinned debug output, measuring executable allocation footprint,
or proposing a narrow upstream API; these observations are not interchangeable.

DEC-047 selects the narrow upstream API path for local preparation. The patch
now observes exact relocated function bytes without IO, and Atlas verifies a
copied scalar observation and a guest-memory control-flow observation after
context destruction. Integration remains provisional until the patch is
reviewed and available from the original MIR remote.

DEC-048 accepts the separate Capstone dependency with only x86 enabled. The
diagnostic exposes relative instruction offsets, bytes, mnemonics and operands
through the first return, while retaining padding and relocation data as an
unclassified suffix. This is enough to inspect the two current generated
functions without defining a persistent instruction schema or pretending to
reconstruct arbitrary control flow. RV64 decoding remains disabled until Atlas
observes actual RV64 code rather than host x86-64 JIT output.

The untimed level matrix now covers scalar addition, read-only guest-memory
`is_sorted`, mutating `reverse`, nested-scan `partition` and stable insertion
over private pairs. It verifies correction and repeated structural summaries at
levels 0 through 3. On the pinned x86-64 stack, guest level 1 is smaller than
levels 2 and 3 for `is_sorted` and partition, while reverse levels 1 through 3
share the same prefix length and insertion levels 2 and 3 are slightly smaller
than level 1; level 0 is larger for every guest workload. Calls and branch
classes remain invariant. This rules out using the numeric optimization level
as a proxy for compactness. The remaining B1 work is to separate construction
latency, execution latency and executable allocation footprint before selecting
or ranking a level.

### C4. Rust toolchain support baseline

Context: the workspace declares Rust 1.85, but the currently locked
`rusqlite 0.40.1` does not compile on that compiler. The GitHub workflow uses
current stable Rust so it validates the actual dependency graph rather than an
unsupported minimum-version claim.

| Option | Consequence |
|---|---|
| A. Raise the declared minimum after a reproducible probe | Honest support contract; may exclude older local toolchains. |
| B. Pin or replace dependencies to retain Rust 1.85 | Preserves the stated floor; adds maintenance and compatibility constraints. |
| C. Keep an unverified 1.85 declaration | Misleading; CI success would not establish the claimed support. |

Recommendation: **A**, after recording the oldest Rust release that compiles
the locked workspace and its feature profiles. This is a public toolchain
support decision, so changing `workspace.package.rust-version` is class **C**.

### C5. Post-MVP4 representation for imported algorithms

Context: industrial import from TAOCP, open-source repositories and papers
needs a source-faithful representation before it can become executable. Rust
source alone is insufficient, while the current AST remains experimental and
deliberately narrow.

| Option | Consequence |
|---|---|
| A. Curated structured pseudocode plus citations | Faithful to books and papers; requires human normalization before execution. |
| B. Language-specific source adapters plus extracted metadata | Reuses real code; provenance is strong but semantics depend on each language. |
| C. A universal executable IR | Uniform downstream tooling; premature without several independently imported corpora. |

Recommendation: **A** as the first post-MVP4 research track, with links to
source material and separate dataset specifications. Add B only after two
imports expose common extraction needs. Do not start C without evidence from
multiple source families. Decision class: **C** when an import format becomes
persisted or public.

### C6. First MIR generator and host-JIT slice

Context: the interpreter path is now broad enough for correction, but the MIR
generator is not compiled. Enabling it changes the native build and execution
surface even if the API remains private.

| Option | Consequence |
|---|---|
| A. Host JIT for one scalar and one guest workload | Tests correction with the smallest new build surface and permits separate latency/size measurement. |
| B. Continue interpreter-only through MVP 4 | Preserves simplicity but leaves a central vision claim unevaluated. |
| C. Start directly with MIR RISC-V generation | Reaches the target sooner but mixes generator, target and emulator failures. |

Accepted: **A** under DEC-046. Scalar addition and guest `is_sorted` now
reproduce interpreter and native results without timing. The later approved
`reverse`, partition and stable insertion extensions add checked guest writes,
nested scans and shifted private pairs without changing the model or lifecycle.
The interpreter remains responsible for semantic trace events. The remaining
measurement protocol is class B; MIR RISC-V generation remains outside this
decision. DEC-049 later accepts only the independent scalar RV64 generator
probe.

### C7. Multi-region guest memory

Context: merge sort, copying filter, merge-sorted and deduplication require
separate input, output or scratch identities. A second region changes reference
identity and lifetime rules; a larger undifferentiated arena would hide those
effects.

| Option | Consequence |
|---|---|
| A. Fixed host-owned region table plus `(region_id, u32 offset)` | Makes input/output/scratch identity and bounds explicit without guest allocation. |
| B. Multiple slices inside one `GuestOffset(u32)` arena | Reuses the current scalar reference but makes object identity conventional. |
| C. `u32` handles through an object table | Supports object lifetime naturally but adds indirection and allocation policy. |

Recommendation: **A**, beginning with exactly two or three fixed regions for
one real algorithm and no guest-visible allocator. This is class **C** and must
be accepted before implementation.

### C8. First standalone MIR RV64 generator probe

Context: host JIT correction established MIR generation on x86-64, while the
existing RV64 probe established only GCC, ELF and QEMU user-mode availability.
MIR selects its backend when `mir-gen.c` is compiled, so a target probe must run
the RV64-compiled generator itself.

| Option | Consequence |
|---|---|
| A. Scalar RV64 generator artifact | Isolates generator, allocator, emulator and instruction inspection before runtime imports. |
| B. Start with guest-memory `is_sorted` | More representative, but mixes generator validation with target ABI and import failures. |
| C. Defer RISC-V generation | Avoids target tooling but leaves the central MVP 4 target claim untested. |

Accepted: **A** under DEC-049. The temporary LP64D probe generates a 16-byte
addition, executes it under QEMU with result 42, and verifies `add` plus `ret`
through the cross-toolchain disassembler. Extending RV64 code to Atlas guest
imports remains class **C**. DEC-050 later accepts only the first read-only
import.

### C9. First RV64 guest-memory import

Context: the scalar generator isolates MIR's target backend, allocator and QEMU
execution, but it does not cross the runtime call boundary used by Atlas guest
memory. Read-only adjacent `is_sorted` already has interpreter and host-JIT
oracles over one bounded offset region.

| Option | Consequence |
|---|---|
| A. Checked `is_sorted` load import | Tests target calls, offsets, bounds and control flow without mutation. |
| B. Start with mutating `reverse` | Covers reads and writes immediately but mixes more failure sources. |
| C. Stop the RV64 track at scalar generation | Retains the isolated proof but leaves runtime interoperability unknown. |

Accepted: **A** under DEC-050. Four shared correction fixtures pass through the
generated 128-byte RV64 function, which performs ten checked little-endian
loads. An inconsistent span is rejected before execution without an import
call. DEC-051 later accepts the first checked store and mutation probe.

### C10. First RV64 guest-memory write

Context: DEC-050 validates target calls, offsets, bounds and read-only control
flow. Mutation adds a second runtime import and makes byte preservation on error
part of the target correction boundary.

| Option | Consequence |
|---|---|
| A. Checked `reverse` loads and stores | Smallest mutation, with exact output and involution as strong oracles. |
| B. Start with mutating `partition` | Adds a returned boundary and nested scans, but mixes more failure sources. |
| C. Stop RV64 at read-only guest memory | Preserves the smaller target ABI but leaves writes unvalidated. |

Accepted: **A** under DEC-051. Empty, singleton, even and odd fixtures pass
through the generated 176-byte RV64 function. Valid runs perform 12 checked
loads and 12 checked stores, double reversal restores the input, and an invalid
span causes no import or mutation. Multi-region memory remains class **C**.

### C11. MVP 4 closure boundary

Context: interpreter, host-JIT and RV64 probes cover the complete selected
single-region read/write ladder. Additional mono-region algorithms add coverage
but do not resolve a new architectural uncertainty.

| Option | Consequence |
|---|---|
| A. Close MVP 4 at the demonstrated checkpoint | Preserves the narrow result and makes deferred runtime work explicit. |
| B. Activate multi-region memory inside MVP 4 | Reaches output/scratch algorithms but changes identity and lifetime semantics. |
| C. Add another mono-region RV64 algorithm | Increases coverage with limited strategic information. |

Accepted: **A** under DEC-052. No subsequent MVP is active.

### C12. Public distribution architecture

Context: Atlas can be shown publicly as searchable knowledge plus local
algorithm execution. Distribution changes the audience and build surface, and
publication is outside the closed MVPs.

| Option | Consequence |
|---|---|
| A. Static site plus curated Rust/WASM execution | No server or remote execution; supports a real interactive artifact and reproducible hosting. |
| B. Static read-only catalog | Lowest runtime risk but does not demonstrate the executable-atlas claim. |
| C. Application server executing algorithms | Enables centralized measurements but adds operations, security and reproducibility costs. |

Accepted: **A** under DEC-053. A reproducible release bundle is the first
distribution boundary. GitHub Pages and other external publication remain
class **D** and require separate approval.

### C13. Browser data and runtime boundary

Context: the browser needs compact structured data and callable algorithms, but
neither the registry YAML nor all internal Rust APIs should accidentally become
public Web contracts.

| Option | Consequence |
|---|---|
| A. Disposable derived projection plus a curated WASM facade | Keeps Git/YAML authoritative and exposes only tested use cases; projection can evolve before stabilization. |
| B. Parse authoritative YAML directly in the browser | Avoids a projection step but couples UI, validation and schema evolution. |
| C. Publish a stable JSON API and broad algorithm ABI now | Maximizes reuse but prematurely freezes two public contracts. |

Accepted: **A** under DEC-054. The projection must include commit and logical
digest. Its initial shape remains private and disposable; stabilization later
requires a separate class C schema decision.

### C14. Interactive characteristics and measurement semantics

Context: users should see both theoretical complexity and behavior on working
data. Browser timing alone cannot establish asymptotic complexity or portable
performance.

| Option | Consequence |
|---|---|
| A. Sourced complexity plus deterministic counters and optional local timing | Separates theory, algorithm dynamics and machine-dependent observation. |
| B. Browser wall-clock charts as the primary characteristic | Visually simple but noisy and easy to overinterpret. |
| C. Publish server-produced benchmark rankings | More controlled, but creates infrastructure and evidence-governance work. |

Accepted: **A** under DEC-055. Count comparisons, reads, writes, swaps, copies
and auxiliary storage only where their boundary is explicit and tested. Label
wall time with dataset and browser environment. A public or persisted
observation format remains a separate class C decision.

### C15. MVP 5 activation boundary

Context: DEC-053 through DEC-055 select the distribution, browser boundary and
measurement semantics. No implementation MVP is active yet. The first slice
must demonstrate Atlas publicly without freezing private build formats or
combining catalog, traces, deployment and general execution into one project.

| Option | Consequence |
|---|---|
| A. Activate one local static vertical slice through characteristics | Catalog plus `is_sorted`, stable insertion and `reverse`; sourced complexity, counters and qualified local timing; traces and publication deferred. |
| B. Activate a read-only static catalog only | Validates projection and presentation cheaply, but does not demonstrate local execution. |
| C. Activate the complete stages 1-5 roadmap | Delivers traces and deployment together, but creates a broad frontend/runtime/distribution scope with weak intermediate gates. |

Recommendation: **A**. The exit artifact is a locally openable static bundle,
not a published service. Its projection and WASM facade stay private and
replaceable. The minimal risk experiment is native/WASM result equivalence for
`is_sorted` before adding mutation, counters and browser timing. Activation is
class **C**; external publication remains class **D**.

Accepted: **A** under DEC-056.

### C16. Private browser/WASM boundary

Context: the first execution gate needs to transfer bounded sequences between
JavaScript and Rust without turning a temporary bridge into a general algorithm
ABI. The crate and generated JavaScript must also be testable in CI.

| Option | Consequence |
|---|---|
| A. Pinned `wasm-bindgen` facade | Generated typed glue and managed memory transfer; adds exact crate/CLI version coupling. |
| B. Raw pointer/length exports | No binding dependency, but requires a custom allocator, lifetime rules and manual error convention. |
| C. WIT/Component Model | Strong interface description, but disproportionate browser tooling for this bounded MVP. |

Accepted: **A** under DEC-057. Version 0.2.100 is pinned for both crate and CLI;
generated bindings remain ignored products. The private facade currently
contains only a bounded `is_sorted` observation and is not a stable ABI.
