# K-M4 importer B: Bloom bit-field method 2

Protocol: `k-m0.2`

## 1. Work and source record

- Importer: `k-m4-importer-b`.
- Importer-observed batch start: `2026-07-15T01:12:49+02:00`.
- Externally observed batch start/end and elapsed minutes: unavailable to the
  importer; orchestrator-owned.
- Importer-observed end: `2026-07-15T01:24:31+02:00`.
- Activity subdivisions: unavailable; no retrospective estimate.
- Human interventions: none.
- Tools: browser/web fetch, `curl`, `pdftotext`, editor.
- Subject: Bloom hash-coding method 2, insertion and membership test.
- Bibliography: Burton H. Bloom, "Space/Time Trade-offs in Hash Coding with
  Allowable Errors," *Communications of the ACM* 13(7), July 1970, pp. 422-426.
- Mandatory locators actually read:
  - <https://dl.acm.org/doi/10.1145/362686.362692>
  - <https://www.cs.princeton.edu/courses/archive/spr05/cos598E/bib/p422-bloom.pdf>
- Supplemental pages: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper, abstract method rather than reusable code.
- Code license: not applicable/not stated. Article copyright is ACM; no software
  license is asserted.

## 2. Source-faithful account

- Problem: represent a given set of messages compactly for repeated membership
  tests while allowing some nonmembers to be falsely accepted; computational
  factors are bit-area size, average rejection time and allowable error fraction
  (abstract/introduction, p. 422, mandatory PDF).
- Inputs: set of n messages, bit-field of N addressable bits initially zero,
  number d of distinct bit addresses per message, and a deterministic-by-message
  pseudorandom address-generation procedure (Method 2, pp. 423-425).
- Requires: insertion and test generate the same d distinct addresses for the
  same message; N/d parameters and address process satisfy the analysis
  assumptions. The paper assumes constant time to calculate an address, access
  an addressed bit and test it (Computational Factors/analysis, pp. 424-425).
- Output/interface: insertion sets all d addressed bits to one. Membership test
  accepts if all d addressed bits are one and rejects if any is zero (Method 2,
  p. 423).
- Guarantees: inserted messages are accepted under consistent addressing;
  nonmembers may be falsely accepted (errors of commission). A zero bit provides
  definitive rejection. The paper does not support deletion.
- Strategy: no cells/messages are stored; N individually addressable bits encode
  the set via multiple hash addresses (Method 2, p. 423).
- Named invariant: not named. Monotonic bit state follows the procedure: bits
  begin zero and insertion only sets bits to one.
- State/effects: persistent N-bit field; insertion mutates d bits; membership
  reads up to d bits and can reject early. Hash/address computation state is not
  specified. No core I/O is defined.
- Time: the paper's primary time factor is expected reject time, normalized to
  address calculation plus bit access/test. It is not a general Big-O method
  bound. A test examines at most d generated addresses under the described
  procedure; early rejection changes average time. The superiority analysis
  depends on constant-time bit addressing and may fail for multibit chunks or
  additional hash-address costs (pp. 424-425).
- Space: N bits; the analysis relates N, n, d, fraction of zero bits and expected
  error. It explicitly studies space/time/error trade-offs rather than one
  universal bound (pp. 424-425).
- Randomness/numerics: address generation is described as pseudorandom and
  message-dependent. The expected false-accept fraction under the paper's model
  is `P'' = (1 - phi'')^d`, where `phi''` is expected zero-bit proportion;
  equation (16) models `phi'' = (1 - d/N)^n` (Analysis of Method 2, p. 425).
- Variants: method 2 is distinguished from conventional error-free hashing and
  method 1. Applications requiring error-free membership are explicitly outside
  the intended domain (pp. 422-423).
- Source ambiguity: independence/uniformity details of generated addresses are
  described through analysis assumptions rather than a reusable hash API;
  modern false-positive formulas should not replace the paper's notation here.

## 3. Proposed Atlas normalization

- `Problem` identity: `set.approximate_membership_insert_query`.
- Exact input: `fixed N-bit field initially zero, insertion/query messages,
  parameter d, and a repeatable procedure yielding d distinct addresses in
  [0,N) per message`.
