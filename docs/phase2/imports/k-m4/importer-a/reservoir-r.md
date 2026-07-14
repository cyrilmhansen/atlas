# K-M4 import A: reservoir sampling Algorithm R

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` and `pdftotext`.
- Subject: Vitter's presentation of reservoir sampling Algorithm R, attributed
  there to Alan Waterman.
- Author/work: Jeffrey S. Vitter, “Random Sampling with a Reservoir,” *ACM
  Transactions on Mathematical Software* 11(1), March 1985, pages 37-57.
- DOI locator attempted but not readable (HTTP 403):
  <https://dl.acm.org/doi/10.1145/3147.3165>.
- Required readable locator actually read:
  <https://dsf.berkeley.edu/cs286/papers/reservoirsampling-toms1985.pdf>,
  especially Sections 1-3 and Table I.
- Supplemental pages consulted: none.
- Retrieval date: 2026-07-15.
- Source class: journal paper.
- Code license: not applicable to the pseudocode import. The paper states ACM
  copyright 1985 and a conditional noncommercial copying permission; no code is
  copied.

## Source-faithful account

- Problem: select a random sample of `n` records without replacement from a file
  of `N` records when `N` is unknown beforehand, processing the file in one
  sequential pass (mandatory abstract and Sections 1-2).
- Inputs: positive sample size `n`, a sequential record source of eventual length
  `N`, EOF operation, and a generator returning uniform real random variates
  (mandatory Algorithm R pseudocode, Section 2).
- Preconditions: at least `n` records must be available because the first `n` are
  read unconditionally. The truncation step requires a variate convention that
  yields an integer in `0..t-1`; the prose says unit interval but the endpoint
  convention is not explicit (mandatory Section 2).
- Output: the `n` records in candidate array `C` at EOF (mandatory Section 2).
- Guarantee: the candidates form a true random sample of the `N` record
  occurrences, without replacement. After each processed record, the `n`
  candidates are a true random sample of all records processed so far (mandatory
  Definition 1 and Algorithm R explanation).
- Strategy: put the first `n` records in `C`. For record `t+1`, draw a uniform
  integer among the `t+1` positions; if it is below `n`, replace that reservoir
  slot, otherwise skip the record. Equivalently, admit the new record with
  probability `n/(t+1)` and replace a uniformly selected candidate (mandatory
  Section 2).
- Named invariant: after every processed record, a true sample of size `n` can be
  extracted; Algorithm R maintains the designated `n` candidates themselves as
  that sample (mandatory Definition 1 and Section 2).
- State/effects: array of `n` candidate records, processed count `t`, current
  record/random variate; sequential reads/skips, random-number consumption and
  candidate replacement. A secondary-storage variant holds `n` pointers in
  internal memory and may sort them for final retrieval (mandatory Section 2).
- Time: Algorithm R scans all records and processes each in constant time, hence
  O(N) CPU time. Table I lists `N-n` average uniform variates and O(N) average CPU
  time. The paper's performance comparison explicitly ignores I/O time and
  explains a separate skip framework (mandatory Sections 1-3).
- Space: `n` candidate records, therefore O(n) storage under the pseudocode. The
  paper offers secondary storage plus O(n) internal pointers; it does not call
  Algorithm R O(1) in sample size.
- Determinism/randomness: randomized; correctness requires appropriate uniform
  choices at every processed record. Independence is implicit in the probability
  argument and explicit for the paper's later variates.
- Variants: secondary-storage representation and a skip-based reformulation;
  Algorithms X/Y/Z are distinct and outside this subject.
- Ambiguity: “true random sample” is not restated as a formal distribution over
  duplicate-valued records, but the algorithm operates on record occurrences.

## Proposed Atlas normalization

- `Problem` identity: `sampling.uniform_without_replacement.unknown_length`.
- Exact `input`: finite sequential stream of `N` record occurrences, whose `N`
  is initially unknown, and requested sample size `n`.
- Exact `requires`: `1 <= n <= N`; stream is consumed at most once in order; each
  random integer draw is independent and uniform over `0..t-1` at step `t`.
- Exact `output`: exactly `n` selected record occurrences, with no occurrence
  selected twice.
- Exact `ensures`: every size-`n` subset of the `N` positions has equal
  probability; equivalently each position has inclusion probability `n/N`.
- `Algorithm` identity: `sampling.reservoir.algorithm-r`.
- Algorithm requirements: one-pass EOF stream and uniform random generator.
- Determinism: false; a concrete implementation may be replayable with recorded
  random inputs, but that is not algorithmic determinism.
- Time: O(N) as source-declared, with constant work per record under the paper's
  operation model.
- Memory: O(n) candidate records as source representation; this is an inferred
  asymptotic restatement of an explicitly sized array, not a quoted paper bound.
- Evidence: probability invariant and O(N) are `declared`; uniformity is argued
  by the paper but not imported as an Atlas proof artifact; no executable claim.
- Implementation boundary: source pseudocode with `RANDOM`,
  `READ-NEXT-RECORD`, `SKIP-RECORDS` and array `C`, not a language ABI. Effects
  are sequential input I/O, random input consumption, O(n) persistent storage
  and in-place replacement; no allocation policy is defined.
- Candidate tests: all subsets for small N across exhaustive idealized random
  choices; stream length exactly n; duplicate values tracked by position; seeded
  replay; invalid short stream. Statistical tests alone do not prove uniformity.
- Information left documentary: physical I/O optimization, secondary-storage
  layout, exact PRNG, endpoint convention and Algorithm R authorship attribution.

## Fidelity

### Bibliographic fidelity

Author, title, venue, volume, issue, pages, DOI and readable author-paper locator
are preserved; DOI landing content was inaccessible. Assessment: **preserved**.

### Algorithmic fidelity

The initialization, admission/replacement rule, invariant, one-pass constraint,
randomness and O(N) processing are retained. Assessment: **preserved**.

### Representational fidelity

File records and Pascal-like primitives become an abstract finite stream, while
record occurrences remain distinct. Assessment: **intentionally transformed**.

### Executable fidelity

Only pseudocode is in scope. A parameterized random-choice oracle is proposed,
but no implementation was run. Assessment: **not assessed**.

### Declared transformations

- Translation: file primitives to stream/random-source operations.
- Generalization: physical record representation removed; unknown-length
  one-pass boundary retained.
- Type adaptation: random real/truncation expressed as uniform integer draw to
  avoid an unstated endpoint convention.
- API aggregation: initialization and per-record update form one algorithm.
- Bug correction: none; endpoint ambiguity is declared.
- Pedagogical simplification: secondary storage stays documentary.
- Other: exact uniform-subset wording makes the selection consequence explicit.

## Model friction

| Source fact | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| Uniform distribution over subsets | ensures prose | lossy | selection, substitution |
| Random-source law/independence | requires/deterministic | lossy | selection |
| Unknown N and one-pass stream | input/requires prose | lossy | selection, composition |
| O(n) persistent reservoir versus auxiliary memory | one memory string | ambiguous | selection |
| I/O ignored by CPU model but operationally required | effects prose | lossy | composition |
| Statistical evidence versus proof | evidence level lacks distribution artifact | absent | substitution |

A biased sampler and Algorithm R can look identical in coarse fields. An invalid
deterministic or multi-pass candidate could be selected. A valid seeded replay
could be rejected if determinism is treated as a single boolean. Random-law gaps
also occur independently in Bloom membership and hash collision costs, so they
are not reservoir-specific.

## Selection requests

1. “Sample exactly 100 occurrences uniformly without replacement from a
   one-pass stream of unknown length, using O(100) retained records.” **Accept**
   when the stream has at least 100 records and supplies uniform randomness.
2. “Produce the same sample without randomness for identical records while still
   guaranteeing a uniform sample.” **Reject**: Algorithm R is randomized; fixed
   replay requires an additional recorded/seeded random input contract.
3. “Use expected CPU work sublinear in N while reading an ordinary sequential
   stream.” **Reject**: Algorithm R processes every record and is O(N); the paper
   develops different skip algorithms for another I/O model.

## Ambiguities and conclusion

- Source ambiguity: random-real endpoint and formal treatment of equal-valued
  record occurrences.
- Protocol ambiguity: whether to normalize random real generation or preserve
  the paper's exact primitive; this import declares the adaptation.
- Model ambiguity: probability law, replayability and persistent sample storage
  are not structured.
- Recommended normalization: unknown-length uniform sampling problem and
  Algorithm R, separate from later skip algorithms.
- Minimal next probe: exhaustive small-N transition enumeration with symbolic
  uniform draws, plus a duplicate-value position test.
- Public schema change requested: **none**.
