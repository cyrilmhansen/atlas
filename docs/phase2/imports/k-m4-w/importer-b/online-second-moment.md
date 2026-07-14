# K-M4-W online second moment import worksheet

Protocol revision: `k-m0.2`

## 1. Work record

- Importer identifier: `importer-b`
- Externally observed batch start and end time: unavailable; no orchestrator timestamps were exposed to the importer.
- Importer-local observation window: `2026-07-15T01:41:16+02:00` to `2026-07-15T01:44:57+02:00` (Europe/Paris).
- Batch elapsed minutes: unavailable from the orchestrator; the importer-local observation window spans 3 minutes 41 seconds and is not a substitute for batch elapsed time.
- Active authoring minutes: unavailable.
- Source-reading minutes: unavailable.
- Atlas-modeling minutes: unavailable.
- Human interventions and their duration: none observed.
- Tools used beyond browser, editor and existing Atlas CLI: `curl`, `sha256sum`, `pdfinfo`, and `pdftotext`.

## 2. Source identity

### Source A: Pébay 2008

- Source subject and source-local name: centered statistical moments; incremental (or online) and pairwise update formulas for `M_{p,S}`.
- Authors or maintainers: Philippe Pébay.
- Work or project title: *Formulas for Robust, One-Pass Parallel Computation of Covariances and Arbitrary-Order Statistical Moments*.
- Edition, release, tag or commit: Sandia report SAND2008-6212, Unlimited Release, September 2008.
- Section, page, module or symbol: sections 1-2; equations (1.1)-(1.4); Proposition 2.1; Corollary 2.2; Remark 2.1.
- Stable URL(s) actually read: <https://www.osti.gov/servlets/purl/1028931/>.
- Retrieval date and time: `2026-07-15T01:41+02:00` (minute-resolution importer observation).
- Retrieved artifact SHA-256: `db5dfcd01b3bdd21071930fb14a772968dedb33e062286d408844f3c7fa0c297`, matching the frozen packet.
- Source class: paper (official public technical report).
- Code license: not applicable to the formulas; the mandatory pages mention a VTK implementation but no implementation or license was inspected.
- Documentation license or copyright status: not stated on the mandatory pages; public readability is not treated as a permissive license.
- Mandatory pages consulted: printed pages 7-11 (PDF pages 7-11), mandatory.
- Supplemental pages consulted: none.

### Source B: Chan, Golub and LeVeque 1979

- Source subject and source-local name: the sum of squares `S`; updating formulae and a pairwise algorithm for computing sample variances.
- Authors or maintainers: Tony F. Chan, Gene H. Golub, and Randall J. LeVeque.
- Work or project title: *Updating Formulae and a Pairwise Algorithm for Computing Sample Variances*.
- Edition, release, tag or commit: Stanford technical report STAN-CS-79-773, November 1979.
- Section, page, module or symbol: sections 1-3; definition (1.1); formulas (1.5)-(1.6), (2.1); pairwise algorithm.
- Stable URL(s) actually read: <https://ftpmirror.your.org/pub/misc/bitsavers/pdf/stanford/Stanford_CS_TR_Collection_2025-12-12/OCR/CS-TR-79-773-ocr.pdf>.
- Retrieval date and time: `2026-07-15T01:41+02:00` (minute-resolution importer observation).
- Retrieved artifact SHA-256: `07ee77389bfafb982198b29275d3ee5ef8e1fab21698d06f03a557620f43574e`, matching the frozen packet.
- Source class: paper (readable archival technical-report copy).
- Code license: not applicable; the report mentions a sample Fortran routine outside the mandatory pages, but it was not read or imported.
- Documentation license or copyright status: not stated on the mandatory pages; archival readability is not treated as a permissive license.
- Mandatory pages consulted: PDF pages 5-10, containing report sections 1-3 and printed report pages 1-6, mandatory.
- Supplemental pages consulted: none.

## 3. Source-faithful account

### Pébay 2008

