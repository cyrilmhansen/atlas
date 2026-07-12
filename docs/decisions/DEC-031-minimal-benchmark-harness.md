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

The comparison suite rotates implementation order on every round. Warmup stops
only after every implementation has passed three consecutive comparisons of
adjacent five-sample windows within a 5% median tolerance. It is bounded and
fails without producing measured results when stability is not reached.
After measurement, any dispersion or half-series-drift warning makes the whole
suite unsuitable as an observation candidate and the example exits with an
error after printing its diagnostics.
The suite also rejects an individual sample more than 20% from the median and
compares medians grouped by execution position, rejecting position bias above
the same 5% tolerance.

Under `env-A`, the suite non-invasively captures Linux load averages, effective
CPU affinity, voluntary and involuntary context-switch counters, scheduler
migrations, scaling governors, and the visible current-frequency range before
and after execution. Missing platform data remains explicitly unavailable. No
affinity, governor, frequency, or scheduling state is changed.

Each implementation is calibrated independently to approximately 10 ms of
algorithm work per sample. A batch contains multiple independently prepared
inputs; preparation stays outside the timed region. Total batch duration,
invocation count, and normalized duration are retained. Prepared buffers are
bounded to 64 MiB per batch. Page-fault counters are captured with the other
boundary diagnostics.
After a stable warmup, the suite may recalibrate the batch from the observed hot
per-invocation median and restart warmup. At most two recalibrations are allowed;
a change smaller than or equal to 10% keeps the current batch.
Prepared pools are allocated once per calibration size and reused across all
warmup and measured samples. Input values are restored into existing buffers
outside the timer; scratch capacities are retained. This prevents allocator and
minor-page-fault churn between samples.
Requested adaptive settings are stored separately from the actual warmup and
recalibration counts. Cross-run comparability uses the requested protocol;
different convergence lengths are observations, not parameter mismatches.

## Consequences

The knowledge model remains independent of execution backends. Benchmarks are
run in release mode and are never correction tests. This harness is exploratory:
it stores no execution record and makes no generalized performance claim.
Criterion or another statistical tool may later be compared using the same
datasets if results become decision-critical.
