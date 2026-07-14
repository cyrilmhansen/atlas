# K-M4-W import A: incremental second central moment

Protocol: `k-m0.2`

## 1. Work record

- Importer identifier: `importer-a`
- Externally observed batch start: `2026-07-15T01:41:11+02:00` (first timestamp captured by the importer; orchestrator start unavailable)
- Externally observed batch end: recorded in the final timestamp block below
- Batch elapsed minutes: unavailable; no orchestrator duration was exposed
- Active authoring minutes: unavailable
- Source-reading minutes: unavailable
- Atlas-modeling minutes: unavailable
- Human interventions and their duration: none
- Tools used beyond browser, editor and existing Atlas CLI: `curl`, `sha256sum`, `pdfinfo`, `pdftotext`, `sed`, `stat`, `date`
- Source retrieval timestamps: Pebay PDF `2026-07-15T01:41:19.716938294+02:00`; Chan-Golub-LeVeque PDF `2026-07-15T01:41:23.535910763+02:00`
- Local locators actually read: `docs/phase2/k-m4-w-source-packet.md`; `docs/phase2/import-worksheet.md`; `docs/phase2/import-equivalence-rubric.md`; `docs/phase2/current-model-baseline.md`; `docs/schema-0.1.md`
- Local locator allowed but not read: `docs/vision.md`

## 2. Source identity

### Pebay 2008

- Source subject and source-local name: centered statistical moments; `M2,S` is the sum of squared deviations from the mean
- Author: Philippe Pebay
- Work title: *Formulas for Robust, One-Pass Parallel Computation of Covariances and Arbitrary-Order Statistical Moments*
- Edition/release: Sandia report SAND2008-6212, Unlimited Release, September 2008
- Sections/pages/items: printed pp. 7-11; equations (1.1)-(1.4), Proposition 2.1, Corollary 2.2, Remark 2.1
- Stable URL actually read: <https://www.osti.gov/servlets/purl/1028931/>
- Retrieval date: 2026-07-15
- Retrieved SHA-256: `db5dfcd01b3bdd21071930fb14a772968dedb33e062286d408844f3c7fa0c297` (matches frozen packet)
- Source class: government technical report/paper
- Code license: not stated on mandatory pages; the VTK implementation is mentioned but was not read
- Documentation license or copyright status: marked `Unlimited Release`; no license grant was identified on the mandatory pages
- Mandatory pages consulted: all assigned printed pp. 7-11
- Supplemental pages consulted: none

### Chan, Golub and LeVeque 1979

- Source subject and source-local name: updating formulae and a pairwise algorithm for the sample sum of squares `S`
- Authors: Tony F. Chan, Gene H. Golub and Randall J. LeVeque
- Work title: *Updating Formulae and a Pairwise Algorithm for Computing Sample Variances*
- Edition/release: Stanford technical report STAN-CS-79-773, November 1979
- Sections/pages/items: report sections 1-3, assigned PDF pp. 5-10; definitions (1.1), updating formulas (1.5)-(1.6), combination formulas (2.1), pairwise algorithm
- Stable URL actually read: <https://ftpmirror.your.org/pub/misc/bitsavers/pdf/stanford/Stanford_CS_TR_Collection_2025-12-12/OCR/CS-TR-79-773-ocr.pdf>
- Retrieval date: 2026-07-15
- Retrieved SHA-256: `07ee77389bfafb982198b29275d3ee5ef8e1fab21698d06f03a557620f43574e` (matches frozen packet)
- Source class: archival technical report/paper
- Code license: not stated; the report's Fortran is not treated as licensed software and was not read
- Documentation license or copyright status: not stated on mandatory pages
- Mandatory pages consulted: all assigned PDF pp. 5-10
- Supplemental pages consulted: none

## 3. Source-faithful account

### Pebay

