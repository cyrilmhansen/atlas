# Phase 2 manual import worksheet

Status: K-M0 pilot template  
Protocol revision: `k-m0.2` (frozen for the first corpus batches)

Complete one worksheet per algorithmic subject, comparing every mandatory
source assigned to that subject. Use plain Markdown. Do not edit the registry,
schema, AST or another importer's worksheet while completing it.
Write `not stated`, `not representable`, or `uncertain` instead of filling a gap
from general knowledge.

The source packet distinguishes mandatory pages, which every importer must
inspect, from supplemental pages. Supplemental pages may clarify a fact but
must be listed and kept separate during initial equivalence comparison. Do not
use such a fact to call another import divergent before both imports have been
adjudicated against that page.

## 1. Work record

- Importer identifier:
- Externally observed batch start and end time:
- Batch elapsed minutes:
- Active authoring minutes: observed or `unavailable`:
- Source-reading minutes: observed or `unavailable`:
- Atlas-modeling minutes: observed or `unavailable`:
- Human interventions and their duration:
- Tools used beyond browser, editor and existing Atlas CLI:

Batch elapsed time is recorded by the orchestrator, not reconstructed by the
importer. Activity subdivisions may be `unavailable`; retrospective estimates
must not support an authoring-cost conclusion. No per-field instrumentation is
required.

## 2. Source identity

- Source subject and source-local name:
- Authors or maintainers:
- Work or project title:
- Edition, release, tag or commit:
- Section, page, module or symbol:
- Stable URL(s):
- Retrieval date:
- Source class: book, library, paper, standard, or other:
- Code license:
- Documentation license or copyright status:
- Mandatory pages consulted:
- Supplemental pages consulted:

## 3. Source-faithful account

Describe using the source's own conceptual boundaries before Atlas
normalization.

- Problem stated by the source:
- Inputs and their representation:
- Preconditions and validity domain:
- Output or observable interface:
- Postconditions and guarantees:
- Strategy:
- Named invariants:
- Persistent and temporary state:
- Mutations, allocation, I/O and failure behavior:
- Time claims, including operation and case being bounded:
- Space claims, including what is excluded:
- Determinism, randomness or numerical assumptions:
- Variants explicitly distinguished by the source:
- Ambiguities or internally inconsistent statements:

For each substantive statement, cite its exact source locator and mark whether
it came from a mandatory or supplemental page. Do not assign an Atlas evidence
level yet.

## 4. Proposed Atlas normalization

This section is a proposal, not a manifest patch.

- Proposed `Problem` identity and reason:
- Proposed `Algorithm` identity and reason:
- Proposed implementation identity, if executable source is in scope:
- Existing Atlas entity that may be synonymous:
- Proposed problem `input`, `requires`, `output`, `ensures`:
- Proposed algorithm `requires`, determinism, complexity and memory claims:
- Proposed implementation effects and tests:
- Proposed evidence level for each claim and why:
- Candidate relationships to other imported subjects:
- Information intentionally left documentary:

If several normalizations are reasonable, list at most three and state which
one you would use for the comparison. Do not resolve ambiguity by multiplying
entities without explaining the selection consequence.

## 5. Fidelity assessment

Evaluate dimensions independently.

### Bibliographic fidelity

- Preserved identifiers and locators:
- Missing or unstable source identity:
- Assessment: preserved, partial, or unresolved:

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions:
- Semantic details lost or altered:
- Assessment: preserved, partial, incompatible, or unresolved:

### Representational fidelity

- Source vocabulary and decomposition retained:
- Normalized or collapsed concepts:
- Assessment: preserved, intentionally transformed, lossy, or unresolved:

### Executable fidelity

- Upstream implementation or examples available:
- Correction oracle proposed:
- Behavior actually checked during this worksheet: normally none in K-M0:
- Assessment: not assessed, supported, contradicted, or unresolved:

### Declared transformations

For each transformation record kind, input, output and reason:

- translation:
- specialization or generalization:
- type or representation adaptation:
- API decomposition or aggregation:
- bug correction:
- pedagogical simplification:
- other:

## 6. Model-friction record

Classify every material mismatch without changing public types:

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| | | exact / lossy / absent / ambiguous | identity / selection / substitution / composition / documentary only | worksheet / experimental annotation |

For each `lossy`, `absent` or `ambiguous` row, answer:

- Would two candidates become indistinguishable?
- Could an invalid candidate be selected?
- Could a valid substitution or composition be rejected?
- Is the issue specific to this family?
- What second structural family would be needed before a schema proposal?

AST representability is recorded here but is not an import acceptance gate.

## 7. Operational probes

Using only current Atlas behavior, state the expected result of:

- identity lookup:
- search by the proposed vocabulary:
- qualification by current properties:
- substitution between source variants:
- composition with a stated precondition/effect:

Mark each as `supported`, `unsupported`, or `would require source-specific
logic`. A hypothetical outcome must not be reported as an executed test.

## 8. Importer conclusion

- Recommended normalization:
- Unresolved questions:
- Decision-relevant loss:
- Documentary divergence that should remain acceptable:
- Proposed next minimal experiment:
- Schema or AST change requested now: must be `none` during the first pilot.
