# K4-M0 frozen comparative-selection protocol

Status: complete; frozen before Phase 4 competitor import

Date: 2026-07-15

Authority: DEC-074 and `docs/phase4-comparative-selection.md`

## Question

Can a later candidate be discovered from registry relations and adjudicated
against requests fixed before its import, while distinguishing incompatibility,
missing query machinery and missing authoritative knowledge?

This protocol names current candidates for baseline accounting. Its request
oracles deliberately do not name or assume identifiers for future candidates.

## Baseline inventory

| Family | Problem | Current algorithm | Current implementation |
|---|---|---|---|
| graph | `graph.reachable_traversal` | `graph.bfs.traversal` | `graph.bfs.petgraph.0_8_3` |
| dynamic | `priority_queue.push` | `priority_queue.binary_heap.push` | `priority_queue.push.rust.std.1_85` |
| streaming | `stream.top_k` | `stream.top_k.min_heap` | `stream.top_k.rust.std_binary_heap.v1` |

Each problem has exactly one current algorithm and implementation candidate.
K4-M1 through K4-M3 must add competitors through `solves` and `implements`; no
request or discovery path may enumerate the identifiers above or the future
ones.

## Frozen requests

Evidence levels are part of the oracle. A candidate is not accepted from an
undocumented implementation behavior or a test that does not establish the
requested theoretical guarantee.

### Graph reachability

| Request | Required decision | Reason |
|---|---|---|
| `reach.exact_component` | accept every candidate whose sourced contract yields each reachable node exactly once and no other node | base problem equivalence; traversal order is not a ranking |
| `reach.no_allocation` | reject candidates whose implementation allocates frontier or discovered state | concrete implementation effect, already represented |
| `reach.frontier_for_known_shape` | unsupported unless a sourced claim conditions retained frontier on graph shape, such as depth or layer width | plain `O(V)` does not decide between structurally different frontiers |
| `reach.non_decreasing_hops` | reject or unsupported unless the algorithm contract explicitly guarantees nondecreasing source distance | BFS intuition must not be borrowed from the algorithm name |

### Priority-queue push

| Request | Required decision | Reason |
|---|---|---|
| `push.in_place` | accept candidates declaring mutation of the existing state rather than replacement with a distinct queue | current schema 0.1 `in_place` control |
| `push.no_growth_without_capacity` | reject candidates whose implementation may grow storage when no spare-capacity condition is supplied | unconditional implementation effect is authoritative |
| `push.log_worst_with_spare_capacity` | unsupported unless a sourced worst-case claim is explicitly conditioned on sufficient capacity | K-M5 showed that unconditional `O(n)` and conditioned `O(log n)` are different decisions |
| `push.state_compatible` | unsupported unless the candidate's required state representation is compatible with the supplied queue state | common problem identity alone does not prove cross-representation state interchangeability |

### Exact bounded top-k

| Request | Required decision | Reason |
|---|---|---|
| `top_k.exact_bounded` | accept candidates preserving exactly `min(k, n)` occurrences with no omitted occurrence greater than a retained one and `O(k)` retained state | base exactness and memory contract |
| `top_k.no_allocation` | reject candidates whose implementation allocates retained or output storage | concrete implementation effect, already represented |
| `top_k.n_log_k_worst` | accept only candidates with sourced worst time no worse than `O(n log k)` | current min-heap claim is present, but the public CLI cannot query it |
| `top_k.sorted_output` | unsupported unless output order is explicitly guaranteed | the problem specifies membership, not presentation order |

## Outcome classes

Every candidate/request pair receives exactly one class:

- `accepted`: all required facts are sourced at an allowed evidence level;
- `rejected`: a sourced fact contradicts the request;
- `unsupported-fact`: the registry lacks a decision-required fact or condition;
- `unsupported-query`: the fact exists authoritatively but the public query
  cannot express or evaluate it.

The last two classes must not be collapsed. Only recurring `unsupported-fact`
outcomes can motivate a schema proposal; `unsupported-query` may instead
indicate a query-only experiment.

## Public schema 0.1 control

Executed before competitor import:

```text
$ atlas qualify graph.reachable_traversal --allocation none
<no candidates>

$ atlas qualify priority_queue.push --in-place
implementation  priority_queue.push.rust.std.1_85
algorithm       priority_queue.binary_heap.push
in_place        true  declared  docs:phase2/k-m2-dynamic-structures.md
allocation      may grow and reallocate state storage when capacity is exhausted

$ atlas qualify stream.top_k --allocation none
<no candidates>
```

The control can already discover a foreign candidate and evaluate three fixed
property/effect predicates. It cannot ask exactness, output order, conditioned
cost, state compatibility or an asymptotic threshold. Empty output means no
match; it does not explain whether facts are absent or contradictory.

## Source and implementation acceptance

A Phase 4 competitor is source-independent when its algorithm-bearing text or
code comes from a named external primary or maintained project source that is
not derived from Atlas or the current candidate's Atlas worksheet.

For every imported implementation, record:

- upstream project, exact version or commit and license;
- exact entry point and target assumptions;
- whether Atlas calls upstream code, writes a thin adapter, translates an
  algorithm, or provides only a behavioral fixture;
- every semantic specialization or correction;
- tests of the problem contract, including boundary cases;
- separate sources for theoretical claims that tests cannot establish.

Using the same external project for two strategies is allowed only if the
algorithm sources are independently reviewable; it weakens source-diversity
evidence and must be stated in the batch result.

## Falsifiers and stop conditions

Automatic discovery is falsified if adding a conforming manifest requires a
candidate ID or source-family branch in selection code. Shared qualification is
falsified if equivalent requests need incompatible family-specific meanings.
Schema sufficiency is falsified when a frozen request is `unsupported-fact`.

Stop a corpus milestone after one reviewed competitor, its candidate matrix and
cost record. Do not add more entries to manufacture convergence.

## Open next decision

K4-M1 source review is orthogonal and may proceed. B2, the phase-local
qualification representation, remains open until the graph baseline and new
candidate reveal whether reuse of K-M5 is smaller and clearer than a disposable
matrix evaluator.
