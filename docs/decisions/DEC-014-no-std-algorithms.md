# DEC-014 - Separate no_std algorithm crate

- Status: accepted
- Class: C
- Date: 2026-07-11

## Context

Algorithm implementations should remain usable by a minimal runtime. `Ordering`
belongs to `core`, while collection-producing implementations need `alloc`.
Registry parsing, file access, and the CLI genuinely need `std`.

## Decision

Move implementation modules into `atlas-algorithms`, declared `#![no_std]`.
Core-only algorithms are always available. Algorithms requiring `Vec` are
enabled by an optional `alloc` feature, enabled by default for workspace tests.
Keep the `atlas` registry and CLI on `std`.

## Consequences

Registered entrypoints and source paths change to the new crate. The following
commands define the portability checks:

```sh
cargo check -p atlas-algorithms --no-default-features
cargo check -p atlas-algorithms --features alloc
```
