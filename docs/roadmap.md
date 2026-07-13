# Project roadmap

This roadmap refines the vision without changing MVP scope. Accepted decisions
remain authoritative when this document and a decision record differ.

## Current position

MVP 1, MVP 2 and MVP 3 are closed locally. GitHub CI runs the reproducible MVP
2 acceptance gate, all workspace targets and the RV64 LP64 probe on pushes and
pull requests. The project has a Git-authoritative
YAML registry (10 problems, 15 algorithms and 20 Rust implementations), a
rebuildable SQLite projection, deterministic datasets and reproducible local
observations, plus five bounded composition scenarios with compiled Rust
orchestration.

MVP 4 is active under DEC-039. It has established a pinned upstream MIR
interpreter boundary, a standard RV64 LP64 compiler/QEMU-user probe, an
independent comparison of three compact guest-reference candidates, and a
complete interpreter-only capability ladder over one bounded offset region.
The demonstrated native/MIR pairs now cover partition, `is_sorted`,
minimum/maximum, reverse and stable insertion, in addition to scalar probes.
None changes the public registry schema, execution-record format or native
reference backend. On x86-64, a narrowly configured embedded Capstone now
decodes the observed scalar and guest-memory JIT functions without external
tools; this remains a local diagnostic.

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

MVP 4 is active under DEC-039 as a narrow LP64 MIR interpreter, host-JIT and
QEMU-user probe. The `atlas-algorithms` core APIs remain the native reference
backend; MIR remains an adapter and never defines registry semantics, compact
references, or evidence formats. JIT measurement, MIR RISC-V code generation,
RV64ILP32 and a fantasy computer remain separate experiments.

## MVP 4 execution path

The active work should progress through the following gates. A failed or
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

Status: correction slice complete under DEC-046; measurement protocol pending.

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

Status: interpreter capability complete locally through DEC-045; host-JIT
correction complete under DEC-046.

The checkpoint demonstrates the interpreter and guest-offset boundary across
read-only scans, selection, swaps and shifted writes. It also demonstrates
exact AST trace links for two materially different control-flow shapes. It does
not demonstrate a general AST compiler, measured JIT behavior, MIR-generated
RISC-V, multi-region memory or a persistent backend artifact.

Recommended order for the remaining MVP 4 work:

1. Measure JIT construction latency and generated-code size under a separate,
   local protocol; retain interpreter traces as the observability reference.
2. Probe MIR RISC-V generation with a standalone artifact before connecting it
   to Atlas guest memory.
3. Introduce multiple regions only when an output/scratch algorithm is selected
   and region identity, lifetime and copy visibility have been accepted.

This ordering tests the remaining claims in the vision before widening the
memory model or algorithm corpus.

## Strategic decisions to prepare

These are intentionally visible before implementation. They are not accepted
decisions and must not be implemented until validated.

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
reproduce interpreter and native results without timing. The next work is the
separate class B measurement protocol; MIR RISC-V generation remains outside
this decision.

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
