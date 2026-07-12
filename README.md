# Atlas executable

Atlas executable is an executable registry of problems, algorithms,
implementations, and execution observations. MVP 1 is active.

The current MVP 1 corpus contains 10 problems, 15 algorithms, and 20 tested Rust
implementations. Schema hardening, local evidence integrity, deterministic
indexing, the acceptance gate, and the local Git baseline are complete.

## Project map

- `registry/atlas.yaml`: authoritative aggregate source registry;
- `crates/atlas-algorithms`: minimal-runtime implementations;
- `crates/atlas`: registry model, validation, and reference CLI;
- `docs/schema-0.1.md`: current public schema contract;
- `docs/vision.md`: authoritative project vision;
- `docs/mvp1-corpus.md`: accepted and completed pilot corpus;
- `docs/mvp1-review.md`: MVP 1 exit-criteria audit;
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
cargo run -p atlas -- index
cargo test --workspace
scripts/check-mvp1.sh
```

`scripts/check-mvp1.sh` is the complete offline MVP 1 acceptance gate. It checks
formatting, feature profiles, tests, Clippy, registry evidence, and deterministic
index reconstruction.

The YAML files committed to Git are authoritative. The validator loads the
aggregate registry in memory and checks its schema, cross-references, local
files, implementation evidence, and Rust test symbols.

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

See [the MVP status](docs/mvp-status.md), [schema 0.1](docs/schema-0.1.md), and
[accepted decisions](docs/decisions/). The [MVP 1 review](docs/mvp1-review.md)
and [roadmap](docs/roadmap.md) describe what remains after corpus completion.
