# Progress log

## 2026-07-11 - MVP 1 activated

- Accepted the aggregate YAML schema direction.
- Accepted a Rust workspace using Serde and a YAML parser.
- Deferred SQLite and kept Cap'n Proto as an uncommitted future option.
- Selected `atlas validate [PATH]` as the first CLI surface.

## 2026-07-11 - First vertical slice

### Result

- Added aggregate schema 0.1 documentation and a one-component registry.
- Added a stable top-down merge sort with explicit `O(n)` auxiliary allocation.
- Added global validation for versions, IDs, references, claims, and provenance.
- Added `atlas validate [PATH]` with actionable errors.

### Verification

```sh
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -q -p atlas -- validate
cargo run -q -p atlas -- validate registry/atlas.yaml
```

The slice has nine passing tests: three algorithm tests and six registry/CLI
acceptance tests.

### Limits

- The corpus contains 1 of 10 problems, 1 of 15 algorithms, and 1 of 20
  implementations targeted by MVP 1.
- Execution records are intentionally rejected until a real case defines their
  fields.
- SQLite and the `list`, `show`, `search`, and `explain` commands are not yet
  implemented.

## 2026-07-11 - Second sorting strategy and listing

### Result

- Added a stable, in-place insertion sort with no allocation.
- Added its algorithm and implementation claims to the aggregate registry.
- Confirmed that a second component requires no registry-validator change.
- Added `atlas list [problem|algorithm|implementation]` with tabular,
  deterministic output.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo run -q -p atlas -- list
cargo run -q -p atlas -- list algorithm
```

The slice has fifteen passing tests: six algorithm tests and nine registry/CLI
acceptance tests.

### Limits

- `list` uses the default registry path and has no search or formatting options.
- The corpus still covers only the `sequence.sort` problem.
- The algorithm schema has not yet been exercised against a second problem
  family.

## 2026-07-11 - Second problem family

### Result

- Made the sort-specific `stable` and `in_place` claims optional under DEC-006.
- Defined `sequence.search`, `search.linear`, and its Rust slice implementation.
- Added deterministic tests for first-match, absent, and empty-input behavior.
- Exercised schema 0.1 against sorting and searching without an untyped property
  map.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- list
```

The slice has eighteen passing tests: nine algorithm tests and nine registry/CLI
acceptance tests.

### Limits

- The corpus only exercises scalar slices and equality/order callbacks.
- Claim sources are checked for presence but not yet resolved to test symbols or
  files.
- `show`, `search`, and `explain` remain unimplemented.

## 2026-07-11 - Entity inspection

### Result

- Added `atlas show <id>` for globally unique problem, algorithm, and
  implementation IDs.
- Exposed relationships, claim values, evidence levels, and provenance.
- Expanded list values and implementation effects so mutations, I/O, blocking,
  and allocation remain visible.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- show search.linear
cargo run -q -p atlas --locked --offline -- show search.linear.rust.slice.v1
```

The slice has twenty-three passing tests: nine algorithm tests and fourteen
registry/CLI acceptance tests.

### Limits

- `show` reads the default registry path only.
- Its human-readable output is explicitly not a stable serialization protocol.
- Claim sources are displayed but not resolved or checked against repository
  artifacts.

## 2026-07-11 - Qualified algorithm requirements

### Result

- Added the optional, qualified `Algorithm.requires` list under DEC-008.
- Added lower-bound binary search returning the first matching position.
- Declared and displayed its sorted-input requirement without changing the
  `sequence.search` problem contract.
- Rejected present-but-empty requirements and missing requirement provenance.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- show search.binary.lower_bound
```

The slice has twenty-nine passing tests: twelve algorithm tests and seventeen
registry/CLI acceptance tests.

### Limits

- Requirements are human-readable strings, not executable predicates.
- The validator checks requirement structure and provenance, not whether input
  data satisfies a requirement.
- Search selection based on requirements is outside the current slice.

## 2026-07-11 - Text discovery

### Result

