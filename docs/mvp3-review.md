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

`atlas compose cleanup --goal expected-time` selects that same copying
merge/hash candidate instead. The selection is explicitly conditional on
`i32: Eq + Hash`: filter is declared `O(n)`, merge sort `O(n log n)`, and hash
deduplication `O(n)` expected. The alternative is rejected for its declared
quadratic insertion-sort and quadratic-deduplication worst cases. This is not a
measurement or a universal latency claim; hash deduplication retains its
declared adversarial `O(n^2)` worst case.

`atlas compose cleanup --rust` renders the Rust orchestration for the selected
candidate. The source is also `crates/atlas/examples/cleanup_generated.rs`, so
Cargo compiles and runs the exact emitted program. It filters a caller-owned
vector in place, sorts that vector in place, then returns a separately allocated
deduplicated vector.

## Deliberate limits

- The types are internal Rust values for this scenario, not schema 0.1 fields
  or a persistent plan format.
- The plan is selected from a fixed, reviewed pair of candidates; it is not a
  general planner or search engine.
- Atlas renders source but does not compile or execute it as part of the CLI.
  The separately runnable Cargo example is the verification boundary.
- The objective interprets declared effects only. It does not turn them into
  empirical allocation measurements.
- Rust source generation is verified only for the allocation objective. Atlas
  rejects `--goal expected-time --rust` until that second source is separately
  compiled and exercised.

## Acceptance checks

```sh
cargo test -p atlas --locked --offline
cargo run -q -p atlas --locked --offline -- compose cleanup
cargo run -q -p atlas --locked --offline -- compose cleanup --goal expected-time
cargo run -q -p atlas --locked --offline -- compose cleanup --rust
cargo run -q -p atlas --locked --offline --example cleanup_generated
```

The unit tests require every selected mutation, copy, and allocation to remain
visible. CLI tests require the rejected alternative and its reason to render.
