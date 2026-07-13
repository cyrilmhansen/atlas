# MVP 3 review

Review date: 2026-07-13. Active scope: the narrow experimental composition
slice authorized by DEC-037.

## Demonstrated scenario

`atlas compose cleanup` renders one non-persistent linear composition:

```text
filter.in_place -> sort.insertion -> deduplicate.quadratic
```

Its internal structural description states the mutable `Vec<i32>` input, the
predicate and ordering assumptions, and the `StableUniqueSequence<i32>` output.
Each step states its implementation identifier, input, output, mutation, copy,
and allocation effects. The selected plan minimizes declared *intermediate*
allocations: filtering and sorting mutate the same vector, while deduplication
copies into the required output.

The renderer also presents `filter.copy -> sort.merge -> deduplicate.hash` as
an explicitly rejected compatible alternative. Its copied filter output, merge
scratch storage, hash-set storage, and output allocation explain the rejection
for this one objective.

## Deliberate limits

- The types are internal Rust values for this scenario, not schema 0.1 fields
  or a persistent plan format.
- The plan is selected from a fixed, reviewed pair of candidates; it is not a
  general planner or search engine.
- It renders orchestration intent but does not execute a generated program.
- The objective interprets declared effects only. It does not turn them into
  empirical allocation measurements.

## Acceptance checks

```sh
cargo test -p atlas --locked --offline
cargo run -q -p atlas --locked --offline -- compose cleanup
```

The unit tests require every selected mutation, copy, and allocation to remain
visible. CLI tests require the rejected alternative and its reason to render.