- Added `atlas search <term>` under DEC-009.
- Matched IDs and declared algorithm names without case sensitivity.
- Reused the deterministic `kind<TAB>id` listing format.
- Defined no matches as success with empty output and rejected empty terms.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- search binary
cargo run -q -p atlas --locked --offline -- search "TOP-DOWN MERGE SORT"
```

The slice has thirty-three passing tests: twelve algorithm tests and twenty-one
registry/CLI acceptance tests.

### Limits

- Problems and implementations have no declared display-name claim in schema
  0.1, so they are currently searchable by ID only.
- Search does not inspect contracts, evidence sources, or effects.
- This command does not rank or select algorithms from constraints.

## 2026-07-11 - Semantic chain explanation

### Result

- Added `atlas explain <implementation-id>` under DEC-010.
- Resolved and displayed the implementation, algorithm, and problem chain.
- Included complete qualified details, notably effects and requirements.
- Kept relationship explanation separate from any future selection rationale.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- explain search.binary.rust.slice.v1
```

The slice has thirty-seven passing tests: twelve algorithm tests and twenty-five
registry/CLI acceptance tests.

### Limits

- `explain` accepts implementation IDs only.
- The command explains stored relations and evidence, not why one implementation
  should be selected over another.
- It reads the default registry path and has one human-readable output format.

## 2026-07-11 - Minimum selection

### Result

- Added `sequence.minimum` and a linear minimum-scan algorithm.
- Added a zero-allocation Rust implementation returning `Option<&T>`.
- Defined deterministic tie behavior: the first equivalent minimum is retained.
- Added the third problem family without changing schema 0.1 or the validator.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain select.minimum.linear.rust.slice.v1
```

The slice has forty-one passing tests: fifteen algorithm tests and twenty-six
registry/CLI acceptance tests.

### Limits

- The minimum implementation returns a borrowed element and therefore does not
  cover consuming iterators or owned outputs.
- Only total-order comparison callbacks are represented.
- The corpus has no partitioning, filtering, or transformation problem yet.

## 2026-07-11 - Stable filtering and output policies

### Result

- Recorded the complete MVP 1 corpus and implementation-counting policy.
- Added `sequence.filter` with stable copying and in-place compaction algorithms.
- Added allocated-output, caller-output, and in-place Rust implementations.
- Made output allocation and mutation differences visible in effects and
  `explain` output.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain filter.copy_into.rust.vec.v1
```

The slice has forty-eight passing tests: twenty-one algorithm tests and
twenty-seven registry/CLI acceptance tests.

### Limits

- `filter_copy_into` may allocate when the caller-provided capacity is
  insufficient; the effect declares this explicitly.
- Predicates are assumed deterministic by the registered algorithms but the
  runtime cannot inspect callback behavior.
- The schema does not distinguish output storage from auxiliary storage as
  separate structured cost fields.

## 2026-07-11 - Stable and in-place partitioning

### Result

- Added `sequence.partition` with stable-copy and unstable in-place algorithms.
- Added allocated-output, two-caller-buffer, and in-place Rust implementations.
- Returned an explicit boundary from the in-place implementation.
- Demonstrated non-stability with a deterministic counterexample while checking
  membership and permutation guarantees.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain partition.copy_into.rust.vec.v1
cargo run -q -p atlas --locked --offline -- show partition.two_pointer.in_place
```

The slice has fifty-five passing tests: twenty-seven algorithm tests and
twenty-eight registry/CLI acceptance tests.

### Limits

- The abstract partition result has two concrete representations: two output
  vectors or a boundary in a mutated sequence.
- Callback determinism remains a declared assumption rather than an enforceable
  property.
- The in-place algorithm intentionally provides no relative-order guarantee.

## 2026-07-11 - Maximum selection

### Result

- Added `sequence.maximum` and a linear maximum-scan algorithm.
- Added a zero-allocation Rust implementation returning `Option<&T>`.
- Defined deterministic tie behavior by retaining the first equivalent maximum.
- Kept minimum and maximum implementations explicit instead of adding an
  unnecessary generic extremum abstraction.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain select.maximum.linear.rust.slice.v1
```

The slice has fifty-nine passing tests: thirty algorithm tests and twenty-nine
registry/CLI acceptance tests.

### Limits

- Like minimum, maximum only covers borrowed slice elements and total-order
  callbacks.
- The two extremum scans intentionally duplicate a small explicit loop.

## 2026-07-11 - In-place reversal

### Result

- Added `sequence.reverse` and symmetric in-place reversal.
- Used explicit swaps over half the slice with no allocation.
- Tested odd, even, empty, and singleton lengths plus the involution property.
- Exposed input mutation and constant auxiliary memory through `explain`.

### Verification

```sh
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain reverse.symmetric.rust.slice.v1
```

The slice has sixty-three passing tests: thirty-three algorithm tests and thirty
registry/CLI acceptance tests.