- Problem stated by the source: compute centered moments in one pass, permit incremental updates as observations arrive, and permit final aggregation of moments computed independently on partitions. [mandatory pp. 7-10, introduction and Proposition 2.1]
- Inputs and representation: a finite data set `S`; for incremental update, `S = S1 union {y}`, prior cardinality `n-1`, prior mean `mu1`, prior `M2,S1`, and new value `y`; for pairwise update, a partition `{S1,S2}`, counts `n1,n2`, means `mu1,mu2`, and centered sums. [mandatory pp. 7-10, eqs. (1.1)-(1.4), Proposition 2.1]
- Preconditions and validity domain: the displayed mean recurrence divides by the new count `n`; Proposition 2.1 requires natural order `p > 1`; pairwise formulas divide by total `n`. The pages do not define the mean of an empty set or a zero-total-count merge. [mandatory pp. 7, 8, 10]
- Output/interface: updated mean and `M2,S = sum[x in S] (x-mu)^2`; higher moments are outside this worksheet's normalized subject. [mandatory pp. 7-10, eqs. (1.1)-(1.4), (2.1), (2.9)]
- Exact-arithmetic postconditions: increment uses `mu = mu1 + (y-mu1)/n` and `M2,S = M2,S1 + (y-mu1)(y-mu)`; pairwise combination uses `mu = mu1 + n2(mu2-mu1)/n` and `M2,S = M2,S1 + M2,S2 + n1*n2*(mu2-mu1)^2/n`. The source identifies the singleton specialization of pairwise combination with the incremental formulas. [mandatory pp. 7-8, eqs. (1.1)-(1.4); pp. 10-11, Corollary 2.2 and Remark 2.1]
- Finalization: the unbiased estimator is `M2,S/(n-1)`. This is a separately derived result, not the maintained state. The mandatory pages do not define it for `n < 2`. [mandatory p. 8, text following eq. (1.2)]
- Strategy and state: retain count, mean, and centered sums; update on each arrival, or compute partition states independently and aggregate them once. [mandatory pp. 7-10]
- Named invariants: `M2,S` is the corrected sum of squared deviations about the current mean; `M1` is zero by definition; `M0` is cardinality in the proof of Proposition 2.1. [mandatory pp. 7, 10, proof of Proposition 2.1]
- Mutations/allocation/I/O/failure: not stated. The recurrence implies replacement of a retained state, but no API or failure behavior is specified.
- Time/space: the report calls the incremental method one-pass and describes final pairwise updates as negligible-cost relative to distributed access, but gives no asymptotic bound for the second-moment recurrence on these pages. O(1) arithmetic per arrival, O(N) total time, and O(1) retained scalar state are inferences from equations (1.1)-(1.2), not quoted claims. [mandatory pp. 7-8]
- Determinism/numerical assumptions: formulas are deterministic algebraic recurrences. The report motivates them as robust and as numerically stable as possible, but the mandatory pages provide no universal IEEE-754 error bound or bitwise-order guarantee. [mandatory pp. 7-8]
- Variants explicitly distinguished: two-pass centered computation; unstable one-pass sum-of-powers computation; incremental centered-moment update; pairwise/parallel centered-moment update; higher-order and covariance updates. [mandatory pp. 7-11]
- Ambiguities: `robust` is not quantified on the assigned pages (source ambiguity). Empty-state initialization and floating-point evaluation order are not specified (source ambiguity).

### Chan, Golub and LeVeque

