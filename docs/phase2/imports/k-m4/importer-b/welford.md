# K-M4 importer B: Welford online corrected moments

Protocol: `k-m0.2`

## 1. Work and source record

- Importer: `k-m4-importer-b`.
- Importer-observed batch start: `2026-07-15T01:12:49+02:00`.
- Externally observed batch start/end and elapsed minutes: unavailable to the
  importer; orchestrator-owned.
- Importer-observed end: `2026-07-15T01:24:31+02:00`.
- Activity subdivisions: unavailable; no retrospective estimate.
- Human interventions: none.
- Tools: browser/web fetch, DOI content negotiation with `curl`, editor.
- Subject: Welford online corrected sums of squares and products.
- Bibliography: B. P. Welford, "Note on a Method for Calculating Corrected Sums
  of Squares and Products," *Technometrics* 4(3), August 1962, pp. 419-420.
- Mandatory locator actually read:
  <https://doi.org/10.1080/00401706.1962.10490022>.
- Directly reachable locator attempted: publisher abstract/PDF destination from
  DOI metadata; it returned a Cloudflare challenge rather than article text.
- Supplemental pages: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper.
- Code license: not applicable/not stated. Documentation/article copyright and
  reuse license: not stated in accessible DOI metadata.
- Coverage warning: only DOI bibliographic metadata was accessible. No abstract,
  formulas, pseudocode, proof, preconditions or complexity text was readable.

## 2. Source-faithful account

The packet names the subject "Welford online corrected sums of squares" and the
accessible DOI metadata confirms only title, author, journal, volume, issue,
date and pages. Consequently:

- Problem: calculating corrected sums of squares and products; "online" comes
  from the frozen subject label, not accessible article text.
- Inputs and representation: `not stated in accessible source`.
- Preconditions/validity domain: `not stated`.
- Output/interface and postconditions: `not stated`; the title names corrected
  sums, but does not define their exact convention.
- Strategy, recurrence and named invariant: `not stated`.
- Persistent/temporary state and mutations: `not stated`.
- Allocation, I/O and failure behavior: `not stated`.
- Time and space claims: `not stated`.
- Determinism, randomness and numerical assumptions: `not stated`.
- Variants: corrected sums of squares and products are both named in the title;
  their relationship is `unresolved`.
- Source ambiguity: the source content is unavailable, so even standard-looking
  recurrence, sample/population normalization, initialization, and finite-
  precision properties cannot be attributed to Welford from this packet.

No gap above is filled from general knowledge.

## 3. Proposed Atlas normalization

- `Problem` identity: provisional `statistics.online_corrected_sums`. This is
  deliberately broader than variance because the accessible title names sums of
  squares and products, not a variance normalization.
- Exact problem input: `not established`; temporary documentary placeholder:
  `a sequentially supplied observation stream, with value arity unresolved`.
- Exact problem requires: `not established`.
- Exact problem output: `corrected sums of squares and/or products, convention
  unresolved`.
- Exact problem ensures: `not established`; do not claim equality to a batch
  formula or numerical stability without article text.
- `Algorithm` identity: provisional `statistics.welford_1962.corrected_sums`.
- Algorithm requirements: `not established`.
- Determinism: `uncertain`; no random mechanism is suggested by metadata, but a
  required schema value must not be inferred from absence.
- Time and auxiliary memory: `not stated`. Do not force conventional O(n)/O(1)
  claims into required schema fields.
- Implementation identity/boundary/effects: none; accessible material defines no
  executable artifact or inspectable pseudocode.
- Tests proposed after source access: compare every recurrence and initialization
  against the paper; distinguish sums of squares from products, empty/singleton
  streams, shifted large-magnitude values, order effects, and batch identities.
- Evidence: bibliography `declared` by DOI metadata; every algorithmic claim
  remains `unresolved`, not `declared`.
- Existing Atlas synonym: not assessed under experiment boundary.
- Documentary only: all provisional vocabulary until the paper is readable.

## 4. Fidelity and transformations

### Bibliographic fidelity

DOI, title, author, journal, volume/issue, date and pages are preserved from DOI
metadata. Assessment: `preserved` bibliographically.

### Algorithmic fidelity

No algorithmic content was accessible, so none can be assessed. Assessment:
`unresolved`.

### Representational fidelity

Only source title vocabulary is retained; input/output/state decomposition is
not available. Assessment: `unresolved`.

### Executable fidelity

No executable artifact was identified from accessible material. No oracle or
behavioral check is defensible yet. Assessment: `not assessed`.

### Declared transformations

- Translation: title vocabulary only.
- Specialization/generalization: provisional umbrella over squares/products;
  explicitly not stabilized.
- Type adaptation: none.
- API aggregation/decomposition: none.
- Bug correction: none.
- Pedagogical simplification: none.
- Other: inaccessible source content is represented as missing evidence rather
  than reconstructed knowledge.

## 5. Schema-loss record

There is first a source-coverage failure, not evidence of a schema defect. If the
expected online recurrence were later confirmed, schema 0.1 still lacks typed
numeric domains, rounding/error models, streaming state transitions, and
multiple moment/product outputs. Those are baseline predictions only; this
worksheet cannot establish their decision impact from the inaccessible paper.

## 6. Selection requests

1. **Request:** exact online corrected sum of squares for finite real inputs.
   **Reject pending source access**: exact convention and arithmetic domain are
   not established.
2. **Request:** one-pass covariance/product accumulator with bounded auxiliary
   memory. **Reject pending source access**: products are title-level only; pass
   count and memory are unreadable.
3. **Request:** numerically stable population variance under IEEE-754 arithmetic.
   **Reject**: neither variance normalization nor a floating-point error guarantee
   is available from the accessible source.

## 7. Ambiguities and conclusion

- Source ambiguity: all algorithmic substance is unavailable behind publisher
  access/challenge; formulas and guarantees cannot be adjudicated.
- Protocol ambiguity: the packet supplies no readable alternate copy for this
  subject, unlike three other papers.
- Model ambiguity: provisional only; numeric/streaming gaps cannot be promoted
  from inaccessible evidence.
- Decision-relevant divergence: any importer supplying recurrence, O(1) state or
  stability from memory would accept requests this evidence-bounded import must
  reject.
- Minimal follow-up: provide a frozen readable copy or licensed transcription of
  pages 419-420, then redo this worksheet independently.
- Public schema change requested: **none**.