### Limits

- Reversal is represented only as an in-place slice operation.
- No lazy reversed view or copying output variant is included in the accepted
  MVP 1 corpus.

## 2026-07-11 - Minimal-runtime crate boundary

### Result

- Moved all implementation modules to dependency-free `atlas-algorithms`.
- Declared the crate `#![no_std]` and replaced comparison imports with `core`.
- Gated `Vec`-based modules and APIs behind the default `alloc` feature.
- Kept registry parsing, filesystem access, process handling, and CLI rendering
  in the `std`-based `atlas` crate.
- Updated every registered source path and entrypoint.
- Recorded deferred decisions for a structured algorithm AST, separate dataset
  specifications, and minimal semantic traces.

### Verification

```sh
cargo check -p atlas-algorithms --no-default-features
cargo test -p atlas-algorithms --no-default-features --locked --offline
cargo test -p atlas-algorithms --features alloc --locked --offline
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
```

The core-only profile has twenty-one passing tests. The default workspace suite
still has sixty-three passing tests: thirty-three algorithm tests and thirty
registry/CLI acceptance tests.

### Limits

- `alloc` is enabled by default in workspace builds.
- No allocator implementation is selected or supplied by the algorithm crate.
- The AST, dataset, and trace decisions define direction only; their public
  schemas remain intentionally absent from MVP 1.

## 2026-07-11 - Stable merge of sorted inputs

### Result

- Added qualified requirements to problem contracts under DEC-018.
- Added `sequence.merge_sorted` with two sorted-input requirements.
- Added a stable two-way merge with allocated and caller-provided outputs.
- Kept the implementation in the `alloc` profile while preserving the core-only
  build.

### Verification

```sh
cargo check -p atlas-algorithms --no-default-features
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain merge.sorted_into.rust.vec.v1
```

The slice has seventy passing workspace tests: thirty-seven algorithm tests and
thirty-three registry/CLI acceptance tests. The core-only profile retains its
twenty-one tests.

### Limits

- Both inputs must already satisfy the problem's sortedness requirements.
- The caller-output variant may allocate when capacity is insufficient.
- Stability chooses left-input elements before equal right-input elements.

## 2026-07-11 - Adjacent sortedness validation

### Result

- Added `sequence.is_sorted` and an adjacent-pair scan.
- Accepted equal neighbors and stopped immediately at the first inversion.
- Added a core-only, allocation-free implementation.
- Exposed the boolean contract and absence of effects through `explain`.

### Verification

```sh
cargo test -p atlas-algorithms --no-default-features --locked --offline
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
cargo run -q -p atlas --locked --offline -- explain order.is_sorted.rust.slice.v1
```

The slice has seventy-five passing workspace tests: forty-one algorithm tests
and thirty-four registry/CLI acceptance tests. The core-only profile has
twenty-five passing tests.

### Limits

- The callback is declared to represent a total order but this cannot be checked
  dynamically.
- Early exit makes actual comparisons data-dependent; no observation count is
  stored until the execution model is introduced.

## 2026-07-11 - Stable deduplication

### Result

- Added `sequence.deduplicate` with stable quadratic and hash-based algorithms.
- Added allocated quadratic, allocated hash, and caller-output hash
  implementations.
- Added optional `hashbrown` support through the `hash-dedup` feature while
  preserving core-only and alloc-only profiles.
- Implemented the qualified `time_expected` claim separately from worst-case
  time.
- Reached the MVP 1 targets of ten problems and fifteen algorithms.

### Verification

```sh
cargo test -p atlas-algorithms --no-default-features --locked --offline
cargo test -p atlas-algorithms --no-default-features --features alloc --locked --offline
cargo test -p atlas-algorithms --no-default-features --features hash-dedup --locked --offline
cargo test --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- explain deduplicate.hash_into.rust.vec.v1
```

The core profile has twenty-five tests, alloc-only has forty-three, hash-enabled
has forty-six, and the workspace has eighty-two tests including thirty-six
registry/CLI acceptance tests.

### Limits

- Expected linear time assumes expected constant-time hash-table operations;
  worst-case time remains quadratic.
- The default hashbrown hasher is not HashDoS-resistant and is not approved for
  hostile input.
- Hash implementations always allocate an internal set for non-empty inputs.

## 2026-07-11 - Corpus completion and project review

### Result