- Problem stated by the source: compute the sum of squared deviations `S` for `N` data points, especially when two passes or retaining all data is undesirable, and combine independently computed samples. [mandatory PDF pp. 5-9, report sections 1-2]
- Inputs and representation: data `z_i`; sample count; running sum `T`; sum of squares `S`; for combination, two sample sizes, sums and corrected sums of squares. [mandatory PDF pp. 5, 7-8, definitions (1.1), (1.5)-(1.6), (2.1)]
- Preconditions and validity domain: the updating loop starts from a sample of size one; a singleton has `S=0`. Empty input behavior is not specified. [mandatory PDF pp. 7-8]
- Output/interface: `S = sum_i (z_i - mean)^2`; variance scaling is not part of the maintained updating state in the displayed algorithms. [mandatory PDF p. 5, definition (1.1)]
- Exact-arithmetic postconditions: formula (1.6) updates the corrected sum of squares when one observation is added; formulas (2.1) combine the sums and corrected sums for arbitrary sample sizes, and reduce to the one-new-point formula when one side is a singleton. [mandatory PDF pp. 7-8]
- Strategy and state: the sequential algorithm repeatedly applies the singleton update. The pairwise algorithm recursively splits samples, computes partial sums of squares, and combines them with (2.1); its streaming tableau implementation makes one pass and retains O(log2 N) intermediate locations. [mandatory PDF pp. 7, 9-10, sections 1 and 3]
- Named invariants: the updating method adds a nonnegative quantity to the prior `S`, in contrast with the cancellation-prone textbook rearrangement. [mandatory PDF p. 7]
- Mutations/allocation/I/O/failure: assignment to running `T` and `S` is explicit in (1.5); pairwise intermediate locations or a stack are described. I/O and failure behavior are not stated. [mandatory PDF pp. 7, 9-10]
- Time/operation claims: the modified two-pass method adds `N` additions and two multiplications to the stated two-pass cost of `3N-2` additions and `N-1` multiplications. No exact operation count for the normalized singleton update is stated on the mandatory pages. O(N) update time is inferred from loop (1.5). [mandatory PDF pp. 6-7]
- Space claims: the pairwise tableau needs one pass and O(log2 N) intermediate storage; grouping with size `m>1` retains `m` data values but is, strictly, no longer one-pass. The singleton update loop has constant scalar state by inspection, an inferred rather than stated bound. [mandatory PDF pp. 8-10]
- Determinism/numerical assumptions: the displayed algorithms are deterministic for a fixed evaluation order. The textbook rearrangement can suffer cancellation; the updating method generally performs comparably to the two-pass method; experiments suggest pairwise processing can be more accurate. The source explicitly says it lacks a satisfactory error analysis for one grouped variant. None of these statements guarantees bitwise equality across IEEE-754 evaluation orders. [mandatory PDF pp. 5-9]
- Variants explicitly distinguished: ordinary two-pass, corrected two-pass, textbook one-pass, singleton updating, grouped updating, and recursive pairwise computation. [mandatory PDF pp. 5-10]
- Ambiguities: the assigned pages do not give total empty-input semantics or an IEEE-754 equivalence theorem (source ambiguity). OCR defects affect typography but the relevant conceptual distinctions are readable (source-document ambiguity, not normalized into formulas from OCR alone).

### Domain distinctions

| Quantity | Domain proposed from the mandatory sources | Empty | Singleton |
|---|---|---|---|
| Maintained count | all processed prefixes | `0` can be an implementation initialization, but is not a source-defined complete state | `1` |
| Mean | nonempty prefixes | undefined/not stated | the observation |
| Corrected sum of squares `M2`/`S` | source-defined for nonempty samples | not stated | `0` [Pebay mandatory p. 10; Chan et al. mandatory PDF p. 8] |
| Population variance `M2/n` | proposed only for `n >= 1` | undefined | `0` |
| Unbiased sample variance `M2/(n-1)` | `n >= 2` | undefined | undefined |

Population variance is an explicit normalized finalization needed to preserve the requested distinction, but `M2/n` is not stated on the assigned pages. It is therefore an inferred, separately qualified operation, not a source claim. Unbiased sample variance is explicitly stated by Pebay and remains partial at `n < 2`.

## 4. Proposed Atlas normalization

### Recommended identities

