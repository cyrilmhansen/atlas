# DEC-028 - Close MVP 1 and activate MVP 2

## Status

Accepted on 2026-07-12 (`mvp2-A`).

## Decision

Close MVP 1 at the local baseline commit `8a2a520` and activate MVP 2 in an
experimental phase focused on empirical qualification.

The first slice prototypes separate dataset specifications for
`sequence.sort` and `sequence.partition`. It covers typical, boundary,
degenerate, adversarial, and regression cases with deterministic generation and
explicit seeds. These Rust types are internal experiment version 0 and do not
change or extend the public aggregate schema 0.1.

Benchmarks, execution records, and public dataset serialization remain deferred
until the experiment demonstrates a useful representation.