- Added caller-scratch merge sort as the twentieth implementation.
- Returned a structured size error before any input mutation.
- Made the scratch implementation available in the core-only profile.
- Completed the accepted 10-problem, 15-algorithm, 20-implementation corpus.
- Replaced first-slice documentation with a current project map, MVP 1 exit
  review, and ordered roadmap.

### Verification

```sh
cargo test -p atlas-algorithms --no-default-features --locked --offline
cargo test -p atlas-algorithms --no-default-features --features alloc --locked --offline
cargo test -p atlas-algorithms --no-default-features --features hash-dedup --locked --offline
cargo test --workspace --locked --offline
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- validate
```

The core profile has twenty-eight tests, alloc-only has forty-six, hash-enabled
has forty-nine, and the workspace has eighty-six tests including thirty-seven
registry/CLI acceptance tests.

### Limits

- Corpus completion does not close MVP 1: SQLite, deterministic rebuild,
  evidence resolution, coverage reporting, schema version rules, Git baseline,
  and CI remain open.
- The vision source is still binary DOCX rather than maintained Markdown.
- Implementation license, version, target, dependency, and ABI fields are not
  yet represented in schema 0.1.

## 2026-07-12 - Vision, schema metadata, and deterministic index

### Result

- Made Markdown the authoritative vision source while retaining the DOCX
  snapshot.
- Added mandatory version, license, target, dependency, and ABI claims to all
  twenty implementations.
- Documented schema compatibility and migration rules.
- Added system-linked `rusqlite`, normalized projection version 1, and canonical
  SHA-256 logical digests.
- Added `atlas index [DB_PATH]` with an ignored default database path.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
cargo fmt --all -- --check
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline -- index
```

The workspace has ninety-three tests: forty-nine algorithm tests, three index
unit tests, and forty-one registry/CLI integration tests. The current projection
contains 45 entities, 35 relations, and 313 claims.

### Limits

- System SQLite discovery currently requires
  `PKG_CONFIG_PATH=/usr/lib/pkgconfig` on this host.
- Local provenance references are still syntactic strings rather than resolved
  evidence links.
- The repository still has no initial Git commit or CI gate.

## 2026-07-12 - Local evidence resolution

### Result

- Formalized the restricted schema 0.1 evidence grammar under DEC-026.
- Resolved workspace-relative files, implementation IDs, and Rust test symbols.
- Kept documentary sources offline and explicitly syntax-only.
- Added actionable rejection tests for missing files, unknown implementations,
  stale tests, and unsupported schemes.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
cargo clippy -p atlas-algorithms --no-default-features --all-targets --locked --offline -- -D warnings
cargo clippy -p atlas-algorithms --no-default-features --features alloc --all-targets --locked --offline -- -D warnings
cargo clippy -p atlas-algorithms --all-features --all-targets --locked --offline -- -D warnings
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-targets --locked --offline -- -D warnings
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo run -q -p atlas --locked --offline -- validate
```

The workspace has ninety-eight tests: forty-nine algorithm tests, three index
unit tests, and forty-six registry/CLI integration tests.

### Limits

- Test-symbol lookup intentionally targets the MVP 1 `atlas-algorithms` module
  layout and is not a general Rust parser.
- Documentary evidence is not fetched or semantically verified.
- The repository still has no single acceptance command, initial commit, or CI
  gate.

## 2026-07-12 - Unified MVP 1 acceptance gate

### Result

- Added `scripts/check-mvp1.sh` as the single offline acceptance command.
- Covered formatting, core/alloc/hash profiles, workspace tests, Clippy,
  evidence validation, and deterministic SQLite reconstruction.
- Kept orchestration outside the Atlas domain CLI and added no dependency.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig scripts/check-mvp1.sh
```

### Limits

- The host must provide the Rust toolchain and system SQLite development
  metadata before running the gate.
- CI integration remains open.

## 2026-07-12 - Local MVP 1 Git baseline

### Result

- Established the human-approved local root commit for the complete MVP 1.
- Kept generated `target/` and `build/` artifacts outside Git.
- Performed no remote configuration, push, publication, or CI platform choice.

### Verification

```sh
git status --short
git log -1 --oneline
```

### Limits

- CI and any remote publication require separate decisions.

## 2026-07-12 - MVP 2 activation and first dataset specifications

### Result

- Closed MVP 1 at the human-approved local baseline and activated MVP 2 under
  DEC-028.
- Added separate experimental specifications for sorting and partitioning.
- Covered typical, boundary, degenerate, adversarial, and regression cases.
- Made generated instances carry parameters, seed, values, predicate, and a
  canonical content digest without changing schema 0.1.
- Added a Cargo example that materializes and identifies all ten instances.

### Limits

- Dataset types are experimental Rust structures, not a persistent public
  format.
- No benchmark or execution observation is recorded in this slice.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig scripts/check-mvp1.sh
cargo run -q -p atlas --locked --offline --example dataset_specs
```

