# Public CLI selection experiment

Date: 2026-07-17

## Question

Can an independent consumer choose implementations across three unrelated
families using only the public Atlas CLI, without reading source or registry
files and without inventing comparisons between opaque bounds?

## Frozen requests and oracle

1. Priority-queue push: worst time exactly `O(log n)`, with
   `state.spare_capacity`.
   Expected implementations: `priority_queue.push.rust.std.1_85` and
   `priority_queue.push.dary_heap.quaternary.0_3_9`.
2. Associative-map insert: expected time exactly `O(1)`, with
   `workload.nonadversarial_hash_distribution`.
   Expected implementation: `associative_map.insert.hashbrown.0_17_1`.
3. Exact stream top-k: worst auxiliary memory exactly
   `O(k) persistent retained elements`.
   Expected implementation: `stream.top_k.rust.std_binary_heap.v1`.

The consumer must also state that the third result does not prove domination
over the distinct `O(k) retained elements, with capacity up to 2k` profile.

## Protocol

The consumer receives the three requests and the `atlas qualify` syntax. It may
execute `target/debug/atlas` from the repository root, but may not inspect any
source, manifest, generated data or documentation file. Its reported candidate
sets and interpretation are compared with the frozen oracle above.

## Result

The independent consumer returned the exact frozen candidate sets:

- both binary and quaternary heap pushes, with the inferred cost source and the
  declared spare-capacity condition source;
- the hashbrown insert implementation, with the declared expected-cost and
  workload-condition sources;
- only the standard-library heap top-k implementation for the requested exact
  auxiliary-memory string.

It explicitly refused to infer domination over the distinct relaxed-selection
memory profile because Atlas treats both bounds as opaque strings.

Verdict: supported. The public CLI is sufficient for an independent consumer to
apply exact qualified selections across these three families without private
overlay access. This experiment supplied Atlas IDs and exact bound strings; it
does not test translation from an ordinary natural-language requirement.

## Natural-language follow-up

Question: can a fresh consumer translate an ordinary heap requirement into the
exact public query using only Atlas discovery commands?

Prompt: "A maximum-priority queue receives one new item. Its backing storage is
guaranteed to have room for that insertion. Find every implementation whose
worst-case insertion time is logarithmic. Report the Atlas query, candidates,
evidence and any assumption that Atlas does not verify itself."

The consumer may use only `target/debug/atlas search`, `show`, `explain` and
`qualify`; it receives no Atlas IDs, bound strings or condition IDs. It may not
inspect repository files. The frozen oracle is the two priority-queue push
implementations selected by worst time `O(log n)` under
`state.spare_capacity`. It must state that the caller, not Atlas, establishes
that condition.

Result: the fresh consumer discovered `priority_queue.push`, both heap
algorithms, the exact `O(log n)` bound and `state.spare_capacity`, then produced
the frozen qualification query and candidate set. It reported the inferred cost
source, declared condition source and correctly stated that the user, not
Atlas, establishes available capacity. It also noted the unconditional `O(n)`
growth case.

Verdict: supported with discovery friction. The translation from ordinary prose
to an exact qualified selection succeeded without repository access. However,
the consumer needed multiple exploratory commands because public `search` and
`show` do not expose Condition entities directly; the condition was discoverable
only inside Algorithm cost profiles and qualification output.

### Discovery remediation

The existing `search` and `show` commands were extended to include public
Condition entities and their sourced statements. A fresh consumer then repeated
the same natural-language task. It discovered `state.spare_capacity` with
`search`, inspected its statement directly with `show`, reproduced the same two
candidates and again assigned capacity truth to the caller.

Verdict: supported. Conditions are now directly inspectable without changing
the historical three-kind `list` surface or the exact selection semantics.
