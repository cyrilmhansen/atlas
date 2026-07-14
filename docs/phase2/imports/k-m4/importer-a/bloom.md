# K-M4 import A: Bloom bit-field method 2

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` and `pdftotext`.
- Subject: Bloom's hash-coding method 2, insertion and membership test.
- Author/work: Burton H. Bloom, “Space/Time Trade-offs in Hash Coding with
  Allowable Errors,” *Communications of the ACM* 13(7), July 1970,
  pages 422-426.
- DOI locator attempted but not readable (HTTP 403):
  <https://dl.acm.org/doi/10.1145/362686.362692>.
- Required readable locator actually read:
  <https://www.cs.princeton.edu/courses/archive/spr05/cos598E/bib/p422-bloom.pdf>.
- Supplemental pages consulted: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper.
- Code license: not applicable; no implementation code is imported.
- Article copyright: not stated in the readable scan beyond its 1970 CACM
  publication marks.

## Source-faithful account

- Problem: test messages one by one for membership in a fixed set while trading
  hash-area space and average rejection time against an allowable fraction of
  false identifications as members (mandatory abstract/introduction).
- Inputs: set of `n` messages, a zero-initialized hash area of `N` individually
  addressable bits, `d` distinct pseudorandom bit addresses derived from each
  message, and later test messages (mandatory “Two Hash-Coding Methods,” Method
  2).
- Preconditions: insertion and testing generate addresses in the same manner;
  the `d` addresses for one message are distinct; bit-address calculation/access
  follows the paper's cost assumptions. Method 2 is unsuitable where error-free
  membership is required (mandatory introduction and Method 2).
- Output: accept if all `d` addressed bits are one, reject when any addressed bit
  is zero (mandatory Method 2).
- Guarantees: inserting sets all addressed bits. Therefore an inserted message is
  accepted while bits are never cleared; a nonmember may be accepted, an error
  of commission. The paper analyzes expected false acceptance, not a per-query
  worst-case probability guarantee (mandatory introduction and Method 2
  analysis).
- Strategy: initialize bits to zero; for every stored message generate `d`
  distinct addresses and set them to one. A query regenerates its `d` addresses
  and stops at the first zero, or accepts after all are one (mandatory Method 2).
- Named state/invariant: no formal invariant is named. The expected zero-bit
  fraction after storing `n` messages is denoted phi and approximated from
  `(1-d/N)^n`; false acceptance occurs when all queried bits are one (mandatory
  equations 16-17).
- State/effects: persistent `N`-bit field; insertion monotonically writes zero to
  one, membership reads and may stop early. Hash-address generation has
  computational cost. No deletion operation is defined.
- Time: the paper measures reject time as expected bit-address calculations,
  accesses and tests. Under its assumptions and approximations, expected bits
  tested for a rejected nonmember are `T = 1/phi`; insertion and an accepted test
  touch `d` bits. It explicitly warns that constant bit-access/address time and
  single-bit units are strong machine assumptions (mandatory “Computational
  Factors,” equation 20 and comparison note).
- Space: exactly `N` hash-area bits, excluding hash-address computation and any
  representation of functions. The paper derives trade-offs among `N`, expected
  error fraction, and reject time (mandatory equations 16-24).
- Determinism/randomness: addresses are pseudorandom computations dependent on
  the message. Operations need not draw runtime randomness, but expected
  occupancy/error analysis assumes hash-like distribution behavior.
- Variants: Method 1 stores compact codes in cells; Method 2 uses bits and cannot
  reach error-free performance by parameter growth in the same way. The paper's
  optimum-space discussion places half the bits at one under its model.
- Ambiguity: independence/uniformity assumptions for distinct addresses are not
  packaged as a formal hash-family contract. “Allowable fraction” is an expected
  fraction over the message space, not necessarily a bound for every workload.

## Proposed Atlas normalization

- `Problem` identity: `approximate_membership.insert_and_query.false_positive`.
- Exact `input`: an initially empty approximate-membership state parameterized by
  `N` bits and `d` deterministic address functions; a sequence of insert(message)
  and contains(message) operations with no deletion.
- Exact `requires`: each message maps reproducibly to `d` distinct addresses in
  `0..N-1`; bit field starts zero and is never cleared; workload/hash assumptions
  required for any expected-error claim are declared separately.
- Exact `output`: insertion has no return; membership returns `false` upon the
  first zero addressed bit, otherwise `true`.
- Exact `ensures`: `false` proves the queried message was not inserted; every
  inserted message returns true; true may be a false positive. Expected false
  positive rate follows the paper's equations only under its model.
- `Algorithm` identity: `approximate_membership.bloom.bit_field_method_2`.
- Requirements: fixed no-delete bit field, matching address generation, and
  allowable false positives.
- Determinism: deterministic state transitions for fixed address functions;
  probabilistic/expected quality is about address/workload distribution.
- Time: insertion and positive membership O(d) address/bit operations; negative
  query at most O(d), with the paper's expected reject-time formula under stated
  approximations.
- Memory: N persistent bits, or O(N); hash function state is excluded/not stated.
- Evidence: method and equations are `declared`; the paper's derivation remains
  documentary rather than Atlas `proven`; no implementation is tested.
- Implementation boundary: algorithmic method and cost model, not executable
  code or ABI. Effects are persistent bit mutation during insertion, read-only
  early-exit query, hash computation, no I/O/blocking stated.
- Candidate tests: inserted-item no-false-negative property, all-zero field,
  saturation, address distinctness, controlled exhaustive hash mappings and
  observed false-positive estimates labeled `observed`, never promoted to a
  theorem.
- Information left documentary: exact hash construction, independence model,
  machine word/cache costs, optimal-parameter derivation and application example.

## Fidelity

### Bibliographic fidelity

Author, title, venue, issue, date, pages, DOI and readable scan are preserved;
DOI landing content was inaccessible. Assessment: **preserved**.

### Algorithmic fidelity

Zero initialization, distinct addresses, monotone insertion, early rejection,
false-positive-only semantics and paper cost/error model are retained.
Assessment: **preserved**.

### Representational fidelity

Paper messages/hash area become a stateful operation sequence, while `N`, `d`
and bit-address semantics remain explicit. Assessment: **intentionally
transformed**.

### Executable fidelity

No source implementation is defined. A controlled address-function oracle can
check exact state semantics but not establish the paper's distribution model.
Nothing was executed. Assessment: **not assessed**.

### Declared transformations

- Translation: paper store/test procedure to insert/contains operations.
- Generalization: message representation abstracted, bit field retained.
- Type adaptation: pseudorandom address process exposed as parameterized
  deterministic functions plus distribution assumptions.
- API aggregation: insertion and membership share one persistent state contract.
- Bug correction: none.
- Pedagogical simplification: Method 1 and application sizing excluded.
- Other: expected false-positive analysis is separated from deterministic
  no-false-negative semantics.

## Model friction

| Source fact | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| One-sided error | ensures prose | lossy | selection, substitution |
| Parameterized expected error fraction | no probability/error field | absent | selection |
| Hash/workload distribution premise | requirements prose | lossy | selection |
| Persistent insert/query state | problem prose | lossy | composition |
| Different insertion/negative/positive costs | one time claim | absent | selection |
| N bits versus function state/cache behavior | memory string | ambiguous | selection |
| No deletion | absent operation relation | lossy | composition |

A false-negative-capable filter could appear equivalent, or Method 2 could be
selected for an error-free/deletion request. A valid implementation could be
rejected if deterministic execution is confused with probabilistic quality.
One-sided probabilistic guarantees also arise in sampling and frequent-item
candidates, while stateful operation costs occur in hash maps.

## Selection requests

1. “Rejects must be definitive; false positives are allowed up to a calibrated
   expected rate under a declared hash/workload model.” **Accept conditionally**:
   this is Method 2's intended contract, subject to choosing `N` and `d`.
2. “Membership answers must be error-free for all messages.” **Reject**: Method 2
   explicitly allows false acceptance and precludes error-free performance.
3. “Support deletion of arbitrary inserted messages without false negatives.”
   **Reject**: the source defines a monotone bit field and no deletion; clearing a
   shared bit would invalidate the stated guarantee.

## Ambiguities and conclusion

- Source ambiguity: exact hash independence/distribution and whether expected
  error is evaluated uniformly over the full message space or a workload.
- Protocol ambiguity: modern term “Bloom filter” is not used as a reason to add
  later variants; only method 2 is normalized.
- Model ambiguity: deterministic behavior and probabilistic quality cannot be
  independently qualified, and per-operation costs collapse.
- Recommended normalization: one stateful approximate-membership problem and
  source-named method 2 algorithm, parameterized by `N`, `d`, and a declared
  address model.
- Minimal next probe: exhaust a tiny finite message/address space to distinguish
  exact no-false-negative semantics from model-dependent false-positive rates.
- Public schema change requested: **none**.
