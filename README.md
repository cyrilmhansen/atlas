# Atlas executable

Atlas executable is an executable registry of problems, algorithms,
implementations, and execution observations. MVP 4 is closed under DEC-052 as a
narrow LP64 MIR adapter experiment. Its interpreter-only, single-region
capability checkpoint is complete through DEC-045. DEC-046 adds a narrow
host-JIT correction path, now including mutating reverse, partition and stable
insertion probes. Exact generated spans and x86-64 instruction shapes are
observable.
DEC-049 validates a standalone scalar MIR-generated RV64 function under QEMU;
DEC-050 adds a private read-only `is_sorted` guest import and DEC-051 adds
checked `reverse` mutation. Latency, executable allocation footprint and
multi-region memory remain separate work.

The public Atlas Explorer is available at
<https://cyrilmhansen.github.io/atlas/>. It is a derived static presentation of
the Git-authoritative registry, not a stable JSON, WASM or URL API.

The authoritative registry now contains 31 problems, 37 algorithms, and 41
implementations: the 10/15/20 MVP 1 sequence baseline plus the first external
graph and dynamic-structure batches, plus the first Phase 4 DFS competitor. The
latter separates union-find, binary
heap and collision-aware hash-table operations and adds tested petgraph,
standard-library and hashbrown boundaries. A streaming batch adds bounded
top-k, online moments, reservoir sampling and Bloom membership. Schema hardening, local evidence
integrity, deterministic indexing, the acceptance gate, and the local Git
baseline are complete.

MVP 5 is closed under DEC-060. Its static Web artifact projects the complete
registry and executes adjacent `is_sorted`, stable insertion sort and symmetric
in-place reverse locally through a private `wasm-bindgen` facade. Its dataset
selector is generated from deterministic DatasetSpec cases rather than UI
fixtures. DEC-058 adds editable seeded generation, operation-growth views and
AST-linked incremental WASM execution for `is_sorted`, insertion sort and
reverse. Analytical traces remain test-only. See `docs/mvp5-web.md` for the
exact authority, counter, dynamics and timing boundaries. Its projection and
WASM interfaces remain private. MVP 5 did not publish the site or activate a
subsequent MVP; the later DEC-072 publishes the derived Phase 3 artifact without
changing that interface boundary.

MVP 6 is closed under DEC-064. Its generated paths compile `sequence.minimum`,
even `sequence.partition`, adjacent `sequence.is_sorted`, stable insertion and
symmetric reverse from their exact reviewed ASTs to private programs executed
by the common WASM visual machine. The retained `is_sorted`, insertion and
reverse steppers are operation-for-operation differential oracles. All three
MVP 5 execution paths have migrated under DEC-062/DEC-063. Consolidation A
removes their imports and dispatch from the browser while retaining the WASM
exports for differential tests. The current bytecode, predicate intrinsic and
presentation shapes are not public contracts.

Phase 2 is closed mixed under DEC-071. K-M7 found foreign knowledge preservation
useful while public qualified selection and generic composition remain
unsupported. DEC-071 activated **Phase 3 - Explorer product trial** and DEC-073
closes it supported. DEC-074 activates **Phase 4 - Comparative foreign
selection** with Knowledge active, Explorer maintained and Execution Lab
frozen. E-M1 is complete with searchable entity detail, exact relation
navigation, sourced
claims and factual same-kind comparison. E-M2 validates the distinction between
knowledge, bounded execution and local observation with the informed-owner
limitation recorded explicitly. DEC-072 completes E-M3 distribution through
the dedicated GitHub Pages workflow. See `docs/phase3-explorer.md`,
`docs/phase3/e-m1-review.md` and
`docs/phase3/phase3-exit-audit.md`, `docs/phase4-comparative-selection.md` and
`docs/phase2/k-m7-phase-audit.md`. K-M4-W repaired the inaccessible
online-moments source with two open primary reports while retaining a neutral
algorithm identity. DEC-067 and DEC-068 accept the subsequent bounded ontology
review without changing schema 0.1: a disposable decision-overlay experiment
and an auditable meaning of `proven`. DEC-070 closes K-M5 mixed: the private
evaluator is retained as evidence, while schema 0.1 and the CLI remain unchanged.

## Project map

