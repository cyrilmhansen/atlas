# K-M4 importer B: binary-heap push

Protocol: `k-m0.2`

## 1. Work and source record

- Importer: `k-m4-importer-b`.
- Importer-observed batch start: `2026-07-15T01:12:49+02:00`.
- Externally observed batch start/end and elapsed minutes: unavailable to the
  importer; these fields are owned by the orchestrator.
- Importer-observed end: `2026-07-15T01:24:31+02:00`.
- Active authoring, source-reading and Atlas-modeling minutes: unavailable; no
  retrospective estimate is used.
- Human interventions: none.
- Tools: browser/web fetch, `curl` for the exact frozen page, editor.
- Source subject/local name: Rust `std::collections::BinaryHeap::push`.
- Maintainer/work/release: Rust project, Rust standard library 1.85.0.
- Source class and symbol: library API,
  `std::collections::BinaryHeap<T>::push`.
- Mandatory locator actually read:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#method.push>.
- Supplemental locator actually read:
  <https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html#time-complexity>.
- Retrieval date: 2026-07-15.
- Code/documentation license: Rust standard-library source is distributed under
  MIT or Apache-2.0; the frozen page does not restate documentation copyright.

## 2. Source-faithful account

- Problem: push one item into a mutable max binary heap (mandatory `push`
  description and its enclosing `BinaryHeap` API).
- Inputs: `&mut BinaryHeap<T, A>` and owned `item: T`, with `T: Ord`; current
  heap size is `n` (mandatory method signature; supplemental complexity table).
- Requires: the heap's element ordering must remain consistent while elements
  are in the heap. This enclosing-type logic-error condition is on the same
  mandatory page. Allocator requirements are carried by `A: Allocator`.
- Output: unit return; the heap is mutated to include the item (mandatory).
- Ensures: length increases by one and max-heap behavior includes the new item;
  the example observes `peek() == Some(&5)` after pushing 3, 5, 1 (mandatory).
- Strategy/invariant: the public page identifies a binary max-heap but does not
  specify sift-up steps in the `push` contract. Heap ordering is source context,
  not a derived implementation proof.
- State/effects: mutates persistent heap storage, consumes the item, and may
  reallocate when capacity is exhausted; no I/O or blocking is stated
  (mandatory complexity discussion).
- Time: expected O(1), averaged over every possible insertion ordering and many
  pushes, for inputs not already in a sorted pattern; predominantly ascending
  input degrades, with ascending order giving amortized O(log n) per push;
  one call is worst-case O(n) when resizing exhausted capacity. Resize is
  amortized in the preceding aggregate figures (mandatory `Time complexity`).
  The supplemental table marks push O(1) with the expected-cost suffix.
- Space: no auxiliary-space bound is stated. A resize can allocate replacement
  backing storage; exact allocation and peak-memory costs are unstated.
- Determinism: no randomness is used or stated. Observable heap contents are
  determined by inputs and `Ord`; layout among equivalent items is not promised.
- Variants: min-heap use through `Reverse` is shown on the enclosing page, but
  is not a distinct `push` operation.
- Source ambiguity: "expected" averages over possible orderings rather than an
  internal random choice; schema users could otherwise mistake it for randomized
  expected time.

## 3. Proposed Atlas normalization

- `Problem` identity: `priority_queue.push_max_heap`. It is a stateful priority-
  queue update, not construction of a heap from a complete collection.
- Exact problem input: `a mutable max binary heap of n ordered elements and one
  owned element of the same ordered domain`.
- Exact problem requires: `the element comparison defines the heap ordering and
  does not change for stored elements; allocation succeeds if growth is needed`.
- Exact problem output: `the same heap state containing the additional item`.
- Exact problem ensures: `heap cardinality is n + 1 and subsequent maximum-heap
  observations include the inserted item while preserving all prior items`.
- `Algorithm` identity: `priority_queue.binary_heap_sift_insert`, source-bounded
  to the Rust 1.85.0 binary-heap push contract. The name exposes conventional
  normalization; the frozen API does not document internal sift mechanics, so
  that strategy component remains `inferred`, not `declared`.