The workspace has 103 tests: 49 algorithm tests, 8 Atlas unit tests, and 46
registry/CLI integration tests. The example materializes ten deterministic
instances across the two problem specifications.

## 2026-07-12 - Minimal semantic trace experiment

### Result

- Added a common typed event vocabulary for reads, writes, comparisons, swaps,
  recursion, allocation, copies, partition boundaries, predicates, and asserts.
- Traced caller-scratch merge sort and in-place two-pointer partition without
  modifying the native `no_std` implementations.
- Bound every trace to algorithm, implementation, dataset case, and content
  digest, and checked results against the native implementation.
- Added a human-readable demonstration example.

### Limits

- Trace version `experimental-0` is an in-memory Rust model, not a persistent
  protocol or benchmark record.
- The demonstrators currently operate on `i32` dataset instances.
- The structured pseudocode AST remains unimplemented.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example semantic_traces
```

The workspace has 105 tests. The selected demonstrations produce 109 merge-sort
events and 76 partition events with every invariant checkpoint passing.

## 2026-07-12 - Experimental structured pseudocode AST

### Result

- Modeled merge sort and in-place partition with the same nested statement AST.
- Made parameter modes, mutations, allocations, copies, semantic operations,
  and invariant checkpoints explicit.
- Added deterministic backend-independent rendering and unique node validation.
- Verified that every emitted semantic trace event kind is represented by the
  corresponding AST.

### Limits

- Expressions are readable strings rather than a typed expression sublanguage.
- The AST is an in-memory experiment with no persistent serialization or MIR
  mapping.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example pseudocode_ast
```

The workspace has 108 tests. Both ASTs validate, render deterministically, and
cover every semantic operation kind emitted by their demonstration traces.

## 2026-07-12 - Exact trace-to-AST linkage

### Result

- Replaced aggregate operation coverage with an exact AST node ID on every
  trace step.
- Added lookup and validation of node existence and semantic operation type.
- Added explicit invocation and permutation-assertion nodes where the previous
  global coverage had hidden missing correspondences.
- Made the trace example render `ast_node_id -> event` for every step.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example semantic_traces
```

The workspace has 109 tests. Valid traces resolve all 185 demonstrated steps;
targeted tests reject both an unknown node and an operation-kind mismatch.

### Limits

- Node IDs and trace steps remain experimental in-memory structures rather than
  a persistent protocol.

## 2026-07-12 - Minimal typed AST expressions

### Result

- Replaced structural expression strings with typed variables, constants,
  lengths, indexes, ranges, operators, boolean logic, and abstract calls.
- Added scope, operand, condition, binding, and parameter access-mode checks.
- Made recursive subranges and all read/write operands structurally inspectable.
- Preserved backend-independent pseudocode rendering.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo test --workspace --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example pseudocode_ast
```

The workspace has 112 tests. Negative tests reject unknown variables, invalid
operand types, and writes through a read-only parameter.

### Limits

- Abstract calls carry an explicit result type but do not yet resolve against a
  typed signature environment.
- The expression tree is not an evaluator or persistent language.

## 2026-07-12 - Minimal empirical sorting harness

### Result

- Added a separate `atlas-bench` adapter crate and a deterministic 2,048-element
  sorting dataset.
- Separated correction validation, preparation, warmup, and timed samples.
- Captured raw samples, robust dispersion, complete local context, parameters,
  seed, and dataset digest.
- Rejected comparisons across different contexts, datasets, or settings.
- Demonstrated release measurement while retaining all raw samples; the local
  run identified its dirty worktree and is therefore not a reference result.

### Limits

- Results are ephemeral and do not populate public `Execution` records.
- The harness supports only the three current sorting implementations.
- No measured result is promoted to a general performance claim.