- Problem stated by the source: calculate centered statistical moments robustly and efficiently in one pass, including distributed partial computations followed by aggregation. The second centered moment is `M_{2,S} = sum_{x in S}(x - mu)^2`. [Pébay 2008, mandatory pp. 7-10, eqs. (1.2), (2.1)]
- Inputs and their representation: a finite data set `S`; for an incremental update, an existing finite data set `S1` of cardinality `n - 1` and one additional value `y`; for a pairwise update, a partition `{S1, S2}` with cardinalities `n1`, `n2`, means `mu1`, `mu2`, and centered moments. [Pébay 2008, mandatory pp. 7-10, eqs. (1.1)-(1.4), Proposition 2.1]
- Preconditions and validity domain: Proposition 2.1 takes natural order `p > 1` and finite sets; the displayed mean and update denominators require the relevant combined cardinality to be nonzero. The source says `S1` may have non-negative cardinality, but does not define `mu1` for empty `S1`; initialization from the empty set is therefore source-ambiguous. [Pébay 2008, mandatory pp. 7, 10, eq. (1.1), Proposition 2.1]
- Output or observable interface: updated mean and corrected sum of powers; at order two, updated `(mu, M2)`. The unbiased variance estimator is separately obtained as `M2/(n - 1)`. [Pébay 2008, mandatory pp. 7-8, eqs. (1.1)-(1.4)]
- Postconditions and guarantees in exact arithmetic: for adding `y`, `mu = mu1 + (y - mu1)/n` and `M2,S = M2,S1 + (y - mu1)(y - mu)`; for two nonempty partitions, with `delta = mu2 - mu1` and `n = n1 + n2`, `mu = mu1 + n2*delta/n` and `M2,S = M2,S1 + M2,S2 + n1*n2*delta^2/n`. [Pébay 2008, mandatory pp. 7-8, eqs. (1.1)-(1.4)]
- Strategy: maintain or independently compute centered moments, then use recurrence formulas for a singleton update or a general pairwise update. Proposition 2.1 generalizes pairwise updates to arbitrary order; Corollary 2.2 specializes it to a singleton. [Pébay 2008, mandatory pp. 8-11, Proposition 2.1, Corollary 2.2]
- Named invariants: no program invariant is named. The mathematical state meaning is `n = |S|`, `mu` is the mean of `S`, and `M_{p,S}` is the centered sum of order `p`; `M_{1,S1}` and `M_{1,S2}` vanish by definition. [Pébay 2008, mandatory p. 10, proof of Proposition 2.1]
- Persistent and temporary state: cardinality, mean, and desired centered moment(s) per data set; `delta` is used in the formulas. A concrete storage layout is not stated. [Pébay 2008, mandatory pp. 7-10, eqs. (1.1)-(1.4), Proposition 2.1]
- Mutations, allocation, I/O and failure behavior: not stated on the mandatory pages.
- Time claims, including operation and case being bounded: the report calls the method one-pass and describes final distributed updates as negligible-cost aggregation, but the mandatory pages give no formal asymptotic bound for the order-two recurrence. [Pébay 2008, mandatory pp. 7-8]
- Space claims, including what is excluded: no formal bound is stated. The report motivates one-pass processing that does not access every point twice, but does not specify an implementation memory contract. [Pébay 2008, mandatory p. 7]
- Determinism, randomness or numerical assumptions: the identities are derived algebraically over finite sets. The report describes direct power-sum one-pass methods as numerically unstable, two-pass methods as generally more stable, and its target one-pass method as as numerically stable as possible; no universal IEEE-754 error or bitwise-order guarantee is stated. [Pébay 2008, mandatory pp. 7, 10, proof of Proposition 2.1]
- Variants explicitly distinguished by the source: two-pass versus one-pass; incremental singleton update versus pairwise update; second versus higher centered moments; centered moments versus covariance; corrected sum `M2` versus its unbiased variance finalization. [Pébay 2008, mandatory pp. 7-11]
- Ambiguities or internally inconsistent statements: the empty predecessor is allowed by the cardinality wording but its mean is undefined in the formulas (source ambiguity). “Robust” is motivational rather than a quantified floating-point guarantee (source ambiguity). No internal contradiction was found in the mandatory material.

### Chan, Golub and LeVeque 1979

