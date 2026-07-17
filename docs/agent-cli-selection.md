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
