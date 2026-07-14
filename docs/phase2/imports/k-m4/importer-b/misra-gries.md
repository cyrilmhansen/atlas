# K-M4 importer B: Misra-Gries repeated elements

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
- Subject: bounded-counter repeated-elements algorithm for frequency > n/k,
  corresponding to the paper's second algorithm (algorithm (3)).
- Bibliography: J. Misra and David Gries, "Finding Repeated Elements," *Science
  of Computer Programming* 2(2), 1982, pp. 143-152.
- Mandatory locators actually read:
  - <https://doi.org/10.1016/0167-6423(82)90012-0>
  - <https://khoury.northeastern.edu/home/pandey/courses/cs7800/spring26/papers/mg.pdf>
- Supplemental pages: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper with guarded-command algorithms/proofs.
- Code license: not applicable/not stated. Article copyright: North-Holland 1982;
  no software license is asserted.

## 2. Source-faithful account

- Problem: find values occurring more than `n/k` times in array `b[0..n-1]`, for
  `2 <= k <= n`. Both presented methods use two passes: first produce possible
  values, then count candidates to identify actual frequent values (Introduction,
  pp. 143-144, mandatory PDF; slash rendering verified against packet subject).
- Inputs: finite array of n comparable values and integer k in [2,n]. Algorithm
  (3) treats the prefix as a bag (multiset) (Sections 1 and 3).
- Requires: values support equality/comparison used by the chosen bag
  implementation; full array is available for the verifying second pass.
- Output: exact final answer after verification is every value whose count > n/k.
  First pass alone outputs a candidate bag/set containing all possible answers,
  not an exact heavy-hitter set (Sections 1 and 3).
- Guarantees: deleting k distinct elements repeatedly yields a k-reduced bag;
  only values in any k-reduced bag can occur more than N/k in original bag
  (Theorem 1, p. 147). Algorithm (3) maintains `t` as a k-reduced bag for the
  processed prefix (p. 147).
- Strategy: maintain bag t and number d of distinct values. Insert the next value;
  when d reaches k, delete one occurrence of each of k distinct values. Before
  and after each iteration t has at most k-1 distinct values (Section 3, p. 147).
- Named invariant: `0 <= i <= n`, t is a k-reduced bag of prefix `b[0..i-1]`, and
  d is the number of distinct elements of t (algorithm (3), p. 147).
- State/effects: persistent candidate counts/bag and index; first pass reads each
  array item. Second pass rereads array to verify counts. Paper AVL realization
  uses at most k nodes `(value,count)` and mutates/searches/deletes them
  (Section 4, p. 148).
- Time: paper's AVL implementation gives worst O(n log k) for operations on bag
  t and O(k) extra space (abstract and Section 4). The second verification pass
  can be O(n log |t|) (Introduction). The paper's comparison-model result suggests
  optimality but should not be generalized beyond that model.
- Space: O(k) for AVL representation (Section 4).
- Determinism/randomness: deterministic; k-reduced bag is not unique in general,
  but the specified iteration/deletion implementation can choose a deterministic
  representative. Exact verified output is order-independent.
- Variants: majority k=2, first algorithm (2) with no established useful bound on
  candidate-set size, second bounded algorithm (3), and exact two-pass result.
- Source ambiguity: modern "Misra-Gries counters" often names a one-pass candidate
  summary; this paper's stated finding problem includes verification. Candidate
  counts are not asserted to equal true frequencies.

## 3. Proposed Atlas normalization

- `Problem` identity: `frequency.exact_values_above_n_over_k` for the two-pass
  paper problem. A related but distinct problem
  `frequency.candidates_above_n_over_k` represents first-pass summary output.
- Exact problem input: `finite rereadable sequence of n comparable values and
  integer k with 2 <= k <= n`.
- Exact problem requires: `equality/comparison is consistent; a second pass is
  available for exact output`.
- Exact problem output: `set of distinct values whose true occurrence count is
  strictly greater than n/k`.
- Exact problem ensures: `no qualifying value omitted and no nonqualifying value
  included after verification`.