- Problem stated by the source: compute the sum of squared deviations from the sample mean, denoted `S`, with attention to one-pass access, core-memory limits, operation counts, and numerical behavior; combine two samples and construct a pairwise algorithm. [Chan et al. 1979, mandatory PDF pp. 5-10, sections 1-3, definitions (1.1)]
- Inputs and their representation: a sample of `N` data points; updating uses a previously accumulated sample plus a new point; general combination uses two contiguous samples of sizes `m` and `n`, their sums `T`, and their corrected sums of squares `S`. [Chan et al. 1979, mandatory PDF pp. 5, 7-9, formulas (1.5)-(1.6), (2.1)]
- Preconditions and validity domain: the updating algorithm starts with a sample of size one and processes points 2 through `N`; general combination assumes two samples with positive sizes in the displayed denominators. Empty-input behavior is not stated. [Chan et al. 1979, mandatory PDF pp. 7-8, formulas (1.5)-(1.6), (2.1)]
- Output or observable interface: `S`, the corrected sum of squared deviations; intermediate sample sums `T` are also maintained. Although the title says sample variances, the mandatory algorithmic formulas compute `S`, not a universally defined variance value for every cardinality. [Chan et al. 1979, mandatory PDF pp. 5, 7-9, definitions (1.1), formulas (1.5), (2.1)]
- Postconditions and guarantees in exact arithmetic: definition (1.1) fixes `S = sum_i (x_i - mean)^2`; formula (2.1) combines the sums and corrected sums of two samples and reduces to the singleton updating formula. The correction added by the singleton updating formula is nonnegative in exact arithmetic. [Chan et al. 1979, mandatory PDF pp. 5, 7-9, formulas (1.1), (1.6), (2.1)]
- Strategy: avoid the cancellation-prone textbook rearrangement; update the running sum and corrected sum for each new datum, or recursively split the sample and combine partial states pairwise. [Chan et al. 1979, mandatory PDF pp. 5-10, formulas (1.2), (1.5)-(1.6), sections 2-3]
- Named invariants: no program invariant is named. The notation ties every `T_{i,j}` to the sum of its sample and every `S_{i,j}` to that sample's corrected sum of squares. [Chan et al. 1979, mandatory PDF pp. 7-9, formulas (1.6), (2.1)]
- Persistent and temporary state: the singleton updater retains running `T` and `S`; the pairwise tableau retains one intermediate location per column for both `T` and `S`, or uses a stack. [Chan et al. 1979, mandatory PDF pp. 7, 9-10, formula (1.5), section 3]
- Mutations, allocation, I/O and failure behavior: assignment-style updates to `T` and `S` are shown. The pairwise algorithm stores intermediate states. I/O and failure behavior are not stated. [Chan et al. 1979, mandatory PDF pp. 7, 9-10, formula (1.5), section 3]
- Time claims, including operation and case being bounded: the report gives explicit operation counts for two-pass variants and says grouping at `m = sqrt(N)` minimizes total arithmetic operations on the data; it does not state a formal big-O time bound for the singleton or pairwise algorithms on the mandatory pages. [Chan et al. 1979, mandatory PDF pp. 6, 9]
- Space claims, including what is excluded: the streaming tableau implementation of the pairwise algorithm uses one pass and `O(log_2 N)` storage locations for intermediates. A grouped update with group size `m` retains only `m` data values at a time, but is strictly no longer one-pass for `m > 1`. [Chan et al. 1979, mandatory PDF pp. 9-10, sections 2-3]
- Determinism, randomness or numerical assumptions: the algorithms are deterministic in their stated evaluation order. Floating-point discussion uses machine roundoff `u`, conditioning, analysis referenced to later sections, and experiments; the mandatory pages say singleton updating generally performs comparably to two-pass in practice and pairwise is often more stable, not that all orders are bitwise equal. [Chan et al. 1979, mandatory PDF pp. 5-10, sections 1-3]
- Variants explicitly distinguished by the source: direct two-pass; cancellation-prone textbook one-pass; corrected two-pass; singleton updating; grouped updating; recursive pairwise evaluation; streaming-tableau pairwise evaluation; cleanup alternatives for non-power-of-two sizes. [Chan et al. 1979, mandatory PDF pp. 5-10, sections 1-3]
- Ambiguities or internally inconsistent statements: empty-input and singleton variance finalization are not specified (source ambiguity). The report distinguishes two pairwise cleanup/evaluation arrangements and discusses their possible accuracy difference without a universal choice justified by the mandatory material (source ambiguity). The OCR is imperfect, but the required formulas and conceptual boundaries are readable; this is documentary source quality, not a normalized semantic difference.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason: `maintain-online-second-central-moment`. It is the problem of consuming a finite real-valued sequence once and producing its cardinality, mean when defined, and corrected sum of squared deviations. The identity names the mathematical quantity rather than Welford, because neither mandatory source supports using Welford as the normalized algorithm identity.
- Proposed `Algorithm` identity and reason: `incremental-centered-second-moment-update`. It denotes singleton initialization followed by the recurrence in Pébay (1.1)-(1.2), independently corroborated by the singleton updating account in Chan et al. (1.5)-(1.6). It is a strategy identity, not a variance-output alias.
- Proposed implementation identity, if executable source is in scope: none. No upstream implementation was inspected or imported.
- Existing Atlas entity that may be synonymous: not inspected under the isolation boundary; uncertain.

