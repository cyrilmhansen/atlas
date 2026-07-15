# K4-M2 priority-queue source review

Status: Option A accepted as `dary-A`; implementation complete

Date: 2026-07-15

## Target

Add a structurally distinct foreign implementation for the existing
`priority_queue.push` Problem. The competitor must remain a max-priority queue,
preserve duplicate occurrences, mutate existing queue state and expose capacity
growth separately from heap repair.

The frozen requests remain `push.in_place`, `push.no_growth_without_capacity`,
`push.log_worst_with_spare_capacity` and `push.state_compatible`.

## Option A - dary_heap 0.3.9 QuaternaryHeap

Use
[`dary_heap::QuaternaryHeap<T>`](https://docs.rs/dary_heap/0.3.9/dary_heap/type.QuaternaryHeap.html),
a `DaryHeap<T, 4>` with an API modeled after `std::collections::BinaryHeap`.
The upstream documentation states `O(log n)` insertion, explains the reduced
depth/increased per-level work of higher arity, and documents that a single
`push` is `O(n)` when exhausted capacity resizes.

The exact 0.3.9 Cargo metadata, verified with `cargo info dary_heap`, declares
MIT OR Apache-2.0, no mandatory normal dependency and optional Serde only. The
crate uses `alloc::vec::Vec`, advertises `no_std` and provides `reserve` and
`with_capacity`, so the existing spare-capacity experiment remains applicable.

Cost:

- one pinned, default-feature-free workspace dependency;
- one Algorithm and one Implementation for `priority_queue.push`;
- one focused adapter test using actual upstream `push`, `peek`, `pop`,
  `capacity` and `reserve` behavior;
- one import worksheet and frozen-request matrix.

Risks:

- its implementation is explicitly based on the Rust standard-library heap,
  so it supplies a distinct project and arity but not wholly independent code
  ancestry;
- binary and quaternary concrete states are not interchangeable even though
  both refine the abstract `MaxPriorityQueue<T>` problem state;
- performance claims about one arity being faster remain workload observations,
  not selection facts.

Reversibility: high. The dependency, two entities and one test are isolated and
can be removed without schema or production API migration.

## Option B - orx-priority-queue 1.8.0

Use
[`orx_priority_queue::DaryHeap`](https://docs.rs/orx-priority-queue/1.8.0/orx_priority_queue/struct.DaryHeap.html).
The project is MIT OR Apache-2.0, fully documented, supports `no_std` without
default features and provides queue traits plus binary and quaternary variants.

Cost and risks:

- substantially larger crate surface centered on `(node, key)` pairs and a
  minimum-key queue contract;
- adapting `MaxPriorityQueue<T>` requires an ordering inversion and an explicit
  item/key specialization;
- its richer decrease-key variants and optional integrations are irrelevant to
  `push` and could blur the state-compatibility question.

Reversibility: high while test-only, but integration measures more semantic
adaptation than the selected operation requires.

## Option C - Atlas-authored quaternary heap

Implement a small `Vec<T>` quaternary heap directly in the adapter test.

Cost and risks:

- no dependency and complete control over capacity cases;
- not a foreign implementation, so it weakens the main Phase 4 hypothesis;
- duplicates reviewed heap code and makes Atlas responsible for correction.

Reversibility: high, epistemic value low.

## Recommendation

Recommend **Option A (`dary-A`)**.

It holds language, item model, max ordering, contiguous storage and API shape
nearly constant while changing arity. That isolates the decision-relevant
questions of concrete state compatibility and conditioned capacity cost. Its
shared standard-library ancestry is recorded and prevents overclaiming source
independence.

## Minimum experiment

1. Pin 0.3.9 with default features disabled.
2. Test duplicate preservation, max order, interleaved push/pop and empty state.
3. Test a reserved-capacity push separately from an exhausted-capacity growth.
4. Discover both push candidates through registry relations, never IDs.
5. Adjudicate the four frozen requests and stop after the two-candidate matrix.

Owner decision: `dary-A`.
