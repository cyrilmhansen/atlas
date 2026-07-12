# DEC-032 - Linux pinned benchmark runner

## Status

Accepted on 2026-07-12 (`env-B`).

## Decision

Provide `scripts/run-benchmark-linux.sh CPU` as an explicit Linux-only wrapper.
It builds the release executable before pinning, verifies the requested CPU and
required tools, refuses a one-minute system load above half the available CPU
count, reports governor and maximum frequency, then runs only the benchmark
process through `taskset --cpu-list`.

The CPU argument is mandatory because heterogeneous cores can have materially
different frequency limits. The runner reads but never modifies affinity of the
calling shell, governor, frequency, scheduler policy, or other system state.

Under `isolation-A`, the wrapper starts one fresh pinned process per sorting
implementation. Each process performs its own calibration, adaptive warmup,
measurement, diagnostics, and quality verdict. The wrapper continues after a
rejection to collect all diagnostics and returns failure if any process fails.
It does not parse or aggregate human-readable output.

## Consequences

The benchmark context records the resulting single-CPU affinity and migration
counter. Results from different CPUs or environments remain incomparable. This
wrapper is an optional execution adapter; Atlas datasets and benchmark records
do not depend on Linux or `taskset`.
Cross-process ranking remains deferred until a structured observation format is
approved; process isolation must not introduce ad hoc text parsing.