### Proposed problem contract

- `input`: a finite sequence `x = (x1, ..., xn)` of real values.
- `requires`: each input value participates in the source's real-arithmetic operations; for a total output contract including the empty sequence, the mean component must be optional. No IEEE-754 accuracy threshold is implied.
- `output`: `(n, mean, M2)`, where `n` is a nonnegative integer, `mean` is absent exactly when `n = 0`, and `M2` is a real corrected sum of squares. Population variance and unbiased sample variance are not this problem's primary output.
- `ensures`: `n = length(x)`; if `n = 0`, `mean = absent` and `M2 = 0` by the normalized wrapper convention, which is not an explicit source formula; if `n > 0`, `mean = (sum_i xi)/n` and `M2 = sum_i (xi - mean)^2`. The exact-arithmetic population variance is separately `M2/n` only for `n > 0`. The exact-arithmetic unbiased sample variance is separately `M2/(n - 1)` only for `n > 1`. For `n = 1`, `M2 = 0`, population variance is `0`, and unbiased sample variance is undefined. For `n = 0`, both variance finalizations and the mean are undefined under the source formulas.

The empty-state convention is an API normalization needed to make the stream contract total; it is not attributed to either source. A stricter alternative is to require `n > 0` and omit the optional mean. I recommend the optional-mean contract because it preserves the common request's finite-stream boundary without inventing a numeric empty mean.

### Proposed incremental algorithm contract

- `requires`: initialize a nonempty stream with `(n, mean, M2) = (1, x1, 0)`. For each later `y`, the prior state exactly denotes the processed prefix and `n >= 1`. Arithmetic must support the displayed real operations. The source does not provide a total transition whose prior mean is defined at `n = 0`.
- Transition: set `n' = n + 1`, `delta = y - mean`, `mean' = mean + delta/n'`, and `M2' = M2 + delta*(y - mean')`; return `(n', mean', M2')`. [Pébay 2008, mandatory p. 7, eqs. (1.1)-(1.2)]
- Determinism: deterministic for a fixed input order and fixed arithmetic semantics. No equality across different evaluation or partition orders is claimed.
- `time_worst`: `O(n)` for `n` values, inferred from one constant-size update per value; the sources declare one-pass access but do not state this asymptotic bound on the mandatory pages.
- `auxiliary_memory`: `O(1)` retained scalar state, inferred from the displayed `(n, mean, M2)` recurrence. This excludes the input stream and output representation. It is not the memory bound of the pairwise tableau.
- Stability/in-place: schema properties are not applicable to this numeric streaming family as currently defined; no claim proposed.
- Evidence: exact recurrence and state meaning are `declared` from Pébay (1.1)-(1.2) and independently supported by Chan et al. (1.5)-(1.6); the general pairwise identity has a source proof in Pébay Proposition 2.1 and may be recorded as `proven` in the worksheet, although external proof locators are not structural Atlas sources. One-pass status is `declared`. `O(n)` time and `O(1)` scalar memory are `inferred` from the recurrence. Floating-point comparative observations remain `declared` documentary claims, not correctness guarantees.

