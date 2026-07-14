# K-M4 importer B: reservoir sampling Algorithm R

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
- Subject: Vitter Algorithm R, attributed in the paper to Alan Waterman.
- Bibliography: Jeffrey Scott Vitter, "Random Sampling with a Reservoir," *ACM
  Transactions on Mathematical Software* 11(1), March 1985, pp. 37-57.
- Mandatory locators actually read:
  - <https://dl.acm.org/doi/10.1145/3147.3165>
  - <https://dsf.berkeley.edu/cs286/papers/reservoirsampling-toms1985.pdf>
- Supplemental pages: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper with Pascal-like pseudocode.
- Code license: no reusable software license stated. Paper copyright is ACM 1985;
  its printed copying notice is not treated as a code license.

## 2. Source-faithful account

- Problem: choose without replacement a random sample of size `n` from `N`
  sequential records when `N` is unknown beforehand, in one sequential pass
  (paper Introduction, pp. 37-38, mandatory PDF).
- Inputs: record stream/file of ultimately `N` records, requested sample size
  `n`, EOF operation, and independent uniform random variates; pseudocode assumes
  the first `n` records can be read (Section 2, pp. 39-40).
- Requires: `N >= n`; sequential access and a uniform random generator whose
  truncation produces the stated integer range. These are implied by algorithm
  initialization and probability statement; malformed/short input behavior is
  not defined.
- Output: array `C[0..n-1]` of candidate records at EOF (Section 2, p. 39).
- Ensures: the candidates are a true random sample of the N records; after each
  processed record, current n candidates form a true random sample of all records
  processed so far (Definition 1 and Algorithm R, p. 39).
- Strategy: first n records become candidates. For record `t+1`, include it with
  probability `n/(t+1)` and, if selected, replace one uniformly chosen candidate;
  otherwise skip it (Section 2, p. 39).
- Named invariant: after `t >= n` processed records, the n candidates are a true
  random sample of the first t records (Section 2, p. 39).
- State/effects: mutable n-record candidate array and counter t; reads/skips every
  remaining record, consumes random variates, replaces candidates, and performs
  stream I/O. A secondary-storage/pointer variant is described when candidates
  do not fit internal memory (Section 2, pp. 39-40).
- Time: Algorithm R O(N), because the complete file is scanned and each record
  takes constant processing (Section 2, p. 40). Paper performance analysis
  explicitly ignores I/O time when comparing CPU time (Introduction, p. 38).
- Space: the direct pseudocode stores n candidate records; the secondary-storage
  variant assumes internal space for n pointers. No "constant space" claim for R
  independent of sample size is made.
- Determinism/randomness: randomized; exact uniform-without-replacement guarantee
  relies on the stated random choices. Output order semantics are not specified.
- Variants: direct in-memory reservoir and secondary-storage candidates with
  in-memory pointers; the paper distinguishes later Algorithms X/Y/Z.
- Source ambiguity: "true random sample" depends on RNG idealization; failure,
  seeding and reproducibility are not specified. O(N) is CPU-processing analysis
  with I/O separately excluded.

## 3. Proposed Atlas normalization

- `Problem` identity: `stream.uniform_sample_without_replacement_unknown_length`.
- Exact input: `single-pass stream of N records, target n, and source of uniform
  independent random choices`.
- Exact requires: `0 <= n <= N; each stream record is readable once; random
  integer choices realize the probabilities specified by Algorithm R`.
- Exact output: `n selected records`.
- Exact ensures: `every size-n subset of the N input positions has equal
  probability; no position is selected twice`.
- `Algorithm` identity: `stream.reservoir_sampling.algorithm_r`.
- Requirements: exact problem requirements; direct variant stores n records.
- Determinism: `false`; distributional guarantee requires randomness.
- Time: worst O(N) record-processing steps; I/O latency explicitly excluded by
  the source analysis. Random calls are one per processed post-initial record in
  the shown formulation.
- Auxiliary memory: O(n) records for direct candidate array; source uses n as
  sample size, not constant. Secondary variant uses O(n) pointers internally and
  external storage.
- Implementation identity: none. The paper gives pseudocode, not an executable
  artifact imported into Atlas.
- Boundary/effects: consumes stream once, performs input/skip I/O, mutates
  reservoir, uses randomness, may write external storage in the variant; no
  allocation API/failure contract.
- Tests: exhaustive small-N distribution enumeration with controlled random
  choices, empirical distribution only as observation, N=n, n=0 ambiguity,
  short stream, duplicate record values tracked by position, and deterministic
  replay with fixed random sequence.
- Evidence: problem, invariant, strategy, O(N), and storage forms `declared` by
  mandatory paper; no `tested` or `proven` Atlas evidence assigned here.
- Existing Atlas synonym: not assessed under experiment boundary.
- Documentary only: EOF/RNG API, record size, output order, secondary-storage
  mechanics, and CPU/I/O cost decomposition.

## 4. Fidelity and transformations

### Bibliographic fidelity

Authors, title, venue, volume, issue, pages, DOI and readable copy are preserved.
Assessment: `preserved`.

### Algorithmic fidelity

One-pass boundary, probability, replacement, invariant and uniform sample are
preserved. Assessment: `preserved` under the ideal RNG assumption.

### Representational fidelity

File operations and Pascal array become stream/reservoir abstractions; the
secondary-storage variant remains documentary. Assessment: `intentionally
transformed`.

### Executable fidelity

Only pseudocode is present; no upstream executable fixture was run. Oracle is a
distribution over input positions, not values, plus invariant checks after each
prefix. Assessment: `not assessed`.

### Declared transformations

- Translation: paper pseudocode to stream state transition.
- Specialization/generalization: Algorithm R only; X/Y/Z excluded.
- Type adaptation: file records become opaque stream records; RNG requirement is
  explicit.
- API aggregation/decomposition: initialization and update form one algorithm.
- Bug correction: none.
- Pedagogical simplification: external-storage mechanics omitted from core.
- Other: source's `n` sample size and `N` population notation retained.

## 5. Schema-loss record

Schema 0.1 cannot structurally represent probability distributions, randomized
postconditions, RNG assumptions, one-pass streams, prefix invariants, I/O cost
exclusion, or memory parameterized by sample size versus record bytes. Free-text
`deterministic: false` alone cannot distinguish uniform sampling from any random
subset procedure.

## 6. Selection requests

1. **Request:** uniform size-n sample from an unknown-length one-pass stream.
   **Accept** when N>=n and ideal random choices are available.
2. **Request:** reproducible sample without providing seed/random sequence.
   **Reject**: Algorithm R is randomized and the paper gives no seed protocol.
3. **Request:** O(1) memory independent of requested sample size and record size.
   **Reject**: direct R stores n candidates; the external variant still keeps n
   pointers internally.

## 7. Ambiguities and conclusion

- Source ambiguity: n=0, stream shorter than n, RNG finite precision, failure and
  output ordering are not specified.
- Protocol ambiguity: whether paper proof-level reasoning maps to `proven`; this
  worksheet conservatively records source claims as `declared`.
- Model ambiguity: randomized distributional ensures and resource accounting
  cannot be qualified structurally.
- Decision-relevant divergence: treating memory as constant or omitting the
  uniformity/RNG contract changes candidate acceptance.
- Public schema change requested: **none**; randomized guarantees also require a
  second structural family before any proposal.