- Algorithm requires: problem requirements above.
- Determinism: `true` for abstract multiset/maximum behavior; internal layout
  and equal-item order are outside the guarantee.
- Time: worst single call O(n); expected aggregate O(1) under the page's ordering
  average; ascending-order amortized O(log n). Schema 0.1 can store only one
  worst and one expected claim and loses the workload-conditioned amortized one.
- Auxiliary memory: not stated by source. Temporary representation should say
  `not stated; growth may reallocate O(n) stored elements` rather than claim O(1).
- Implementation identity: `rust-1.85.0.std.binary_heap.push`.
- Implementation boundary/effects: Rust standard-library method, consumes `T`,
  mutates heap, may allocate/reallocate, no stable ABI asserted, no I/O/blocking.
  Later tests should check empty/nonempty, duplicate/equal items, ascending and
  descending streams, preserved multiset, and capacity-resize boundary.
- Evidence: API behavior and cost regimes `declared`; implementation identity
  `declared`; no executable behavior was tested here.
- Existing Atlas synonym: not assessed because the registry is outside this
  experiment's permitted sources.
- Documentary only: allocator details, backing-vector layout, equal-key layout,
  and internal sift path.

## 4. Fidelity and transformations

### Bibliographic fidelity

Release, type, method, and both exact anchors are preserved. The documentation
page lacks a commit locator in this worksheet. Assessment: `partial`.

### Algorithmic fidelity

Mutation, ordering contract, and all three cost regimes are preserved. Sift-up
is not promoted to a source fact. Assessment: `preserved` for the API contract,
`unresolved` for implementation strategy.

### Representational fidelity

Rust ownership, `Ord`, allocator, and method boundary remain explicit; physical
layout is intentionally omitted. Assessment: `intentionally transformed`.

### Executable fidelity

An upstream implementation and runnable examples exist. Proposed oracle compares
the multiset before/after and drains in nonincreasing order. No behavior was run
in this worksheet. Assessment: `not assessed`.

### Declared transformations

- Translation: Rust signature to a state-transition contract.
- Specialization/generalization: max-heap push only; `Reverse` remains a type
  adaptation, not a new algorithm.
- Type adaptation: `T: Ord` becomes an ordered element domain.
- API aggregation/decomposition: one method remains one stateful operation.
- Bug correction: none.
- Pedagogical simplification: no backing-array mechanics claimed.
- Other: `sift_insert` is explicitly inferred conventional vocabulary.

## 5. Schema-loss record

Schema 0.1 cannot structurally represent persistent container state, comparison
stability, workload-conditioned expected versus amortized versus one-call worst
costs, capacity-triggered allocation, or abstract multiset equality distinct
from physical layout. Prose is lossy: candidates with O(1) expected push but
different adversarial and allocation behavior can become indistinguishable.

## 6. Selection requests

1. **Request:** push into a pre-reserved max heap under nonsorted insertion order;
   expected throughput is the objective. **Accept**, because the source declares
   expected O(1) over the stated ordering model; explain that it is not worst-case.
2. **Request:** every individual push must be O(log n) worst-case, including
   allocation. **Reject**, because one capacity-exhausting call is O(n).
3. **Request:** no allocation during the operation. **Accept only with an external
   precondition that spare capacity exists**; otherwise reject this implementation
   boundary because `push` may resize.

These consequences are manual; current qualification cannot express the regimes.

## 7. Ambiguities and conclusion

- Source ambiguity: the source does not state auxiliary peak memory or equal-key
  layout; "expected" is workload-order averaging.
- Protocol ambiguity: whether a conventional strategy name may appear in the
  proposed identity when internals are not source-declared. It is marked inferred.
- Model ambiguity: whether allocation-triggered O(n) belongs in `time_worst`
  while the central algorithmic bound is amortized O(log n).
- Decision-relevant divergence: choosing expected O(1), amortized O(log n), or
  single-call O(n) as the sole headline changes selection.
- Public schema change requested: **none**. Preserve all regimes in this worksheet
  pending evidence from another structural family.
