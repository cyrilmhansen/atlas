# Atlas executable

Atlas executable is an executable registry of problems, algorithms,
implementations, and execution observations. MVP 4 is active as a narrow LP64
MIR adapter under DEC-039. Its interpreter-only, single-region capability
checkpoint is complete through DEC-045. DEC-046 adds a narrow host-JIT
correction path, now including the first mutating guest-memory probe. Exact
generated spans and x86-64 instruction shapes are observable; latency,
executable allocation footprint, MIR-generated RISC-V and multi-region memory
remain separate work.

The current MVP 1 corpus contains 10 problems, 15 algorithms, and 20 tested Rust
implementations. Schema hardening, local evidence integrity, deterministic
indexing, the acceptance gate, and the local Git baseline are complete.

## Project map

- `registry/atlas.yaml`: authoritative aggregate source registry;
- `crates/atlas-algorithms`: minimal-runtime implementations;
- `crates/atlas`: registry model, validation, and reference CLI;
- `crates/atlas-mir`: experimental MIR adapter boundary;
- `docs/schema-0.1.md`: current public schema contract;
- `docs/mir-integration.md`: exact MVP 4 MIR build and execution boundary;
- `docs/vision.md`: authoritative project vision;
- `docs/mvp1-corpus.md`: accepted and completed pilot corpus;
- `docs/mvp1-review.md`: MVP 1 exit-criteria audit;
- `docs/mvp4-review.md`: current single-region MIR checkpoint audit;
- `docs/roadmap.md`: ordered project roadmap;
- `docs/sqlite-projection-v1.md`: derived index format and digest;
- `docs/decisions`: accepted architectural decisions.

## Use

```sh
cargo run -p atlas -- validate
cargo run -p atlas -- validate registry/atlas.yaml
cargo run -p atlas -- list
cargo run -p atlas -- list algorithm
cargo run -p atlas -- show search.linear
cargo run -p atlas -- search "merge sort"
cargo run -p atlas -- explain search.binary.rust.slice.v1
cargo run -p atlas -- qualify sequence.sort --stable --in-place --allocation none
cargo run -p atlas -- compose cleanup
cargo run -p atlas -- compose cleanup --goal expected-time
cargo run -p atlas -- compose cleanup --rust
cargo run -p atlas -- compose cleanup --goal expected-time --rust
cargo run -p atlas --example cleanup_generated
cargo run -p atlas --example cleanup_expected_time_generated
cargo run -p atlas -- compose find
cargo run -p atlas -- compose find --rust
cargo run -p atlas --example find_generated
cargo run -p atlas -- compose merge-sorted
cargo run -p atlas -- compose merge-sorted --rust
cargo run -p atlas --example merge_sorted_generated
cargo run -p atlas -- compose partition-sort
cargo run -p atlas -- compose partition-sort --rust
cargo run -p atlas --example partition_sort_generated
cargo run -p atlas -- compose unique-sort
cargo run -p atlas -- compose unique-sort --rust
cargo run -p atlas --example unique_sort_generated
cargo run -p atlas -- compose cleanup --forbid filter.in_place.rust.vec.v1
cargo run -p atlas -- index
git submodule update --init --recursive
scripts/apply-mir-patches.sh
cargo test --workspace
scripts/check-mvp1.sh
scripts/check-mvp2.sh
cargo test -p atlas-mir --locked --offline
sh scripts/check-rv64-lp64-abi.sh
cargo run -p atlas --example dataset_specs
cargo run -p atlas --example semantic_traces
cargo run -p atlas --example pseudocode_ast
cargo run --release -p atlas-bench --example compare_sorts -- sort.merge.rust.slice.v1
scripts/run-benchmark-linux.sh 0
scripts/record-sort-comparison-linux.sh 0
```

`scripts/check-mvp1.sh` is the complete offline MVP 1 acceptance gate. It checks
formatting, feature profiles, tests, Clippy, registry evidence, and deterministic
index reconstruction.

The `dataset_specs` example materializes the first experimental MVP 2 sorting
and partitioning datasets with their class, seed, size, and content digest.
The `semantic_traces` example executes two small demonstrations and prints every
typed semantic event and invariant checkpoint.
The `pseudocode_ast` example renders the two experimental backend-independent
algorithm descriptions.
The release-only `compare_sorts` example emits raw-context empirical summaries;
its output is not a persistent benchmark conclusion.

Execution observations are regenerable products outside the registry. The
first deterministic correction recipe writes an ignored experimental record:

```sh
cargo run -q -p atlas --locked --offline --example record_sort_correction
cargo run -q -p atlas --locked --offline --example record_partition_correction
cargo run --release -q -p atlas-bench --locked --offline --example record_sort_benchmark -- sort.insertion.rust.slice.v1
```

The outputs are written under `build/executions/`; deleting them does not remove
Git-authoritative knowledge.

