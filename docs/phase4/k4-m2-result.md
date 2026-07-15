# K4-M2 dynamic priority-queue competition result

Status: complete; second foreign competition supported with qualifications

Date: 2026-07-15

Authority: DEC-074, `dary-A`, `b2-follow-A` and the K4-M0 protocol

## Result

Atlas now registers `dary_heap` 0.3.9 quaternary max-heap insertion as a second
Algorithm and Implementation for the existing `priority_queue.push` Problem.
The exact upstream crate is executed directly with default features disabled.
No Problem, schema field, CLI behavior, runtime or Explorer-specific branch was
added.

Relation-driven discovery yields two algorithms and two implementations. Tests
cover duplicate maxima, interleaved operations, empty state, reserved-capacity
pushes and exhausted-capacity growth.

## Frozen-request adjudication

| Request | Binary heap | Quaternary heap | Authoritative reason |
|---|---|---|---|
| `push.in_place` | `accepted` | `accepted` | both mutate existing concrete queue storage |
| `push.no_growth_without_capacity` | `rejected` | `rejected` | both may grow and reallocate exhausted storage |
| `push.log_worst_with_spare_capacity` | `unsupported-query` | `unsupported-query` | the conditioned `O(log n)` facts are sourced, but schema 0.1 and `atlas qualify` cannot express the capacity condition or asymptotic request |
| `push.state_compatible` with a supplied binary-heap state | `unsupported-query` | `unsupported-query` | exact concrete state types are sourced, but the public query cannot ask state compatibility; documentary adjudication accepts binary and rejects quaternary |

The last row fixes the supplied state as `std::collections::BinaryHeap<T>`;
without a named supplied representation, compatibility has no truth value.
Common `MaxPriorityQueue<T>` problem identity does not imply concrete-state
interchangeability.

## Unchanged-evaluator control

The separate `k4-m2-priority-overlay.yaml` restates the two discovered
candidates in the frozen K-M5 format. Without evaluator changes it:

- accepts both candidates for in-place mutation;
- rejects both for no-growth without a spare-capacity condition;
- accepts both conditioned `O(log n)` profiles when spare capacity is supplied;
- accepts only the binary candidate for a supplied binary concrete state.

This family does not reproduce the graph batch's absent-versus-refuted
guarantee ambiguity: allocation contradiction is already an explicit effect.
It does reproduce the other limitation exactly: candidates and their
decision facts must be manually projected into the overlay. K-M5 therefore
remains a useful partial oracle, not manifest-driven qualified selection.

## Cost and provenance

The batch adds one pinned dev-dependency, one Algorithm, one Implementation,
two direct upstream-adapter tests, one import worksheet and one phase-local
overlay. `dary_heap` is a distinct maintained project and changes heap arity,
but its code explicitly derives from the standard-library heap; the batch does
not claim independent code ancestry.

The derived index contains 111 entities, 80 relations and 718 claims. Its
logical SHA-256 is
`07a24888ebb8fc6a75d1765bd85867e615bb1ef4cd1993cca2b12c8b2da05e04`.

## Consequence

K4-M2 supports generic relation discovery and shows that conditioned costs and
typed state transfer across a second family in the private vocabulary. It also
shows that the recurring architectural deficit is projection/query machinery,
not yet a recurring missing public fact. Per the stop rule, no third heap
candidate and no evaluator extension is added. K4-M3 can now test exact bounded
top-k output and cost requirements independently.