- `registry/atlas.yaml`: authoritative aggregate source registry;
- `crates/atlas-algorithms`: minimal-runtime implementations;
- `crates/atlas`: registry model, validation, and reference CLI;
- `crates/atlas-mir`: experimental MIR adapter boundary;
- `crates/atlas-web-wasm`: private curated browser execution facade;
- `web`: static generated-execution workbench and catalog sources;
- `.github/workflows/pages.yml`: verified static Explorer publication;
- `docs/schema-0.1.md`: current public schema contract;
- `docs/mir-integration.md`: exact MVP 4 MIR build and execution boundary;
- `docs/mvp5-web.md`: static artifact build, authority and timing boundaries;
- `docs/mvp6-visual-machine.md`: private generated-program and WASM-machine boundary;
- `docs/mvp6-review.md`: MVP 6 closure audit and reproducibility evidence;
- `docs/audits/2026-07-14-external-project-audit.md`: external post-foundation audit;
- `docs/project-vocabulary.md`: accepted work-program and planning vocabulary;
- `docs/phase2-external-corpus.md`: closed external-corpus and agent-consumer phase;
- `docs/phase2/k-m1-graph-corpus.md`: first external graph batch and model-friction report;
- `docs/phase2/k-m4-dual-import-comparison.md`: independent normalization agreement and divergence gate;
- `docs/phase2/k-m4-w-comparison.md`: open-access source repair for incremental second central moments;
- `docs/phase2/ontology-review.md`: evidence-based ontology alternatives and K-M5 discriminants;
- `docs/phase2/import-protocol-k-m0.3.md`: current source-normalization and evidence protocol;
- `docs/phase2/k-m5-decision-overlay.md`: private overlay experiment and complexity budget;
- `docs/phase2/k-m5-overlay-result.md`: generic evaluator checkpoint, measured cost and remaining gate;
- `docs/phase2/k-m5-independent-authoring-packet.md`: isolated top-k convergence task and adjudication protocol;
- `docs/phase2/k-m5-independent-authoring-result.md`: operational agreement and taxonomic divergence report;
- `docs/phase2/k-m5-normalization-b.md`: bounded encoding equivalences, controls and measured cost;
- `docs/phase2/k-m5-heap-condition-result.md`: conditioned cost/guarantee equivalence falsifier;
- `docs/phase2/k-m5-review.md`: K-M5 exit audit and mixed-closure recommendation;
- `docs/phase2/k-m6-agent-consumer-protocol.md`: frozen assisted/control task and source-reveal protocol;
- `docs/phase2/k-m6-result.md`: supported blind agent-consumer comparison and measured interface cost;
- `docs/phase2/k-m7-phase-audit.md`: mixed Phase 2 synthesis, exit audit and next-phase options;
- `docs/phase3-explorer.md`: closed Explorer product phase, boundaries and milestone plan;
- `docs/phase3/e-m1-review.md`: relational catalog acceptance evidence and limits;
- `docs/phase3/phase3-exit-audit.md`: supported Phase 3 verdict and its evidence limitation;
- `docs/phase4-comparative-selection.md`: active cross-family competitor and selection experiment;
- `docs/phase4/k4-m0-protocol.md`: frozen pre-import requests and schema 0.1 control baseline;
- `docs/phase4/k4-m1-dfs-source-review.md`: first competitor source options and dependency tradeoffs;
- `docs/phase4/k4-m1-result.md`: graph candidate discovery and frozen-request adjudication;
- `docs/phase4/k4-b2-options.md`: executable qualification alternatives after the graph batch;
- `docs/phase4/k4-b2-result.md`: mixed unchanged-evaluator transfer and follow-up options;
- `docs/decisions/DEC-073-close-phase3-supported.md`: owner acceptance and Phase 3 closure boundary;
- `docs/decisions/DEC-074-activate-phase4-comparative-selection.md`: Phase 4 authority and exclusions;
- `docs/decisions/DEC-072-publish-explorer-on-github-pages.md`: public distribution boundary;
- `docs/performance-model-research.md`: non-normative layered performance-model research;
- `docs/vision.md`: authoritative project vision;
- `docs/mvp1-corpus.md`: accepted and completed pilot corpus;
- `docs/mvp1-review.md`: MVP 1 exit-criteria audit;
- `docs/mvp4-review.md`: current single-region MIR checkpoint audit;
- `docs/mvp5-review.md`: static interactive artifact exit audit;
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
scripts/check-web.sh
cargo test -p atlas-mir --locked --offline
sh scripts/check-rv64-lp64-abi.sh
cargo run -p atlas --example dataset_specs
cargo run -p atlas --example semantic_traces
cargo run -p atlas --example pseudocode_ast
cargo run --release -p atlas-bench --example compare_sorts -- sort.merge.rust.slice.v1
scripts/run-benchmark-linux.sh 0
scripts/record-sort-comparison-linux.sh 0
```

Build and serve the local static artifact:

```sh
scripts/build-web.sh
python3 -m http.server 4173 --directory build/web
```

Then open `http://127.0.0.1:4173/`. Generated Web data, JavaScript bindings and
WebAssembly stay under ignored `build/` paths.

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
