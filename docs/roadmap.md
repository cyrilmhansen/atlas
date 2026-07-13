# Project roadmap

This roadmap refines the vision without changing MVP scope. Accepted decisions
remain authoritative when this document and a decision record differ.

## Current position

MVP 1 corpus, CLI, schema integrity, evidence integrity, derived indexing, the
local acceptance gate, and the local Git baseline are complete. Remote CI and
publication remain outside the local MVP 1 baseline.

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

MVP 3 is active under DEC-037 as a narrow experiment in explainable constrained
selection and linear composition. Its first slice uses one real sequence
pipeline, internal structural types, and a non-public readable plan. The
structured AST may support readable orchestration but must not expand MVP 3 into
a general compiler.

MVP 4 remains the first phase allowed to introduce MIR. The `atlas-algorithms`
core APIs provide the native reference backend; MIR remains an adapter and never
defines registry semantics, compact references, or evidence formats.