- `Algorithm` identity: `frequency.misra_gries_1982.algorithm_3_verified`.
  First pass should be related as a stage, not substituted for the exact algorithm.
- Requirements: exact problem requirements; bounded candidate structure.
- Determinism: `true` for exact set, given deterministic comparison/iteration.
- Time: worst O(n log k) under paper's AVL comparison implementation, including
  same-order verification; exact accounting of both passes remains documentary.
- Auxiliary memory: O(k) candidate-tree nodes/counters.
- Implementation identity: none executable; the paper specifies guarded-command
  pseudocode and an AVL representation boundary.
- Boundary/effects: reads sequence twice for exact result; mutates O(k) candidate
  state; no randomness; allocation and I/O/failure behavior unstated.
- Tests: threshold strictness at exactly floor/real n/k, k=2, k=n, all distinct,
  all equal, adversarial interleavings, compare first-pass candidate superset to
  verified exact set, and use object identity only through comparison semantics.
- Evidence: problem, theorem, invariant, bounds `declared` by mandatory paper;
  no Atlas execution was tested.
- Existing Atlas synonym: not assessed under experiment boundary.
- Documentary only: guarded-command syntax, AVL node layout, proof development,
  and comparison-tree lower-bound discussion.

## 4. Fidelity and transformations

### Bibliographic fidelity

Authors, title, venue, year, pages, DOI and readable copy are preserved.
Assessment: `preserved`.

### Algorithmic fidelity

k-reduction, bounded candidates, strict threshold and mandatory verification for
exact results are retained. Assessment: `preserved`.

### Representational fidelity

Bag multiplicities become value/counter state; guarded commands become a staged
summary-plus-verification contract. Assessment: `intentionally transformed`.

### Executable fidelity

No upstream executable artifact is present. Oracle computes exact frequencies
and separately checks first-pass candidate inclusion. No behavior was run.
Assessment: `not assessed`.

### Declared transformations

- Translation: mathematical bag/guarded commands to bounded counters/stages.
- Specialization/generalization: algorithm (3) only; algorithm (2) excluded.
- Type adaptation: comparable array values become equality-consistent sequence
  elements; AVL remains the source cost implementation.
- API aggregation/decomposition: candidate generation and verification are two
  stages of exact output, with first stage also named as a distinct capability.
- Bug correction: none; PDF slash extraction follows packet's explicit n/k.
- Pedagogical simplification: proof derivation omitted, invariant preserved.
- Other: candidate counts are not relabeled exact frequencies.

## 5. Schema-loss record

Schema 0.1 cannot structurally express a staged/two-pass algorithm, rereadability,
candidate-superset versus exact-set guarantees, strict parameterized frequency
threshold, memory/time parameterized by k, or implementation-dependent comparison
cost. One prose output can collapse first-pass candidates with verified answers
and permit a false-positive result.

## 6. Selection requests

1. **Request:** exact values occurring strictly more than n/k in a rereadable
   sequence, O(k) state acceptable. **Accept** the verified two-pass algorithm.
2. **Request:** exact heavy hitters from a nonreplayable one-pass stream.
   **Reject**: first pass only guarantees a candidate superset; exact verification
   needs a second scan.
3. **Request:** bounded candidate set containing every value above n/k, false
   candidates acceptable. **Accept** first-pass capability; do not represent its
   counters as exact frequencies.

## 7. Ambiguities and conclusion

- Source ambiguity: k-reduced bag nonuniqueness and exact implementation choice;
  threshold notation in extracted PDF is degraded, resolved by packet subject.
- Protocol ambiguity: whether one subject should yield two `Problem` identities.
  It is necessary here because candidate and exact outputs change acceptance.
- Model ambiguity: schema has no stage/capability relation or reread requirement.
- Decision-relevant divergence: collapsing candidate production with exact
  repeated-element finding wrongly accepts one-pass exact requests.
- Public schema change requested: **none**; keep the two-stage relation
  experimental pending cross-family evidence.
