# K-M4 import A: Misra-Gries repeated elements

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` and `pdftotext`.
- Subject: bounded-counter repeated-elements algorithm for values occurring
  strictly more than `n / k` times.
- Authors/work: J. Misra and David Gries, “Finding Repeated Elements,” *Science
  of Computer Programming* 2(2), 1982, pages 143-152.
- DOI locator attempted but not readable through its publisher:
  <https://doi.org/10.1016/0167-6423(82)90012-0>.
- Required readable locator actually read:
  <https://khoury.northeastern.edu/home/pandey/courses/cs7800/spring26/papers/mg.pdf>,
  especially Sections 1, 3, 4 and 6.
- Supplemental pages consulted: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper.
- Code license: not applicable; guarded-command pseudocode only.
- Article copyright: 1982 North-Holland, as printed in the scan.

## Source-faithful account

- Problem: for array `b[0..n-1]` and `2 <= k <= n`, find the values occurring
  strictly more than `n / k` times (mandatory abstract/introduction; the scan's
  extracted division glyph is unreliable, so the packet's explicit `n / k`
  transcription fixes the symbol).
- Inputs: array of `n` comparable values and integer `k`. Section 6 notes that if
  `n` is unknown, a preliminary linear length pass preserves the asymptotic
  bound (mandatory Sections 1 and 6).
- Preconditions: `2 <= k <= n`; values support comparison for the AVL
  implementation; the array can be scanned again for exact verification
  (mandatory Sections 1 and 4).
- Output: all and only values whose actual occurrence count exceeds `n/k` after a
  candidate pass and a verifying count pass (mandatory introduction). The first
  pass alone returns possible values, not exact answers.
- Guarantee: any value over the threshold remains in every relevant
  `k`-reduced candidate bag. Theorem 1 states that only values in a `k`-reduced
  bag can exceed the threshold (mandatory Section 3).
- Strategy: maintain bag `t` for the processed prefix. If the next value is
  already present, add one occurrence. Otherwise add it as a new distinct value;
  when there are `k` distinct values, delete one occurrence of each and remove
  zero counters. Then rescan the input and count only retained distinct
  candidates (mandatory Algorithms 3 and Section 1 verification description).
- Named invariant: `t` is a `k`-reduced bag of the processed prefix and `d` is
  the number of distinct elements in `t` (mandatory Section 3).
- State/effects: mutable bag/counter table with at most `k-1` distinct values
  before and after each iteration; input array is read, not mutated. The proposed
  implementation uses an AVL tree whose nodes are `(value,count)` (mandatory
  Sections 3-4).
- Time: AVL search/insert are O(log k); decrementing all `k` counters costs at
  most O(k log k) but occurs at most `n/k` times, yielding O(n log k) for the
  first pass. Verification is O(n log |t|), so the two-pass algorithm remains
  O(n log k). The paper states worst-case costs and discusses comparison-based
  optimality (mandatory abstract and Sections 1, 4-5).
- Space: AVL tree has at most k nodes and uses O(k) extra space (mandatory
  Section 4).
- Determinism/randomness: no randomness. A `k`-reduced bag need not be unique,
  but the described scan/update policy yields a valid candidate bag; exact
  verified output is independent of candidate representation.
- Variants: the paper gives a first algorithm with an unproven candidate-size
  bound and a second bounded algorithm; this worksheet uses the second. `k=2`
  is the majority special case. Candidate-only and verified outputs are distinct
  stages.
- Ambiguity: the paper calls its structure a bag while an implementation stores
  one node and a count per distinct value. Candidate identity/order is not part
  of the final semantic output.

## Proposed Atlas normalization

- `Problem` identity: `frequency.values_above_fraction.exact`.
- Exact `input`: replayable finite sequence of length `n` and integer `k` with
  `2 <= k <= n`.
- Exact `requires`: equality/comparison compatible across passes; sequence can be
  scanned twice (plus an optional prior length scan when n is unknown).
- Exact `output`: set of distinct values whose sequence multiplicity is strictly
  greater than `n/k`.
- Exact `ensures`: no qualifying value is omitted and no value at or below the
  threshold is returned.
- `Algorithm` identity: `frequency.misra_gries.repeated_elements_two_pass`.
- Algorithm requirements: bounded candidate table and replayable input for exact
  verification.
- Determinism: true for final set; candidate representation/order is unspecified.
- Time: O(n log k) worst case for the paper's comparison/AVL implementation.
- Memory: O(k) auxiliary state.
- Evidence: problem, invariant, bounds and strategy are paper-declared/argued;
  they are `declared` in Atlas absent a formal imported proof; no test performed.
- Implementation boundary: guarded-command Algorithm 3 plus AVL realization and
  second counting pass, not executable code/ABI. Effects: allocate/mutate O(k)
  counters/tree, read input twice, compare values, no input mutation or random
  effects.
- Candidate tests: threshold equality versus strictly greater, k=2, k=n,
  all-distinct, all-equal, adversarial counter cancellations, multiple qualifying
  values, first-pass false candidates and exact second-pass removal.
- Candidate relationships: a separate
  `frequency.misra_gries.candidates_one_pass` could solve a superset/candidate
  problem, but schema 0.1 cannot structurally relate it to exact verification.
- Information left documentary: AVL rotations, decision-tree lower-bound details,
  candidate ordering and the paper's first algorithm.

Alternative A, used above, models the exact two-pass result. Alternative B models
only the one-pass candidate superset; it is useful for streams but must not be
substituted for exact output. Alternative C creates both related problems, but
schema 0.1 lacks a refinement/composition relation, so no extra public entity is
requested during K-M4.

## Fidelity

### Bibliographic fidelity

Authors, title, venue, date, pages, DOI, readable scan and publisher mark are
preserved. The DOI landing page was unavailable. Assessment: **preserved**.

### Algorithmic fidelity

Strict threshold, candidate invariant, cancellation, second-pass verification,
AVL complexity and O(k) storage are retained. Assessment: **preserved**.

### Representational fidelity

Guarded-command bag operations become a bounded counter table and replayable
sequence contract; paper terms and stage boundary remain explicit. Assessment:
**intentionally transformed**.

### Executable fidelity

No upstream executable implementation is in scope. A direct frequency table is
the correction oracle for bounded fixtures. Nothing was executed. Assessment:
**not assessed**.

### Declared transformations

- Translation: guarded commands/bag notation to sequence and counter operations.
- Specialization: second bounded algorithm, excluding the first algorithm.
- Type adaptation: AVL total comparison is implementation-specific; abstract
  frequency semantics need equality.
- API aggregation: candidate generation plus verification form exact algorithm.
- Bug correction: none; packet transcription resolves the PDF extraction glyph.
- Pedagogical simplification: lower-bound proof omitted from executable contract.
- Other: candidate-only output is not mislabeled exact.

## Model friction

| Source fact | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| Strict parameterized threshold n/k | input/ensures prose | lossy | selection |
| Candidate superset versus exact verified set | output prose | ambiguous | identity, substitution |
| Two-pass/replayable input | requirements prose | lossy | selection, composition |
| Persistent bounded counters | memory/effects prose | lossy | selection |
| Stage composition/refinement | no relationship field | absent | composition |
| Complexity parameterized by k and comparison structure | one time string | lossy | selection |

Candidate-only and exact implementations could become indistinguishable, causing
false positives in returned results. A two-pass candidate could be selected for
a non-replayable stream, or a valid one-pass candidate generator rejected when a
superset is sufficient. Candidate/refinement distinctions independently occur in
Bloom tests and reservoir state, so they are not specific to frequency counting.

## Selection requests

1. “Return exactly all values occurring more than n/k from a replayable array,
   using O(k) auxiliary space.” **Accept**: use both candidate and verification
   passes; worst-case time is O(n log k) for the paper's AVL implementation.
2. “Consume a non-rewindable stream once and return no false candidate.”
   **Reject** for the full exact algorithm: the source requires a second pass to
   verify candidate counts.
3. “In one pass, return a bounded superset guaranteed to contain every value over
   n/k; false candidates are acceptable.” **Accept** the first stage only, not the
   exact problem identity; at most k-1 distinct candidates remain per iteration.

## Ambiguities and conclusion

- Source ambiguity: nonuniqueness/order of k-reduced candidate bags, which does
  not affect verified output.
- Protocol ambiguity: packet names the bounded-counter algorithm while the paper
  frames exact finding as two passes; this import preserves both stages.
- Model ambiguity: exact versus candidate output and replayability cannot be
  generic qualification predicates.
- Recommended normalization: exact threshold problem with a two-pass Misra-Gries
  algorithm; keep the one-pass candidate stage as a documented sub-capability.
- Minimal next probe: a sequence where cancellation leaves a nonqualifying
  candidate, proving why one-pass candidate and exact output must be distinct.
- Public schema change requested: **none**.