- Proposed `Problem` ID: `online_second_central_moment_state`
- Identity reason: the contract is maintaining a sufficient state, not returning a variance under one denominator convention. It keeps corrected sum of squares distinct from population and unbiased sample variance.
- Proposed `Algorithm` ID: `incremental_centered_second_moment`
- Algorithm name: `Incremental centered second-moment update`
- Identity reason: source-neutral and faithful to the recurrence without attributing it to Welford. It distinguishes arrival-by-arrival update from partition merging.
- Proposed related `Algorithm` ID: `pairwise_centered_second_moment_combine`
- Pairwise decision: **related algorithm**, not the same algorithm, not merely a stage, and not only documentary capability. It has a distinct input contract (two accumulated states), enables parallel/tree evaluation, and may use O(log N) intermediate storage in Chan et al.'s one-pass tableau. The singleton specialization establishes a derivation relationship that schema 0.1 cannot encode structurally.
- Proposed implementation identity: none; no executable source was in scope.
- Existing Atlas entity that may be synonymous: uncertain; the isolation boundary forbids registry inspection.

### Exact proposed problem contract

- `input`: `a finite, nonempty sequence of real-valued observations presented in encounter order`
- `requires`: `the arithmetic operations and divisions used by the recurrence are defined for the chosen numeric representation`
- `output`: `a state (n, mean, M2) with integer n, mean, and corrected sum of squared deviations M2`
- `ensures`: `in exact arithmetic, n equals the input length, mean equals the arithmetic mean of the observations, and M2 equals the sum over observations of (x - mean)^2`

This contract is deliberately nonempty. A total API may expose an empty state `(0, no mean, 0)`, but the `no mean` representation and empty `M2` convention are not supplied by the mandatory sources and must be declared as an implementation adaptation.

### Proposed incremental algorithm claims

- `requires`: prior state represents a nonempty prefix; the new observation is representable; new count and intermediate arithmetic do not leave the implementation's valid numeric domain
- Determinism: deterministic for a fixed numeric representation and fixed encounter/evaluation order; no claim of equality across different floating-point orders
- `time_worst`: O(N) arithmetic operations for N observations, inferred from one constant-size recurrence per observation
- `auxiliary_memory`: O(1) retained scalar state for the incremental algorithm, inferred from `(n, mean, M2)`
- Initialization: initialize from the first observation as `(1, x, 0)`; this is a type/representation adaptation supported by the sources' singleton facts, not a claim that their recurrence accepts an empty prior state
- Increment step in exact arithmetic: with `n = n_old + 1`, set `delta = x - mean_old`, `mean_new = mean_old + delta/n`, and `M2_new = M2_old + delta*(x - mean_new)`
- Finalizations: population variance is available only for `n >= 1` as inferred `M2/n`; unbiased sample variance is available only for `n >= 2` as source-declared `M2/(n-1)`
- Evidence: equations/contracts and domain facts are `declared`; asymptotic incremental time/memory and population finalization are `inferred`; no claim is `tested`, `observed`, or `proven` by this worksheet

### Alternatives considered

1. Recommended: one state-maintenance problem, with distinct incremental and pairwise algorithms related by singleton specialization.
2. Alternative: one broad centered-moment algorithm with incremental and pairwise modes. Rejected because it collapses different inputs, memory regimes and selection consequences.
3. Alternative: a variance-returning problem with denominator as a mode. Rejected because it hides `M2`, makes singleton validity mode-dependent, and weakens source fidelity.

### Candidate relationships and documentary information

- Relationship: pairwise combination specializes to incremental update when one state represents a singleton. [Pebay eqs. (1.3)-(1.4), Corollary 2.2, Remark 2.1; Chan et al. (2.1)]
- Relationship: population and unbiased variance are finalizations of `M2`, not aliases for the maintained state.
- Intentionally documentary: authorship, report identifiers, equation derivation, error motivation, source-local `S`/`T` notation, empirical accuracy discussion, and the unquantified term `robust`.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: both report identifiers, titles, authors, dates, exact URLs, hashes, mandatory pages, and equation/item locators
- Missing or unstable identity: CGL copy is an archival mirror; no source license was established. Pebay's official URL and hash were verified.
- Assessment: preserved for citation identity; licensing remains unresolved