Replay a locally retained observation by its content-derived ID:

```sh
cargo run -q -p atlas -- replay execution.sha256.EXAMPLE
cargo run -q -p atlas -- replay execution.sha256.BENCHMARK --cpu 4
cargo run -q -p atlas -- compare execution.sha256.FIRST execution.sha256.SECOND
```

A replay regenerates the recipe under the current environment. A benchmark may
therefore be rejected by its quality gate even when the original observation was
qualified.

`compare` writes a generated YAML report under `build/reports/` only when every
input observation is qualified and uses the exact same dataset, context, and
requested protocol from a clean worktree.

The benchmark recipe writes an observation only after the harness accepts every
quality check. Run it through the Linux pinning wrapper's CPU policy when a
measurement is needed.
The Linux-only wrapper requires an explicit CPU, performs a non-invasive
preflight, and pins only the benchmark process without changing the governor.

To capture the single bounded sorting campaign accepted by DEC-035, start from
a clean worktree and run `scripts/record-sort-comparison-linux.sh CPU`. It
records each registered sorting implementation once, stops on the first rejected
quality gate, and invokes `atlas compare` only after all three records exist.
It deliberately never retries a measurement.

The YAML files committed to Git are authoritative. The validator loads the
aggregate registry in memory and checks its schema, cross-references, local
files, implementation evidence, and Rust test symbols.

`qualify` is intentionally narrow: it filters recorded properties and prints
their evidence; it does not rank implementations or infer missing metadata.

`compose cleanup` is the first MVP 3 experiment. It renders one internal,
non-persistent `filter -> sort -> deduplicate` plan with a selected candidate,
a rejected alternative, and all visible mutations, copies, and allocations.
`compose cleanup --rust` renders the corresponding Rust orchestration; the
identical `cleanup_generated` example is compiled and runnable. This remains a
single-scenario generator, not a general planner or a persistent plan format.
`--goal expected-time` instead selects from declared complexity claims and
explains the allocation-heavier alternative; it does not use benchmark results.
Its `--rust` variant is separately compiled as `cleanup_expected_time_generated`.
`compose find` demonstrates a produced precondition: it sorts before binary
search, makes that dependency visible, and renders a verified Rust example under
`--rust`.
`compose partition-sort` keeps both partition branches, projects and sorts only
`matching`, then reassembles the structured result without a hidden copy.
`compose unique-sort` makes the required unique output allocation distinct from
intermediate storage: insertion sort mutates the supplied sequence, then
quadratic deduplication produces the output. The merge/hash alternative is
rendered as rejected for its declared scratch and hash-set storage.
`compose merge-sorted` makes two sorted-input preconditions explicit: it sorts
each input in place, then produces the required stable merged output.

All composition scenarios accept `--force IMPLEMENTATION_ID` or `--forbid
IMPLEMENTATION_ID`. These constraints select only between the reviewed
candidates and never modify the registry. Generated Rust is intentionally
unavailable with an override until that exact constrained source is verified.

## Runtime boundary

`atlas-algorithms` is `#![no_std]`. Core-only algorithms compile without default
features; collection-producing implementations use the optional `alloc` feature.
The `atlas` crate contains the YAML registry and CLI and therefore uses `std`.

```sh
cargo check -p atlas-algorithms --no-default-features
cargo test -p atlas-algorithms --no-default-features
cargo test -p atlas-algorithms --no-default-features --features alloc
cargo test -p atlas-algorithms --no-default-features --features hash-dedup
cargo test -p atlas-algorithms --features alloc
```

The optional `hash-dedup` feature uses `hashbrown` with a non-cryptographic
default hasher. It is intended for local algorithm workloads, not hostile input
requiring HashDoS resistance.

The derived SQLite index uses the system SQLite library. Install SQLite headers
and linker metadata. On systems where `sqlite3.pc` is outside the configured
search path, set `PKG_CONFIG_PATH` for Cargo, for example:

```sh
PKG_CONFIG_PATH=/usr/lib/pkgconfig cargo check -p atlas
```

`atlas index [DB_PATH]` defaults to `build/atlas.sqlite3`, which is ignored by
Git. The command prints deterministic logical row counts and a SHA-256 digest.

See [the MVP status](docs/mvp-status.md), [the MVP 2 closure
scope](docs/decisions/DEC-036-close-mvp2-scope.md), [the MVP 3
activation](docs/decisions/DEC-037-activate-mvp3.md), [the MVP 3 closure
scope](docs/decisions/DEC-038-close-mvp3-scope.md), [schema
0.1](docs/schema-0.1.md), and [accepted decisions](docs/decisions/). The [MVP
1 review](docs/mvp1-review.md), [MVP 3 review](docs/mvp3-review.md), [MVP 4
review](docs/mvp4-review.md), [MIR integration](docs/mir-integration.md), and
[roadmap](docs/roadmap.md) describe the next decisions.
