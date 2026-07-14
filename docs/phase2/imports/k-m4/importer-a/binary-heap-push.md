# K-M4 import A: binary-heap push

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable; no retrospective timing is
  used as evidence.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` and `pandoc` for the frozen page.
- Subject: Rust `BinaryHeap::push`.
- Maintainer/work/release: Rust project, Rust standard library `1.85.0`.
- Source class: standard-library API and implementation documentation.
- Required locator actually read:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#method.push>.
- Supplemental locator actually read:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#time-complexity>.
- Retrieval date: 2026-07-15.
- Code and documentation license: not stated on the frozen page; no source code
  is copied.

## Source-faithful account

- Problem: push one item into a max-priority queue implemented as a binary heap
  (mandatory `BinaryHeap` description and `push`).
- Inputs: mutable `BinaryHeap<T, A>` and owned `item: T`; the shown primary
  implementation requires `T: Ord` (mandatory `push` signature and enclosing
  type documentation).
- Preconditions: the relative order of values already in the heap must not be
  changed through interior mutation. Violation is a logic error whose effects
  remain encapsulated but may include panic, incorrect results, abort, leak or
  nontermination (mandatory `BinaryHeap` description).
- Output and guarantees: `push` returns no value; afterward the item belongs to
  the heap and documented heap operations preserve the heap invariant, provided
  the ordering precondition holds (mandatory `push` and type guarantee).
- Strategy: the source identifies a binary heap but does not describe the
  percolation steps of `push`; no sift-up pseudocode is imported.
- State and effects: the heap is mutated. Capacity exhaustion can trigger resize,
  which accounts for an O(n) individual call; therefore allocation and moving
  stored items are possible (mandatory `push` time-complexity section).
- Failure/I/O: no I/O. Allocation failure behavior is not stated at the selected
  locator. Ordering logic errors have the bounded but otherwise unspecified
  outcomes above.
- Time: expected O(1), averaged over every possible ordering and sufficiently
  many pushes, for inputs not already in a sorted pattern. Predominantly
  ascending insertion degrades; ascending order gives amortized O(log n) per
  push. A single call is O(n) worst case when resize is needed (mandatory
  `push` time-complexity). The summary table marks push `O(1)~`, where `~`
  denotes expected cost (supplemental time-complexity table).
- Space: not stated. The resize explanation establishes possible capacity growth
  but not a formal auxiliary-space bound.
- Determinism: no randomness is specified. Observable priority-queue behavior is
  determined by `Ord`; ordering among equal elements and internal layout are not
  guaranteed by the page.
- Variants: `Reverse` or a custom `Ord` turns the max-heap interface into a
  min-heap (mandatory type page). Cost regimes distinguish unsorted-average,
  ascending amortized, and resize worst case.
- Source ambiguity: “expected” averages over possible insertion orderings, but
  no probability distribution over orderings is defined beyond that wording.

## Proposed Atlas normalization

- `Problem` identity: `priority_queue.push`. It denotes addition of one item to
  an existing priority queue, independent of this binary-heap implementation.
- Exact `input`: `(queue, item)`, where `queue` is a mutable max-priority queue
  containing `n` values governed by one stable total ordering.
- Exact `requires`: item and stored values share a valid `Ord`; no stored value
  changes relative ordering while resident; implementation capacity/allocation
  preconditions remain implementation-specific.
- Exact `output`: the same queue state with one additional occurrence of `item`;
  no direct return value.
- Exact `ensures`: length increases by one, all previous occurrences remain, and
  subsequent priority observation returns a maximal value under `Ord`.
- `Algorithm` identity: `priority_queue.binary_heap_push`. “Sift up” is not placed
  in the ID because the selected documentation does not state its steps.
- Algorithm requirements: binary-heap representation and stable `Ord`.
- Determinism: declared deterministic at the abstract multiset/max interface;
  equal-element layout is unspecified.
- Time: source-declared expected O(1) for averaged unsorted orderings; amortized
  O(log n) for ascending pushes; worst single call O(n) including resize.
- Memory: not stated by the source and must remain `not stated`, despite schema
  0.1 requiring `auxiliary_memory`.
- Evidence: API and costs would be `declared`; no executable test or proof was
  performed.
- Implementation boundary: `std::collections::BinaryHeap<T, A>::push` in Rust
  1.85.0. It consumes `item`, mutates the heap, may allocate/resize/move values,
  performs no documented I/O or blocking.
- Candidate tests: empty heap, duplicates, descending/ascending/randomized input
  orders, pre-reserved versus exhausted capacity, and post-push maximum/multiset
  preservation. Tests establish behavior, not the expected-cost theorem.
- Information left documentary: probability model over insertion orders,
  allocator failure, equal-key layout and exact movement/comparison counts.

## Fidelity

### Bibliographic fidelity

Version, type, method and both official anchors are preserved. The implementation
commit and page license are not stated. Assessment: **partial**.

### Algorithmic fidelity

The heap contract, ordering validity and all three documented cost regimes are
preserved. Internal sift mechanics are intentionally not inferred. Assessment:
**preserved** at the published API boundary.

### Representational fidelity

The Rust method becomes a language-neutral priority-queue operation while its
`Ord`, allocator and ownership boundary remain implementation facts. Assessment:
**intentionally transformed**.

### Executable fidelity

An upstream implementation and examples exist. A future oracle can compare
multisets, length and maximum after each push. Nothing was executed here.
Assessment: **not assessed**.

### Declared transformations

- Translation: Rust ownership/API to an abstract state transition.
- Generalization: `priority_queue.push` separates the problem from binary heap.
- Representation adaptation: `Ord` becomes a stable total-order requirement.
- API decomposition/aggregation: one method remains one operation.
- Bug correction: none.
- Pedagogical simplification: none.
- Other: no implementation strategy added beyond the source.

## Model friction

| Source fact | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| Three incomparable cost regimes | `time_worst` / `time_expected` strings | lossy | selection |
| Expected cost depends on ordering distribution | no structured condition | absent | selection, substitution |
| Resize changes one-call cost and allocation | effects prose | lossy | selection, composition |
| Auxiliary memory not stated | required algorithm field | ambiguous | identity, selection |
| Ordering logic error has multiple outcomes | requirements prose | lossy | selection |

Two candidates could appear equivalent despite different capacity guarantees or
adversarial insertion behavior. An O(log n)-per-call request could incorrectly
accept this implementation if the O(n) resize call is hidden. Similar mixed
expected/amortized/worst regimes occur independently in hash-map insertion, so
the gap is not heap-specific. Temporary representation belongs in worksheet
annotations, not a fabricated scalar complexity.

## Selection requests

1. “Push a long, non-patterned stream; optimize expected aggregate CPU cost and
   permit growth.” **Accept**: the source declares expected O(1) under this
   ordering-average premise and allows resize.
2. “Every individual push must be O(log n) or better, including allocation.”
   **Reject**: an exhausted-capacity call is O(n).
3. “Do not allocate during push; the caller guarantees sufficient spare
   capacity.” **Accept conditionally**: the documented O(n) resize cause is
   excluded by the caller premise; mutation and comparisons remain.

The accept/reject consequences cannot currently be reproduced by generic
qualification because cost conditions and capacity are only prose.

## Ambiguities and conclusion

- Source ambiguity: distribution behind expected O(1), and unstated auxiliary
  memory/allocation failure.
- Protocol ambiguity: whether a problem-level `push` must specify max versus min
  orientation or treat it as comparator configuration.
- Model ambiguity: which cost regime schema 0.1's required `time_worst` should
  expose without hiding the other two.
- Recommended normalization: generic priority-queue push problem plus a
  binary-heap-specific algorithm and Rust 1.85 implementation boundary.
- Minimal next probe: compare pre-reserved and forced-resize calls over ascending
  and shuffled sequences while counting allocations and comparisons.
- Public schema change requested: **none**. Preserve the mismatch as experimental
  annotation until another structural family is adjudicated.
