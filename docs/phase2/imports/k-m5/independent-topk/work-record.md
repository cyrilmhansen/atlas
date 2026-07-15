# K-M5 Independent Author Work Record

Author identifier: `km5-independent-topk`

Externally observed start time: `2026-07-15T02:45:17+02:00`

Externally observed end time: `2026-07-15T02:47:44+02:00`

## Scope and process

I inspected only `PACKET.md` and the six mandatory files in this isolated
directory. I did not inspect Git history, the reference overlay, evaluator
source or tests, another submission, another directory, or the Internet. I
verified all six mandatory-file SHA-256 digests against `PACKET.md` before
authoring. The tools used were `pwd`, `rg --files`, `sed`, `rg`, `wc`, `nl`, and
`sha256sum`, plus the patch-writing tool used to create the two requested files.

## Intervention

One clarification was requested because
`docs/phase2/k-m5-decision-overlay.md:38-73` demonstrates only empty `costs` and
`maximum_costs` lists and therefore does not state the fields of nonempty cost
items. At `2026-07-15T02:46:22+02:00`, with orchestrator handling time reported
as under one minute, the orchestrator supplied the candidate-cost and
request-maximum-cost parser shapes. The answer contained field structure only:
`operation`, `metric`, `regime`, `bound`, `requires`, and candidate `evidence`;
it did not supply reference IDs, taxonomy, facts, wording, or decisions. No
other intervention occurred.

## Request-ID mapping

| Submission request ID | Frozen semantic request |
|---|---|
| `request.empty_when_capacity_zero` | `zero-capacity` |
| `request.exact_with_bounded_retention` | `bounded-retained-memory` |
| `request.exact_without_allocation` | `allocation-free` |

## Fact provenance and rationale

### Identity

- `candidate.bounded_top_k.binary_heap` maps to
  `registry:stream.top_k.rust.std_binary_heap.v1`. The implementation identity
  and its `stream.top_k.min_heap` algorithm mapping are present at
  `registry/atlas.yaml:2644-2645`; its entry point and signature are at
  `registry/atlas.yaml:2652-2653`. The candidate ID is author-chosen and adds no
  second identity.

### Atoms and candidate facts

- `result.top_k.exact_occurrences` means the greatest `min(k, n)` input
  occurrences, with multiplicity and without an ordering promise. The source
  report states this contract at
  `docs/phase2/k-m3-streaming-approximation.md:44-46`, explicitly separates
  descending presentation from membership at lines 54-56, and the registry
  records the exact-occurrence and no-greater-omission guarantees with `tested`
  evidence at `registry/atlas.yaml:449-455`. The fixture at
  `crates/atlas/tests/external_streaming_adapters.rs:109-113` includes duplicate
  maxima and `k > n`. I therefore attached `tested` evidence to the capability.
- `condition.capacity.zero` means only the supplied request regime `capacity ==
  0`; it is not a candidate precondition. The registry includes capacity as an
  input and states the zero result at `registry/atlas.yaml:440-453`. The test at
  `crates/atlas/tests/external_streaming_adapters.rs:113` observes the empty
  result. The candidate's `requires` remains empty, preserving its wider input
  domain.
- `result.sequence.empty` is the exact empty-sequence output projection under
  `condition.capacity.zero`. The single `projects_to` relation is directional
  from exact top-k output to this narrower result and requires the zero-capacity
  condition. Its `tested` evidence is the assertion at
  `crates/atlas/tests/external_streaming_adapters.rs:113`, consistent with the
  registry guarantee at `registry/atlas.yaml:453-455`.
- `effect.allocates.retained_and_output_storage` means this implementation
  allocates both retained heap and output storage. The registry states
  "allocates at most k retained heap elements and at most k output elements"
  with `tested` evidence at `registry/atlas.yaml:2654-2657`. The code constructs
  a capacity-sized `BinaryHeap`, collects into a `Vec`, and sorts that output at
  `crates/atlas/tests/external_streaming_adapters.rs:4-19`. I preserved the
  registry's `tested` level rather than promoting it.
- The `bounded_top_k` worst `retained_memory` cost has the exact opaque bound
  `O(k) persistent retained elements`. This is the registry wording and evidence
  level at `registry/atlas.yaml:1257-1260`; the source report independently says
  the heap contains at most `k` elements and persistent storage is `O(k)` at
  `docs/phase2/k-m3-streaming-approximation.md:44-50`. Following protocol
  `docs/phase2/import-protocol-k-m0.3.md:57-58`, I retained `inferred` evidence.
  The operation string is author-chosen and is repeated exactly in its request.

### Operational requests

- `request.empty_when_capacity_zero` supplies the condition rather than
  requiring it of the candidate, accepts the conditioned empty projection, and
  admits `tested` evidence. It is intended to accept through the single
  relation.
- `request.exact_with_bounded_retention` accepts exact occurrence membership and
  exactly matches the candidate's worst retained-memory cost. It admits both
  `tested` for membership and `inferred` for the cost. It is intended to accept.
- `request.exact_without_allocation` accepts exact occurrence membership but
  forbids the candidate's allocation effect. It admits `tested` evidence and is
  intended to reject because that exact effect is present.

## Ambiguities and rejected alternatives

- The packet omitted the nonempty cost-item shape; this was resolved only by the
  parser-shape intervention recorded above.
- I rejected making zero capacity a candidate `requires` fact because the
  implementation also accepts nonzero capacities and the packet explicitly
  defines zero capacity as a conditioned output projection.
- I rejected encoding descending output order. The source explicitly calls it
  an adapter presentation choice rather than the universal top-k contract.
- I rejected a `guarantee` atom for exact membership because the request's
  selected observable output is directly modeled as a capability, matching the
  overlay's `accepts` field and avoiding duplicate facts.
- I rejected `declared` or `tested` evidence for the `O(k)` cost because the
  registry labels the algorithm claim `inferred`.
- I rejected an `allocation` cost item. Allocation prohibition is directly
  expressible with the supplied effect atom and `forbids_effects`; adding a
  second representation would be redundant and could change exact-cost
  matching.
- I rejected separate heap-allocation and output-allocation effects because the
  frozen request forbids allocation generally and the registry presents them as
  one tested implementation effect. One combined atom preserves both facts
  within the bounded experiment.

## Closed-vocabulary limits

The vocabulary cannot attach human-readable definitions to atoms, so exact
multiplicity semantics and the distinction between membership and descending
presentation remain documented here and source-resolved rather than structurally
decomposed. Cost bounds are opaque strings: `O(k)` cannot be related
algebraically to another bound or parameter constraint. The combined allocation
effect cannot structurally quantify heap versus output allocation or distinguish
capacity reservation from realized allocation. These limits do not prevent the
three frozen operational decisions.