### Verification

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig scripts/check-mvp1.sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo run --release -q -p atlas-bench --locked --offline --example compare_sorts
```

The workspace has 116 tests. The demonstration records 21 raw samples for each
of three implementations on the same 2,048-element dataset and context. Its
numbers are intentionally not recorded as an Atlas observation because the
captured worktree is dirty.

The first clean-context attempt was also rejected as a candidate observation:
the caller-scratch sample series drifted substantially during measurement. The
harness now reports dispersion and half-series drift automatically rather than
leaving this anomaly to manual inspection.

## 2026-07-12 - Adaptive and interleaved benchmark protocol

### Result

- Replaced fixed warmup in the comparison example with a bounded adaptive gate.
- Required three consecutive stable-window checks for every implementation.
- Rotated implementation order deterministically on every warmup and measured
  round.
- Refused to emit measured results when warmup does not stabilize.
- Added a suite-level rejection when any measured series fails dispersion or
  drift checks.
- Added extreme-sample and execution-position bias checks after a pinned run
  exposed cases missed by MAD and half-series medians.

### Limits

- Stability uses a documented 5% median-window tolerance; it is a quality gate,
  not a statistical proof of stationarity.
- CPU affinity, frequency control, and process isolation remain outside the
  current portable harness.

### Verification

```sh
cargo test -p atlas-bench --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo run --release -q -p atlas-bench --locked --offline --example compare_sorts
```

The workspace has 118 tests. On the current host, adaptive warmup completed but
the subsequent measured series still failed dispersion or drift checks. The run
is correctly rejected rather than promoted to an observation candidate.

## 2026-07-12 - Non-invasive benchmark environment diagnostics

### Result

- Captured load averages, effective CPU affinity, context switches, scheduler
  migrations, governors, and visible frequency range before and after a suite.
- Added counter deltas and included both snapshots in warmup-failure errors.
- Kept Linux diagnostics optional and performed no system-state modification.

### Limits

- Frequencies are boundary snapshots, not continuous telemetry.
- Diagnostics can explain correlations but do not prove the cause of an
  unstable run.

### Verification

```sh
cargo test -p atlas-bench --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy -p atlas-bench --all-targets --locked --offline -- -D warnings
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo run --release -q -p atlas-bench --locked --offline --example compare_sorts
```

The workspace has 119 tests. The current diagnostic run was rejected during
adaptive warmup after 100 rounds. It observed CPUs `0-23`, the `powersave`
governor, a roughly 0.6-3.6 GHz boundary frequency range, moderate system load,
and additional involuntary context switches. These are context observations,
not a causal performance conclusion.

## 2026-07-12 - Calibrated benchmark batches

### Result

- Calibrated invocations per sample toward 10 ms independently for each sort.
- Prepared all independent input and scratch buffers before starting the clock.
- Retained total batch durations, invocation counts, and normalized durations.
- Bounded prepared batch memory to 64 MiB and added minor/major page-fault
  diagnostics.
- Added up to two post-stability recalibrations so cold calibration cannot leave
  measured batches materially below the target duration.
- Reused prepared pools across samples after diagnostics exposed excessive minor
  page faults caused by recreating them for every batch.

### Limits

- Normalized durations are integer nanoseconds per invocation.
- Allocation internal to the allocating merge sort remains intentionally timed.

### Verification

```sh
cargo test -p atlas-bench --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy -p atlas-bench --all-targets --locked --offline -- -D warnings
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo run --release -q -p atlas-bench --locked --offline --example compare_sorts
```

The workspace has 121 tests. Reusing pools reduced minor page faults from about
153,000 to 563 in the diagnostic run. The suite still failed adaptive warmup
while observing 66 scheduler migrations, 116 additional involuntary context
switches, and a roughly 0.6-5.1 GHz boundary frequency range. This isolates the
former harness allocation churn from the remaining scheduler/frequency effects;
it does not by itself prove which remaining effect dominates.

## 2026-07-12 - Linux pinned benchmark runner

### Result

- Added an explicit-CPU Linux runner around `taskset`.
- Built before pinning and checked CPU availability, system load, governor, and
  maximum frequency without modifying system state.
- Kept the Linux policy outside the portable benchmark and knowledge crates.
- Restricted frequency diagnostics to the process's effective CPU set and added
  recent warmup windows, recalibration count, and batch sizes to failures.

### Limits

- CPU selection remains a human experiment parameter because this machine has
  core groups with different maximum frequencies.
- Pinning prevents migration but does not prevent preemption or frequency
  scaling on the selected CPU.

### Verification

```sh
sh -n scripts/run-benchmark-linux.sh
scripts/run-benchmark-linux.sh 0
scripts/run-benchmark-linux.sh 1
scripts/run-benchmark-linux.sh 4
```

The workspace has 123 tests. CPU pinning reduced scheduler migrations to zero.
CPU 0 experienced heavy preemption; CPU 1 improved but did not stabilize. CPU 4
reached stable warmup with low preemption, then exposed an execution-position
bias for both merge variants. Insertion sort passed all measured-series checks.
No cross-CPU timing comparison is inferred from these runs.

## 2026-07-12 - Per-implementation process isolation

### Result

- Changed the release example to accept exactly one registered sorting
  implementation ID.
- Made the Linux runner launch three fresh pinned processes after one build.
- Preserved independent calibration, warmup, diagnostics, and quality verdicts
  and combined only process exit statuses.

### Limits

- The wrapper does not rank or aggregate results across processes.
- Structured cross-process observations require a separate persistent-format
  decision; human-readable output is not parsed as a protocol.
- Requested adaptive settings are separated from observed convergence counts so
  independently isolated runs are not marked incomparable merely because their
  warmup lengths differ.

### Verification

```sh
cargo test -p atlas-bench --locked --offline
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
scripts/run-benchmark-linux.sh 4
```

The workspace has 125 tests. On CPU 4, all three isolated processes passed
warmup and measured-series quality checks with zero scheduler migrations and
3-7 involuntary context switches each. Batches remained near 10 ms and the
former execution-position bias disappeared. The run is diagnostic rather than
a candidate observation because the captured worktree is dirty.

## 2026-07-12 - Deterministic sorting dataset matrix

### Result

- Replaced the benchmark's single generated input with 12 explicit dataset
  cases spanning lengths 64, 2,048, and 8,192.
- Covered uniform, ascending, descending, and high-duplication distributions.
- Kept the previous benchmark campaign on its explicit
  `sort.benchmark.uniform.2048` case instead of relying on array position.
- Exposed the matrix through the existing `dataset_specs` example.

### Limits

- The benchmark still executes one reference dataset per process; selecting or
  recording a full campaign belongs to the future execution-record slice.
- These distributions are an initial sorting matrix, not a claim of exhaustive
  input coverage.

### Verification

```sh
cargo test --workspace --locked --offline
cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example dataset_specs
```

The workspace has 127 tests. Dataset generation is deterministic, case IDs and
content digests are unique, and the expected sizes and distribution invariants
are checked without running a timing campaign.

## 2026-07-12 - Deterministic partition dataset matrix

### Result

- Added 12 partition cases spanning lengths 64, 2,048, and 8,192.
- Covered uniform even selection, alternating values, fully selected inputs,
  and fully rejected inputs.
- Verified each generated instance against the native in-place partition
  contract, including output separation and preservation of the input multiset.
- Exposed the matrix through the existing `dataset_specs` example.

### Limits

- The matrix qualifies generated data and correction behavior only; it is not
  connected to benchmark execution or persistent execution records.
- Selectivity is represented by four concrete profiles rather than a general
  parameterized campaign language.

### Verification

```sh
cargo test --workspace --locked --offline
cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example dataset_specs
```

The workspace has 130 tests. Partition generation is deterministic, IDs and
digests are unique, and all 12 instances satisfy the partition contract.

## 2026-07-12 - First regenerable correction execution

### Result

- Recorded DEC-033: executions are generated products under ignored `build/`,
  while their Rust recipes, datasets, and scripts remain versioned.
- Added a backend-independent experimental execution record with a
  content-derived identity and integrity validation.
- Added the versioned `sort.insertion.uniform.64.correction.v1` recipe. It
  verifies the registered implementation and dataset before writing YAML.
- Captured commit and dirty state, compiler, target, parameters, dataset seed
  and digest, correction result, output digest, and provenance.

### Limits

- The generated YAML is deliberately internal and is not schema 0.1.
- The first recipe covers correction only. Benchmark records remain deferred
  until deletion and regeneration have been demonstrated for this boundary.
- Environment or source-state changes legitimately change the execution ID.

### Verification

```sh
cargo test --workspace --locked --offline
cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
cargo run -q -p atlas --locked --offline --example record_sort_correction
```

The workspace has 132 tests. Repeated generation in the same repository state
produces byte-identical YAML under `build/executions/`, and modifying an
observation invalidates its content-derived identity.
