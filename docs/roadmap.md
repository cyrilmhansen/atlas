# Project roadmap

This roadmap refines the vision without changing MVP scope. Accepted decisions
remain authoritative when this document and a decision record differ.

## Current position

MVP 1, MVP 2 and MVP 3 are closed locally. The project has a Git-authoritative
YAML registry (10 problems, 15 algorithms and 20 Rust implementations), a
rebuildable SQLite projection, deterministic datasets and reproducible local
observations, plus five bounded composition scenarios with compiled Rust
orchestration.

MVP 4 is active under DEC-039. It has established a pinned upstream MIR
interpreter boundary, a standard RV64 LP64 compiler/QEMU-user probe, an
independent comparison of three compact guest-reference candidates, and a
private deterministic trace import for a three-value minimum. None of these
experiments changes the public registry schema, execution-record format or
native reference backend.

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

The AST, dataset, and trace schemas remain non-public until this experiment
shows one representation fits both cases without backend coupling.

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

MVP 4 is active under DEC-039 as a narrow LP64 MIR interpreter and QEMU-user
probe. The `atlas-algorithms` core APIs remain the native reference backend;
MIR remains an adapter and never defines registry semantics, compact references,
or evidence formats. JIT, MIR RISC-V code generation, RV64ILP32 and a fantasy
computer are deferred until separate reproducible experiments justify them.

## MVP 4 execution path

The active work should progress through the following gates. A failed or
inconclusive gate records a limit; it does not silently widen the runtime.

### 1. Select and specify compact guest references

Status: pending class C decision after the completed comparison experiment.

- Choose one representation for the first guest runtime: `u32` offset,
  `u32` handle, or region-plus-offset.
- Define the guest address-space bound, alignment, null/invalid value,
  allocation, lifetime, bounds-checking and error semantics.
- Implement one small memory operation using only that representation, with
  overflow and cross-object or cross-region rejection tests.

Exit evidence: a documented memory model and a reproducible test that never
passes a host pointer as a guest reference.

### 2. Lower one existing algorithm AST to private MIR

Status: pending class C decision on the translation boundary.

- Keep the native algorithm as the correction oracle.
- Reuse an existing typed AST rather than inventing a second semantic model.
- Translate exactly one scalar or bounded-sequence algorithm to private MIR.
- Check result and semantic trace against the native execution on deterministic
  cases.

Exit evidence: one AST-backed algorithm runs through native Rust and MIR with
the same declared semantics. No public backend trait or persistent plan format
is introduced by this gate.

### 3. Exercise guest references in that lowered algorithm

Status: blocked on gate 1.

- Pass compact references as MIR scalar values, never as host pointers.
- Make every load, store, bounds check and imported runtime operation visible
  in the lowered program and trace.
- Test ordinary, boundary, invalid and overflow cases.

Exit evidence: a minimal sequence operation reads or writes guest memory through
the selected model and matches the native reference result.

### 4. Compare interpreter and optional JIT behavior

Status: deferred until gates 2 and 3 produce a meaningful same-plan workload.

- Measure startup latency, code size and correction equivalence separately.
- Keep JIT results local observations with environment and protocol provenance.
- Do not promote latency or throughput results to general claims from one host.

Exit evidence: a reproducible local comparison that justifies retaining or
rejecting the MIR generator for this project.

### 5. Reassess RISC-V code generation and the fantasy-computer profile

Status: explicitly deferred.

- First verify MIR's RISC-V generator with a standalone, documented artifact.
- Keep QEMU user mode as the LP64 Linux ABI probe; do not imply a machine model.
- Define memory, imports, console and clock only if a separate MVP decision
  activates a system-emulation experiment.

Exit evidence: a target-specific probe independent of registry semantics and
the compact-reference model.

## Strategic decisions to prepare

These are intentionally visible before implementation. They are not accepted
decisions and must not be implemented until validated.

### C1. First compact-reference model

Context: all three candidates now have distinct overflow, bounds and identity
tests, but no guest runtime exists. The choice fixes object identity and memory
semantics for the first executable guest experiment.

| Option | Consequence |
|---|---|
| A. `u32` offset in one bounded region | Smallest runtime and lowest call overhead; cannot express separate object identity without extra conventions. |
| B. `u32` handle through an object table | Explicit identity and lifetime checks; adds table indirection and allocator policy. |
| C. region ID plus offset | Clear separation between regions; uses more representation and requires region lifecycle rules. |

Recommendation: **A** for one contiguous, fixed-capacity MVP4 memory region.
It is the narrowest path to prove pointer separation and can be replaced before
any public format exists. The experiment to validate it is one bounds-checked
array operation lowered to MIR. Decision class: **C**.

### C2. Boundary between existing AST and MIR lowering

Context: the current trace proves MIR imports but not that an Atlas algorithm
can use the adapter. A second semantic representation would duplicate authority
and make trace validation weaker.

| Option | Consequence |
|---|---|
| A. Handwritten MIR programs per experiment | Fastest probes; no demonstration that the existing AST can drive a backend. |
| B. Private lowering of one existing typed AST | Tests the intended adapter direction while retaining a narrow, reversible implementation. |
| C. Public generic backend trait and plan format | Reusable on paper, but premature and schema-shaping. |

Recommendation: **B**, beginning with a deliberately small existing AST
fragment. The minimum experiment can remain a trace/import probe; selection of
the first AST-backed algorithm follows its inspected AST shape. Decision class:
**C**, because it defines the first real MIR coupling.

### C3. Status of MIR semantic traces

Context: `MinimumTrace` is intentionally private and cannot be mistaken for
observed registry evidence. A cross-backend trace is useful only once there is
a stable semantic event vocabulary and provenance policy.

| Option | Consequence |
|---|---|
| A. Keep traces private through MVP4 | Lowest risk; correction tests inspect traces only inside adapter tests. |
| B. Define a versioned internal trace artifact | Enables replay tooling, but requires identifiers, provenance and retention rules. |
| C. Add traces to public execution evidence | Makes traces queryable, but changes schema and archival policy. |

Recommendation: **A** until one AST-backed algorithm has matching native and
MIR traces. The smallest discriminating experiment is a one-algorithm trace
comparison with no serialization. Decision class: **C** for B or C; **A**
requires no schema work.

### B1. Interpreter versus JIT evaluation protocol

Context: the interpreter is valuable for observability; MIR generation may be
valuable for startup latency and compact native code. Measuring either before a
same-plan workload would be noise.

Recommendation: retain the interpreter as the correction and trace backend;
prepare a local protocol only after gate 3. The protocol should compare build
latency, generated-code size and execution separately on the same deterministic
dataset. Decision class: **B** until enabling the MIR generator itself, which
becomes class C.

### C4. Post-MVP4 representation for imported algorithms

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
