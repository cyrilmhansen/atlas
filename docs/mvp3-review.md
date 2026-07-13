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
deduplicated vector. `--goal expected-time --rust` likewise renders and is
identical to `cleanup_expected_time_generated`; it copies the filter result,
sorts that copy with merge sort, and performs hash-based deduplication.

`atlas compose find` adds a structurally different composition:

```text
sort.insertion -> search.binary
```

The plan states binary search's sorted-input precondition and explicitly marks
the sort step as establishing it. It rejects merge sort for the allocation
objective because merge scratch is declared. `atlas compose find --rust` is
identical to the compiled `find_generated` example; it accepts a mutable slice,
sorts it, then returns the first matching index from binary search.

`atlas compose partition-sort` demonstrates a structured intermediate value:
stable partition produces `matching` and `rejected` vectors, the plan projects
only `matching` for in-place sorting, then reassembles both branches. Both
partition-output allocations, the projection, and reassembly are explicit.

`atlas compose unique-sort` isolates the `sort -> deduplicate` shape. It
selects `sort.insertion -> deduplicate.quadratic`: sorting mutates the supplied
sequence without declared allocation, then deduplication allocates the required
unique output. `sort.merge -> deduplicate.hash` remains a compatible rejected
candidate because it adds declared merge scratch and hash-set storage. Its
`--rust` output is the compiled `unique_sort_generated` example.

All composition scenarios accept an explicit `--force IMPLEMENTATION_ID` or `--forbid
IMPLEMENTATION_ID`. The constraint is evaluated only against their reviewed
candidates: it either retains, swaps, or rejects the candidate set with a
rendered reason. Forbidding binary search from `find`, for example, rejects the
request because no candidate remains. Constraints do not modify the registry.

## Deliberate limits

- The types are internal Rust values for this scenario, not schema 0.1 fields
  or a persistent plan format.
- The plan is selected from a fixed, reviewed pair of candidates; it is not a
  general planner or search engine.
- Atlas renders source but does not compile or execute it as part of the CLI.
  The separately runnable Cargo example is the verification boundary.
- The objective interprets declared effects only. It does not turn them into
  empirical allocation measurements.
- Rust generation is unavailable with an override. The existing sources verify
  only their unconstrained candidate, so emitting another source would be
  misleading until it is separately compiled and exercised.

## Acceptance checks

```sh
cargo test -p atlas --locked --offline
cargo run -q -p atlas --locked --offline -- compose cleanup
cargo run -q -p atlas --locked --offline -- compose cleanup --goal expected-time
cargo run -q -p atlas --locked --offline -- compose cleanup --rust
cargo run -q -p atlas --locked --offline -- compose cleanup --goal expected-time --rust
cargo run -q -p atlas --locked --offline --example cleanup_generated
cargo run -q -p atlas --locked --offline --example cleanup_expected_time_generated
cargo run -q -p atlas --locked --offline -- compose find
cargo run -q -p atlas --locked --offline -- compose find --rust
cargo run -q -p atlas --locked --offline --example find_generated
cargo run -q -p atlas --locked --offline -- compose partition-sort
cargo run -q -p atlas --locked --offline -- compose partition-sort --rust
cargo run -q -p atlas --locked --offline --example partition_sort_generated
cargo run -q -p atlas --locked --offline -- compose unique-sort
cargo run -q -p atlas --locked --offline -- compose unique-sort --rust
cargo run -q -p atlas --locked --offline --example unique_sort_generated
cargo run -q -p atlas --locked --offline -- compose cleanup --forbid filter.in_place.rust.vec.v1
```

The unit tests require every selected mutation, copy, and allocation to remain
visible. CLI tests require the rejected alternative and its reason to render.