### Pairwise decision

Pairwise combination is a **related algorithm**, not the same algorithm, not merely a stage, and not merely documentary capability. Proposed identity: `combine-centered-second-moment-states`. It has a different input contract (two accumulated nonempty states rather than one state and one datum), enables partition aggregation/parallel composition, and can be arranged as Chan et al.'s recursive or streaming-tableau pairwise evaluation with `O(log N)` intermediate storage. Algebraically, the incremental update is a singleton specialization of this combination, as Pébay explicitly states. [Pébay 2008, mandatory pp. 8, 10-11, eqs. (1.3)-(1.4), Corollary 2.2, Remark 2.1; Chan et al. 1979, mandatory PDF pp. 8-10, formula (2.1), section 3]

For nonempty states `(n1, mu1, M2_1)` and `(n2, mu2, M2_2)`, its exact-arithmetic postcondition is:

```text
n = n1 + n2
delta = mu2 - mu1
mu = mu1 + n2 * delta / n
M2 = M2_1 + M2_2 + n1 * n2 * delta^2 / n
```

This identity does not imply associativity or order independence after IEEE-754 rounding.

- Proposed implementation effects and tests: none, because no executable implementation is in scope. A future correction oracle should compare exact-rational recurrence and pairwise identities on nonempty small sequences, check empty/singleton finalization domains, and separately test rather than assume floating-point order behavior.
- Candidate relationships to other imported subjects: `incremental-centered-second-moment-update` is a singleton specialization of `combine-centered-second-moment-states`; population and unbiased sample variance are domain-qualified finalizations of `M2`, not synonyms for `M2`.
- Information intentionally left documentary: bibliographic identity, source terminology (`S`, `M2`, updating, pairwise), derivation history, comparative numerical motivation, conditioning, operation-count details, and the uninspected VTK/Fortran implementation mentions.

### Alternative normalizations considered

1. Recommended: one corrected-sum problem, a singleton incremental algorithm, and a distinct related pairwise-combination algorithm; variance outputs are qualified finalizations.
2. One broad “online/parallel moments” algorithm with singleton and pairwise entry points. Rejected because it collapses distinct input, memory, and composition contracts.
3. Separate problems for corrected sum, population variance, and unbiased sample variance. This is defensible for strict output-based lookup, but I would not use it in this comparison because it multiplies entities around finalizations of the same maintained sufficient state and obscures their cardinality domains.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: authors, titles, report numbers, dates, exact URLs, required pages/items, and verified artifact hashes.
- Missing or unstable source identity: no documentation license was established; the Chan et al. locator is an archival OCR mirror rather than a publisher-hosted copy.
- Assessment: preserved.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: the normalized state retains count, mean, and corrected sum; the singleton recurrence and pairwise identity remain distinct; nonempty formula domains and variance finalization domains are explicit.
- Semantic details lost or altered: the normalized empty wrapper convention is added and declared; higher moments and covariance are outside the frozen subject; Chan et al.'s sum-based representation is translated to mean-based state.
- Assessment: preserved for the frozen subject, with an intentional total-API extension for empty input.

### Representational fidelity

- Source vocabulary and decomposition retained: `M2`/corrected sum of squared deviations, mean, count, singleton update, pairwise combination, population variance, and unbiased sample variance are separately named.
- Normalized or collapsed concepts: Chan et al.'s running sum `T` is adapted to a running mean; source-local `S` and Pébay's `M2` are normalized as the same mathematical corrected sum. Exact and machine arithmetic are not collapsed.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: mandatory pages mention VTK and a sample Fortran routine, but neither implementation is within the inspected mandatory material or imported.
- Correction oracle proposed: exact-rational reference checks for state and merge identities, cardinality-domain checks, plus non-normative IEEE-754 order probes.
- Behavior actually checked during this worksheet: none.
- Assessment: not assessed.

### Declared transformations