### Algorithmic fidelity

- Preserved: one-pass incremental recurrence, `(n, mean, M2)` invariant, singleton base, pairwise combination boundary, partial variance finalizations, and numerical-claim qualifications
- Altered/lost: an explicit typed empty-state wrapper is not source-defined; higher moments and covariance are intentionally outside the subject
- Assessment: preserved for nonempty second-moment maintenance; explicit adaptation required for an empty API state

### Representational fidelity

- Retained: `M2`/sum-of-squares distinct from variance, incremental versus pairwise decomposition, and source-local terms in documentary text
- Normalized: Chan et al.'s running sum `T` may be represented as mean; `S` and `M2` are treated as equivalent exact-arithmetic corrected sums for the same sample
- Assessment: intentionally transformed, with no denominator collapse

### Executable fidelity

- Upstream implementation/examples: Pebay mentions VTK; Chan et al. mention Fortran, but neither implementation was in scope or read
- Correction oracle: exact-arithmetic comparison with a directly computed mean and corrected sum on small nonempty rational inputs; separate permutation/order probes for floating-point non-equivalence
- Behavior checked during worksheet: none
- Assessment: not assessed

### Declared transformations

- Translation: `S` and `M2,S` -> `M2`, because both are defined as the corrected sum of squared deviations about the sample mean.
- Specialization: Pebay arbitrary-order formulas -> order `p=2`, the frozen subject.
- Type/representation adaptation: `(count, running sum T, S)` -> `(count, mean, M2)`; exact arithmetic preserves the state meaning, while floating-point traces need not be bitwise identical.
- Type/representation adaptation: first observation -> explicit `(1, x, 0)` initializer; avoids applying a formula whose prior mean is undefined.
- API decomposition: maintained state -> separately guarded population and unbiased-sample finalizers.
- API decomposition: arrival update and two-state merge -> distinct related algorithms.
- Bug correction: none.
- Pedagogical simplification: omit higher moments, covariance, Fortran, and derivation details outside the frozen subject.
- Other: no use of `Welford` as normalized identity; no inaccessible text is attributed.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Stateful transition `(n,mean,M2)+x -> state` | problem/algorithm prose | lossy | selection, composition | worksheet/experimental annotation |
| Pairwise algorithm derives incremental update by singleton specialization | no relationship field | absent | identity, substitution | worksheet/experimental annotation |
| Exact arithmetic versus IEEE-754 order dependence | requirements/determinism prose | lossy | selection, substitution | worksheet/experimental annotation |
| Population and unbiased finalizers have different domains | problem ensures/requires prose | lossy | selection | worksheet/experimental annotation |
| O(1) incremental state versus O(log N) pairwise tableau | separate algorithm memory claims | exact only if identities remain separate | selection | worksheet |
| Bibliography, pages, equations and terminology aliases | textual source expression/worksheet | lossy | documentary only | worksheet |
| No executable artifact read | implementation collection | exact absence | substitution | worksheet |

For lossy/absent/ambiguous rows:

- Candidates can become indistinguishable if incremental and pairwise identities are collapsed, or if `M2`, population variance, and unbiased sample variance are named merely `variance`.
- An invalid candidate could be selected for `n=0` or `n=1`, for an O(1)-state constraint, or for a bitwise reproducibility constraint.
- A valid substitution/composition could be rejected because the singleton-specialization relationship and merge state contract are not structural.
- Arithmetic-domain and streaming-state friction is family-relevant; relationship absence is broader, but this worksheet alone does not establish a second structurally independent family.
- Before any schema proposal, a second family involving partial finalizers and order-sensitive state composition, independently imported under the same protocol, would be needed.

Information schema 0.1 cannot represent without lossy prose: optional/undefined mean for an empty state; a typed persistent state transition; integer-count/real-accumulator structure; exact versus machine arithmetic domains; overflow/NaN/infinity policy; rounding-order sensitivity; guarded finalization by `n`; merge algebra and singleton-specialization relationship; parameterized per-update and total complexity; and structured bibliographic/transformation provenance.

