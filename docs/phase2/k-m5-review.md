# K-M5 review - manifest-driven candidate discovery

Status: exit audit complete; mixed closure recommended

Date: 2026-07-15

## Verdict

K-M5 is **technically supported but product-level mixed**.

The private evaluator demonstrates generic, explainable selection over explicit
facts. Independent authoring and two structural-family equivalence tests show
that source-faithful encodings can be reconciled without candidate-specific
code. However, schema 0.1 manifests do not contain those facts: the disposable
overlay remains a second, non-authoritative authoring layer. The existing CLI
and composer therefore do not gain this selection capability.

## Evidence obtained

- 30 baseline atoms, 8 candidates, 4 directional relations and 10 adjudicated
  requests cover the seven ontology discriminants.
- Five baseline requests accept candidates and five explain sound rejection.
- One isolated author produced a new top-k overlay in 2 minutes 27 seconds; all
  three frozen operational results agreed.
- Two top-k equivalences reconcile fused/decomposed output and effect/cost
  encodings across four requests.
- One heap equivalence reconciles conditioned cost/guarantee encodings across
  four requests and preserves rejection without spare capacity.
- Candidate discovery and equivalence resolution contain no implementation ID
  or source-family branch.
- Mapping evidence and candidate evidence are checked separately; equivalences
  do not chain.

## Acceptance audit

| K-M5 criterion | Result |
|---|---|
| candidate list comes from data, not Rust IDs | pass for the private overlay |
| positive and negative decisions are explained | pass |
| effects, allocation and preconditions affect decisions | pass |
| a new candidate changes decisions without evaluator code | pass |
| source references resolve to registry or worksheets | pass |
| existing schema 0.1 manifests supply the required facts | fail |
| existing CLI/composer performs the new discovery | fail |
| public schema or query compatibility remains unchanged | pass |

The two failures are the intended falsification boundary, not missing polish.
Fixing them would require a schema 0.2 proposal and migration, a class C choice
explicitly excluded from K-M5.

## Cost

| Artifact | Size |
|---|---:|
| Private Rust model, validation and evaluation | 1,169 non-test lines |
| Baseline overlay | 319 YAML lines |
| Independent submission | 85 YAML lines |
| Top-k equivalence fixture | 119 YAML lines |
| Heap equivalence fixture | 117 YAML lines |
| Independent work record | 132 Markdown lines |
| Runtime dependencies, public commands, schema fields | 0 |

The implementation remains bounded, but its infrastructure-to-corpus ratio is
too high to justify further K-M5 ontology mechanics without a public-schema
decision.

## Recommendation

Close K-M5 as **mixed**, retain the private experiment as falsification evidence
and freeze further overlay/equivalence growth. Proceed to K-M6 using the
accepted existing CLI/text interface, deliberately without granting the agent
access to implementation source during selection.

K-M6 should measure whether current Atlas changes an agent's decisions despite
the known schema limitations. It must not introduce a structured agent API or
promote the overlay merely to improve the experimental outcome.

Alternatives:

- continue K-M5 into a third family: low expected information gain and growing
  private infrastructure;
- promote selected overlay fields to schema 0.2 now: premature class C change
  without a migration or stable authoring protocol;
- delete the experiment immediately: loses useful, reproducible falsification
  evidence without reducing public compatibility burden.