- translation: source symbols `S` and `M_{2,S}` -> normalized `M2`; reason: preserve a neutral mathematical identity while retaining both aliases documentarily.
- specialization or generalization: arbitrary-order pairwise Proposition 2.1 -> order-two combination; reason: frozen subject boundary. Pairwise combination -> singleton incremental transition; reason: source-declared specialization, represented as a relationship rather than identity collapse.
- type or representation adaptation: Chan et al.'s `(sample size, T, S)` -> `(count, mean, M2)`; reason: align independently supported mean recurrence and requested state while preserving exact conversion for nonempty samples.
- API decomposition or aggregation: corrected sum maintenance -> separate population/unbiased finalizers with `n > 0`/`n > 1`; reason: prevent undefined cases and distinct quantities from becoming one output guarantee.
- bug correction: none.
- pedagogical simplification: higher moments, covariance, and detailed operation-count variants omitted; reason: frozen order-two boundary, not a claim about the full reports.
- other: optional empty mean and `M2 = 0` wrapper state added; reason: total finite-stream API. This is explicitly normalized convention, not a source attribution.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Stateful stream transition over `(n, mean, M2)` | Problem/Algorithm free-text claims | lossy | selection, composition | worksheet |
| Real exact arithmetic versus IEEE-754 behavior and error | Algorithm requirements/free text | lossy | selection, substitution | worksheet |
| Singleton update is a specialization of pairwise combination | No variant/derivation relationship | absent | identity, substitution, composition | worksheet |
| Pairwise algorithm has a two-state input contract and partition semantics | One `Algorithm.solves` plus free text | lossy | selection, composition | worksheet |
| Corrected sum and two variance finalizers have different cardinality domains | Problem output/ensures free text | lossy | selection, substitution | worksheet |
| Empty state has absent mean, while singleton has `M2 = 0` but no unbiased variance | Human-readable requirements/output | ambiguous | selection, substitution | worksheet |
| Pairwise streaming tableau uses `O(log N)` intermediates while singleton update uses constant scalar state | Single algorithm complexity fields if identities are collapsed | ambiguous | identity, selection | worksheet |
| Bibliography, aliases `S`/`M2`, report pages and transformation provenance | Source strings/free text | lossy | documentary only | worksheet |
| Comparative/conditional numerical stability observations | No numerical-error/conditioning structure | absent | selection, substitution | worksheet |

For the lossy, absent, and ambiguous rows:

- Would two candidates become indistinguishable? Yes. Incremental and pairwise strategies, or corrected-sum and variance-returning APIs, can become indistinguishable if represented only by broad prose.
- Could an invalid candidate be selected? Yes. A request requiring unbiased variance for `n < 2`, constant retained state, or an IEEE-754 guarantee could match an underqualified claim.
- Could a valid substitution or composition be rejected? Yes. The exact singleton specialization and partition-combination relationship cannot be queried structurally.
- Is the issue specific to this family? Numeric error/conditioning and variance domains are family-specific in detail; state transitions, partial finalization domains, and related merge operations also arise outside this family.
- What second structural family would be needed before a schema proposal? A separate stateful aggregation family with a merge operation and partial/qualified finalizer, such as another mergeable streaming summary, would be needed before proposing public structure. No such family was read or inferred in this isolated import.

Information schema 0.1 cannot represent without lossy prose: the typed numeric domain; exact versus floating-point claim scope; optional/undefined outputs by cardinality; state transition signatures; maintained-state invariants; merge algebra and specialization relationships; per-operation complexity; conditioning/error bounds; bibliographic works and source-local aliases; or structured transformation provenance. The experimental AST likewise lacks real scalar arithmetic, aggregate state transitions, division, and a typed merge operation for this subject.

## 7. Operational probes

These are expected results, not executed tests.

- identity lookup: **unsupported** for the proposed new entities before a registry import; existing synonym lookup is uncertain because the registry is outside the isolation boundary.
- search by the proposed vocabulary: **unsupported** for `M2`, corrected sum, singleton update, and pairwise combination as structural vocabulary; free-text search after a hypothetical import would not establish semantic equivalence.
- qualification by current properties: **would require source-specific logic** for arithmetic domain, optional mean, finalizer cardinality, merge support, or numerical guarantee. Generic worst-time and memory text could be stored but not the operation-specific distinctions.
- substitution between source variants: **would require source-specific logic** to distinguish exact mathematical equivalence from floating-point evaluation behavior and to enforce `n > 0`/`n > 1` finalization domains.
- composition with a stated precondition/effect: **unsupported** generically; current composition cannot discover a new pairwise state combiner, and schema 0.1 cannot type the state effect.

