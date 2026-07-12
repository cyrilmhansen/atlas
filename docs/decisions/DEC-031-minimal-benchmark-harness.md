# DEC-031 - Minimal Atlas benchmark harness

## Status

Accepted on 2026-07-12 (`bench-B`) as a reversible MVP 2 experiment.

## Decision

Use a small `atlas-bench` adapter crate around `std::time::Instant`. Preparation
and correction validation occur outside timed regions. Results retain every raw
sample plus minimum, median, maximum, and median absolute deviation.

Every result records commit and dirty state, compiler, architecture, operating
system, target triple, Rust flags, CPU model, logical CPU count, build profile,
dataset ID, digest, size, seed, warmup count, and measured sample count.
Comparisons are rejected when contexts, dataset digests, or settings differ.
Runs also flag median absolute deviation above 5% and a difference above 5%
between the medians of the first and second halves of the sample series.

## Consequences

The knowledge model remains independent of execution backends. Benchmarks are
run in release mode and are never correction tests. This harness is exploratory:
it stores no execution record and makes no generalized performance claim.
Criterion or another statistical tool may later be compared using the same
datasets if results become decision-critical.
