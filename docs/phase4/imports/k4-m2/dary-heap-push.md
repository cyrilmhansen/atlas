# K4-M2 import - dary_heap quaternary push

Status: accepted foreign implementation

Date: 2026-07-15

## Identity and sources

- Problem: `priority_queue.push`
- Algorithm: `priority_queue.quaternary_heap.push`
- Implementation: `priority_queue.push.dary_heap.quaternary.0_3_9`
- Upstream: `dary_heap` 0.3.9, `QuaternaryHeap<T>` aliasing
  `DaryHeap<T, 4>`
- License: MIT OR Apache-2.0
- API source: [DaryHeap::push](https://docs.rs/dary_heap/0.3.9/dary_heap/struct.DaryHeap.html#method.push)
- Package source: [dary_heap 0.3.9](https://docs.rs/crate/dary_heap/0.3.9)

The dependency is exactly pinned with default features disabled. It has no
mandatory normal dependency, uses `alloc::vec::Vec`, and supports `no_std`.

## Fidelity

Atlas calls the upstream `QuaternaryHeap` API directly. It does not translate,
wrap or reimplement heap repair. Test assertions only supply integers and
observe `push`, `peek`, `pop` and `capacity`.

- Bibliographic fidelity: exact package version, project, API and license.
- Algorithmic fidelity: upstream four-child max-heap insertion is unchanged.
- Representation fidelity: the upstream concrete `DaryHeap<T, 4>` state is
  retained rather than rewritten as Atlas's existing binary heap.
- Executable fidelity: tests execute upstream code on empty, reserved and
  exhausted states, including duplicate maxima and interleaved operations.
- Transformations: none in the algorithm; Atlas maps the concrete API to the
  existing abstract `MaxPriorityQueue<T>` problem contract.

The crate source states that its implementation is based on Rust's standard
library `BinaryHeap`. This is project and arity diversity, not independent code
ancestry, and must not be used as independent confirmation of heap correctness.

## Claims and boundaries

Upstream documents expected `O(1)` insertion averaged over input order and many
pushes, amortized `O(log n)` for ascending input, and `O(n)` for one call when
exhausted capacity resizes. `with_capacity` documents that elements fitting in
the reserved capacity do not reallocate.

Combining that capacity contract with the four-ary sift-up implementation
supports an inferred `O(log n)` worst bound when spare capacity excludes
growth. Schema 0.1 cannot attach the capacity condition to this cost, so the
conditioned claim remains in the Phase 4 result and private overlay. It is not
flattened into an unconditional public complexity claim.

Likewise, `DaryHeap<T, 4>` and `std::collections::BinaryHeap<T>` are distinct
Rust types. Both refine the abstract queue state, but neither accepts the
other's existing concrete state without an explicit conversion. No conversion
is introduced or hidden.

## Acceptance tests

`external_dynamic_structure_adapters` verifies:

- duplicate occurrences and max ordering;
- interleaved `push` and `pop`, including empty-state behavior;
- stable capacity for pushes within a reservation;
- actual capacity growth from an exhausted empty allocation.

Behavioral tests establish the problem contract and allocation regimes, not
asymptotic complexity or comparative speed.
