# K-M5 independent top-k authoring result

Status: complete, operationally convergent and taxonomically mixed

Date: 2026-07-15

## Isolation record

The independent author received only the seven files copied into
`/tmp/atlas-km5-authoring-328d8c2`: the packet plus its six mandatory inputs.
All six input SHA-256 digests matched before and after authoring. The author was
instructed not to inspect the repository, Git history, Internet, reference
overlay or evaluator.

- author: `km5-independent-topk`;
- externally observed start: `2026-07-15T02:45:17+02:00`;
- externally observed end, recorded before inspection: `2026-07-15T02:47:44+02:00`;
- batch elapsed time: 2 minutes 27 seconds;
- intervention: one structure-only clarification, answered in under one minute;
- deliverables: 85-line YAML and 132-line work record.
- authored YAML SHA-256: `ca1ea0a8aae869c5a120014d4c0437b132b79b4729805a28432a9e212af8c6b3`.

The clarification was necessary because the frozen overlay specification named
cost metrics and regimes but showed only empty `costs` and `maximum_costs`
lists. The orchestrator supplied their parser field shapes without facts, IDs,
taxonomy or expected encodings. This is retained as protocol cost.

## Submitted model

The unmodified semantic submission contains:

- 4 atoms;
- 1 candidate resolved to `stream.top_k.rust.std_binary_heap.v1`;
- 1 conditioned `projects_to` relation;
- 3 requests;
- one worst-case retained-memory cost;
- one tested allocation effect.

The author correctly kept capacity zero out of candidate preconditions,
preserved the wider domain, excluded descending presentation from the semantic
contract and retained the source's `inferred` level for `O(k)` memory.

## Operational adjudication

The existing parser, source resolver and generic evaluator consumed the
submission without repair or source-specific branch.

| Frozen request | Expected | Observed |
|---|---|---|
| zero-capacity empty projection | accept | accept through the conditioned relation |
| exact result with worst retained memory `O(k)` | accept | accept through exact cost matching |
| exact result without allocation | reject | reject with the authored allocation effect named |

Operational agreement is 3/3. The independent test is committed next to the
generic evaluator; no evaluator production line changed.

## Equivalence assessment

| Dimension | Result | Reason |
|---|---|---|
| Identity | equivalent | the candidate resolves to the intended implementation |
| Semantic | equivalent for the three requests | multiplicity, zero capacity, retained memory and allocation are preserved |
| Taxonomic | mixed | exactness is fused into a capability; allocation is an effect rather than an allocation-cost fact |
| Operational | equivalent for the frozen oracle | all three accept/reject decisions agree |
| Broader substitution | unresolved | a request for a separate exact guarantee or allocation cost could decide differently |
| Documentary | acceptable | definitions and rejected encodings remain in the work record |

Two independently reasonable encodings therefore remain decision-relevant:

1. **Observable versus guarantee.** The author used
   `result.top_k.exact_occurrences` as one capability and no guarantee atom. The
   reference experiment generally separates output capability from
   `guarantee.exact`. A request requiring the latter would reject this candidate
   despite equivalent prose semantics.
2. **Allocation effect versus cost.** The author used an unconditional
   allocation effect. Existing reference facts use the allocation cost metric
   for capacity-conditioned absence. Both are defensible, but a request written
   against only one representation does not discover the other.

This is not a parser failure and should not be hidden by ID normalization. The
three-request oracle was insufficient to force these taxonomic distinctions.

## Gate result

The experiment supports the bounded evaluator and directional relation: a new
candidate was authored quickly and produced the intended decisions without code
changes. It does not yet support public ontology promotion because operational
agreement was obtained through a taxonomically underdetermined task.

K-M5 remains open. Before another authoring run, choose and document private
normalization rules for:

- separating observable capability from qualifying guarantee;
- using effects for unconditional behavior and cost facts for scoped or
  conditioned resource claims;
- writing at least one cross-encoding request that exposes either distinction.

This is a reversible experiment-protocol choice, not authorization to change
schema 0.1.