Ambiguity classification:

- Source ambiguity: empty input, non-finite numeric values, overflow, exact IEEE-754 evaluation order, and quantified meaning of `robust` are unstated on mandatory pages.
- Protocol ambiguity: the worksheet requests exact problem fields while the source does not totalize empty input; this import chooses a nonempty problem contract and documents the possible wrapper rather than inventing semantics.
- Model ambiguity/insufficiency: schema 0.1 cannot structurally distinguish arithmetic domains, partial finalizers, state transitions, or algorithm derivation relationships.

## 7. Operational probes

These are expected outcomes from the frozen baseline, not executed CLI tests.

- Identity lookup: **supported** only after a future registry entity exists; exact ID lookup can distinguish the proposed incremental and pairwise IDs.
- Search by proposed vocabulary: **supported** for literal modeled names/prose, but source aliases such as `sum of squares`, `corrected sum`, `M2`, and `updating formula` are not structurally linked.
- Qualification by current properties: **unsupported** for one-pass, arithmetic domain, partial empty/singleton behavior, mergeability, and bitwise-order guarantees; complexity prose alone is not a generic typed predicate.
- Substitution between source variants: **would require source-specific logic** to use the exact-arithmetic equivalence while refusing an IEEE-754 bitwise-equivalence claim.
- Composition with a stated precondition/effect: **would require source-specific logic** because generic composition discovery and persistent typed state transitions are absent from the baseline.

### Common selection requests

1. **Accept conditionally**: "Consume a finite real-valued stream once and return its count, mean and corrected sum of squared deviations using O(1) retained scalar state." Accept for a finite **nonempty** stream and a numeric domain in which every recurrence operation is defined. Pebay equations (1.1)-(1.2) and Chan et al. loop (1.5)-(1.6) support the one-pass state; O(1) retained scalars is inferred by inspection. Reject an unqualified empty-stream reading because neither mandatory source defines its mean.
2. **Reject**: "Return an unbiased sample variance for every input length, including empty and singleton streams, without a partial or error result." Pebay gives `M2/(n-1)`; it is undefined at `n=1`, and both sources leave the empty mean/state undefined. A total numeric result would invent a convention not backed by the packet.
3. **Reject**: "Merge independently accumulated partitions and require bit-for-bit equality with every sequential evaluation order under IEEE-754 arithmetic." The sources give exact-algebraic combination identities and numerical motivation, not an IEEE-754 order-independence theorem. Different association/evaluation orders are not guaranteed bitwise equal; no empirical accuracy statement is promoted to a universal guarantee.

## 8. Importer conclusion

- Recommended normalization: `online_second_central_moment_state` solved by `incremental_centered_second_moment`, with `pairwise_centered_second_moment_combine` as a distinct related algorithm and variance estimators as guarded finalizations.
- Three most decision-relevant choices: (1) do not name the normalized algorithm Welford; (2) separate incremental update from pairwise combination while recording singleton specialization; (3) make the maintained object `M2`, with population and unbiased sample variance as distinct domain-guarded finalizations.
- Unresolved questions: desired empty-state API; allowed numeric representations/non-finite values; quantified floating-point error; how a future model should express derived algorithm relationships.
- Decision-relevant loss: arithmetic domain, partiality, state transition and merge relationship are only prose in schema 0.1.
- Documentary divergence acceptable: `S` versus `M2`, running sum versus mean, and differing numerical motivation, provided exact contracts and domains remain explicit.
- Proposed next minimal experiment: compare incremental and balanced pairwise evaluation on the same small floating-point sequence and verify exact-arithmetic equality separately from bitwise results; preserve this as experimental evidence, not a source theorem.
- Schema or AST change requested now: **none**. No public schema change is requested.

## Final timestamp

- Batch end: `2026-07-15T01:44:15+02:00`