## 8. Common selection requests

1. **Accept conditionally**: “Consume a finite real-valued stream once and return its count, mean and corrected sum of squared deviations using O(1) retained scalar state.” For a nonempty stream, Pébay (1.1)-(1.2) supplies the count/mean/`M2` recurrence, and the fixed displayed state supports an inferred `O(1)` retained-scalar bound; Chan et al. independently gives a one-pass updating method. For an empty stream, accept only if `mean` is optional/absent and `(0, absent, 0)` is declared as a normalized wrapper convention, because neither source defines an empty mean or empty transition. This accepts an exact-arithmetic identity, not a universal floating-point accuracy bound. [Pébay 2008, mandatory p. 7, eqs. (1.1)-(1.2); Chan et al. 1979, mandatory PDF p. 7, formulas (1.5)-(1.6)]
2. **Reject**: “Return an unbiased sample variance for every input length, including empty and singleton streams, without a partial or error result.” Pébay obtains the unbiased estimator as `M2/(n - 1)`, which is undefined at `n = 1`; the mean and displayed variance formulas also do not define a value for empty input. A total real-valued result for all lengths would invent unsupported semantics. [Pébay 2008, mandatory pp. 7-8, immediately after eq. (1.2); Chan et al. 1979, mandatory PDF pp. 5, 7, definition (1.1), initialization of (1.5)]
3. **Reject**: “Merge independently accumulated partitions and require bit-for-bit equality with every sequential evaluation order under IEEE-754 arithmetic.” Pébay (1.3)-(1.4) and Proposition 2.1 provide exact-algebraic pairwise identities, while Chan et al. motivates pairwise evaluation through numerical behavior and explicitly discusses evaluation arrangements. Neither source asserts IEEE-754 associativity or bitwise equality across every ordering; their floating-point observations cannot be elevated to that guarantee. [Pébay 2008, mandatory pp. 8-11, eqs. (1.3)-(1.4), Proposition 2.1; Chan et al. 1979, mandatory PDF pp. 8-10, formula (2.1), section 3]

## 9. Ambiguity classification

- Source ambiguity: empty initialization/mean; unquantified use of “robust”; no total unbiased-variance value for `n < 2`; no universal floating-point order guarantee; choice among pairwise evaluation/cleanup arrangements.
- Protocol ambiguity: whether a problem identity should require nonempty input or use an optional mean to totalize the finite-stream request; whether output-only variance finalizations merit separate Problems. The recommended choice is explicit above.
- Model ambiguity/insufficiency: schema 0.1 cannot structurally distinguish arithmetic semantics, state transitions, merge relations, partial finalizers, or operation-specific memory. This can change selection and composition outcomes.
- Importer error: none known. The archival OCR has defects, so no claim depends on copying its prose or Fortran text.

## 10. Importer conclusion

- Recommended normalization: normalize the maintained mathematical object as `(count, optional mean, M2)`; use `incremental-centered-second-moment-update` for singleton updates and a distinct related `combine-centered-second-moment-states` algorithm for partition merging; model population and unbiased sample variance only as separately domain-qualified finalizations.
- Unresolved questions: whether Atlas ultimately prefers optional output or a nonempty problem precondition; how external proof citations should attach to evidence; whether a second independent mergeable-summary family exhibits the same structural gaps.
- Decision-relevant loss: arithmetic-domain scope, undefined-output domains, state transitions, and the incremental/pairwise relationship are not queryable in schema 0.1. These gaps directly affect all three common requests.
- Documentary divergence that should remain acceptable: `S` versus `M2`, sum-based versus mean-based state, locator granularity, and source-specific descriptions of robustness, provided the exact formulas and claim scopes remain auditable.
- Proposed next minimal experiment: after both independent worksheets are frozen, compare the three request decisions and run a manual contract discriminant on empty, singleton, and two-part nonempty inputs, keeping exact-rational results separate from IEEE-754 observations.
- Schema or AST change requested now: none. No public schema change is requested.
