# DEC-055 - Separate theory, deterministic counters and local timing

## Status

Accepted on 2026-07-13 (`metrics-A`).

## Decision

Present three distinct classes of algorithm characteristic in a future public
artifact:

1. sourced theoretical time and auxiliary-space complexity;
2. deterministic operation counts over an identified dataset and execution;
3. optional wall-clock timing observed locally in the browser.

Deterministic counters may include comparisons, reads, writes, swaps, copies
and requested auxiliary storage when those operations have an explicit tested
boundary. Browser timings must record the dataset, repetitions and available
runtime environment, and must not be used to infer asymptotic complexity or a
portable implementation ranking.

## Consequences

- The UI and derived data must not visually or semantically merge declared
  complexity, counted dynamics and observed timing.
- A counter is shown only when its operation boundary is specified and tested;
  missing counters remain unavailable rather than estimated.
- Custom browser inputs and timings are ephemeral and are not registry evidence.
- Persisting or publishing these observations as a supported format requires a
  separate class C evidence-schema decision.
- This decision selects measurement semantics but does not activate MVP 5.