- Exact requires: `same message yields same addresses for insert and query;
  parameters/error model are acceptable; no deletion is required; address/bit
  operations satisfy the chosen cost model`.
- Exact output: insertion returns updated bit field; query returns
  `possibly-member` when all addressed bits are one, otherwise `definitely-not`.
- Exact ensures: `every inserted message tests possibly-member; a definitely-not
  result is sound; nonmembers may return possibly-member according to the
  source's expected error model`.
- `Algorithm` identity: `set.bloom_1970.bit_field_method_2`.
- Requirements: exact problem requirements and monotone insertion-only use.
- Determinism: operations are deterministic for fixed address function and bit
  state; accuracy is probabilistically analyzed over the address/message model.
- Time: insertion at most d address/bit-set units; rejection query between 1 and
  d address/test units, acceptance d. Source emphasizes expected reject time;
  no single source Big-O in n is stated.
- Auxiliary memory: N persistent bits; O(1) scalar temporary state is a plausible
  implementation reading, but not explicitly bounded, so record `not stated`
  for auxiliary memory separate from the N-bit output/state.
- Implementation identity: none; paper method/pseudocode is not an imported
  executable implementation.
- Boundary/effects: initialization clears N bits; insertion mutates bit field;
  query reads it; address generation consumes computation but no specified I/O,
  allocation or failure behavior.
- Tests: no false negatives for inserted identifiers, deterministic repeat of
  addresses, sound zero-bit rejection, collision saturation, d=1, d near N,
  measured false-positive observations clearly separated from source equations.
- Evidence: method and equations `declared`; no Atlas `tested` result.
- Existing Atlas synonym: not assessed under experiment boundary.
- Documentary only: application examples, conventional/method-1 comparison,
  machine bit-access assumptions, and parameter optimization curves.

## 4. Fidelity and transformations

### Bibliographic fidelity

Author, title, venue, date, pages, DOI and readable copy are preserved.
Assessment: `preserved`.

### Algorithmic fidelity

Insertion/query, monotone bits, one-sided errors and source cost/error model are
preserved. Assessment: `preserved` under stated address assumptions.

### Representational fidelity

The source's accept/reject is renamed possibly-member/definitely-not to prevent
an allowable false acceptance being treated as exact membership. Assessment:
`intentionally transformed` and semantically clarifying.

### Executable fidelity

No upstream executable artifact is defined. Oracle checks one-sided behavior
and source equations separately; no behavior was run. Assessment: `not assessed`.

### Declared transformations

- Translation: messages/hash area to identifiers/N-bit state.
- Specialization/generalization: method 2 only; no deletion/counting extension.
- Type adaptation: accept/reject becomes one-sided membership-result vocabulary.
- API aggregation/decomposition: insertion and query remain two operations of
  one persistent stateful method.
- Bug correction: none.
- Pedagogical simplification: application-specific secondary test excluded.
- Other: equations retain the paper's assumptions and evidence scope.

## 5. Schema-loss record

Schema 0.1 cannot structurally express one-sided approximation, false-positive
model/rate, parameterized accuracy, persistent bit state, operation families,
early-rejection expected cost, hashing assumptions, or distinction between
state memory and auxiliary scratch. Prose-only `ensures` could let an approximate
candidate qualify for exact membership.

## 6. Selection requests

1. **Request:** memory-bounded prefilter where false positives are allowed but
   false negatives are not. **Accept** under consistent addressing and no deletion.
2. **Request:** exact set membership for authorization decisions. **Reject**:
   method 2 explicitly permits errors of commission.
3. **Request:** remove elements while preserving guarantees for other members.
   **Reject**: deletion is not defined and clearing shared bits can introduce
   false negatives.

## 7. Ambiguities and conclusion

- Source ambiguity: exact address independence, finite pseudorandom construction,
  edge parameters and failure behavior are not fully operationalized.
- Protocol ambiguity: whether to modernize result vocabulary; transformation is
  declared and source accept/reject terms remain cited.
- Model ambiguity: approximation/error constraints have no selectable fields.
- Decision-relevant divergence: calling acceptance exact or omitting monotone
  no-deletion usage would accept invalid authorization/deletion requests.
- Public schema change requested: **none**; approximation loss is recorded for
  cross-family adjudication rather than fixed during K-M4.
