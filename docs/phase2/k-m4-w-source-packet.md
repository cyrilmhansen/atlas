# K-M4-W independent online-moments source packet

Status: frozen for the addendum experiment

Protocol: `k-m0.2`

Batch size: one subject imported independently twice

## Purpose

K-M4 left Welford's 1962 paper unresolved because the frozen locator exposed
metadata but not the article body. K-M4-W does not rewrite that historical
result and does not claim textual fidelity to Welford. It tests whether openly
readable primary reports support a source-bounded normalization of incremental
second central moments and their pairwise combination.

## Isolation boundary

Two importers receive this exact packet. Each must work independently and must
not inspect:

- `registry/atlas.yaml`;
- `docs/phase2/k-m0-comparison.md`;
- `docs/phase2/k-m1-graph-corpus.md`;
- `docs/phase2/k-m2-dynamic-structures.md`;
- `docs/phase2/k-m3-streaming-approximation.md`;
- `docs/phase2/k-m4-dual-import-comparison.md`;
- `docs/phase2/imports/`, including the K-M4 Welford worksheets;
- the K-M4-W output directory of the other importer;
- Git history or diffs revealing those files.

Allowed local references:

- this packet;
- `docs/phase2/import-worksheet.md`;
- `docs/phase2/import-equivalence-rubric.md`;
- `docs/phase2/current-model-baseline.md`;
- `docs/schema-0.1.md`;
- `docs/vision.md` only when the preceding documents leave a term undefined.

Web access is allowed only for the frozen source locators below and links
directly reachable from their landing pages. General knowledge must not fill a
source gap.

## Frozen subject

### Incremental second central moment

Subject boundary:

- maintaining count, mean and the corrected sum of squared deviations while
  observations arrive incrementally;
- obtaining a population or unbiased sample variance only as a separately
  qualified finalization;
- recording pairwise state combination as a related source capability rather
  than silently treating it as the same operation;
- distinguishing exact-arithmetic identities from floating-point accuracy
  claims.

The import must not use `Welford` as the normalized algorithm identity merely
because later literature associates the recurrence with that name. Welford 1962
remains historical metadata unless its text is actually read in a later packet.

## Mandatory primary sources

### Pébay 2008

Philippe Pébay, *Formulas for Robust, One-Pass Parallel Computation of
Covariances and Arbitrary-Order Statistical Moments*, Sandia report
SAND2008-6212, Unlimited Release, September 2008.

- official public copy:
  <https://www.osti.gov/servlets/purl/1028931/>;
- frozen PDF SHA-256 observed on 2026-07-15:
  `db5dfcd01b3bdd21071930fb14a772968dedb33e062286d408844f3c7fa0c297`;
- mandatory pages: printed pages 7-11;
- mandatory items: equations (1.1)-(1.4), Proposition 2.1, Corollary 2.2 and
  the specialization that retrieves equation (1.2) for the second moment;
- source role: normative algorithmic account of the incremental and pairwise
  centered-moment updates.

### Chan, Golub and LeVeque 1979

Tony F. Chan, Gene H. Golub and Randall J. LeVeque, *Updating Formulae and a
Pairwise Algorithm for Computing Sample Variances*, Stanford technical report
STAN-CS-79-773, November 1979.

- readable archival copy:
  <https://ftpmirror.your.org/pub/misc/bitsavers/pdf/stanford/Stanford_CS_TR_Collection_2025-12-12/OCR/CS-TR-79-773-ocr.pdf>;
- frozen PDF SHA-256 observed on 2026-07-15:
  `07ee77389bfafb982198b29275d3ee5ef8e1fab21698d06f03a557620f43574e`;
- mandatory pages: report sections 1-3, PDF pages 5-10;
- mandatory items: definition (1.1), updating discussion and formulas
  (1.5)-(1.6), general combination formulas (2.1), and the pairwise algorithm;
- source role: independent primary account of updating/pairwise variance
  computation, operation counts and numerical motivation. It must not be used
  to attribute Pébay's notation or the inaccessible Welford text.

An archival copy being readable does not imply a permissive software license.
Import facts and formulas with citations; do not copy prose or the report's
Fortran implementation into Atlas.

## Required deliverable

Each importer creates exactly one worksheet named `online-second-moment.md` in
its assigned output directory. It must follow the manual worksheet and include:

1. timestamps and every source locator actually read;
2. bibliographic, algorithmic, representational and executable fidelity;
3. every declared transformation;
4. proposed `Problem` identity and exact input/requires/output/ensures;
5. proposed incremental `Algorithm` identity, requirements, determinism, time
   and memory;
6. an explicit decision whether pairwise combination is the same algorithm, a
   related algorithm, a stage or documentary capability;
7. exact-arithmetic identities separated from every floating-point statement;
8. population variance, unbiased sample variance and corrected sum of squares
   kept distinct, including empty and singleton domains;
9. information schema 0.1 cannot represent without lossy prose;
10. expected decisions for the three requests below;
11. ambiguity classified as source, protocol or model ambiguity;
12. explicit statement that no public schema change is requested.

## Common selection requests

Both importers must adjudicate these exact requests:

1. "Consume a finite real-valued stream once and return its count, mean and
   corrected sum of squared deviations using O(1) retained scalar state."
2. "Return an unbiased sample variance for every input length, including empty
   and singleton streams, without a partial or error result."
3. "Merge independently accumulated partitions and require bit-for-bit equality
   with every sequential evaluation order under IEEE-754 arithmetic."

For each request, state `accept`, `accept conditionally` or `reject`, followed by
the exact source-backed reason. Do not turn an empirical accuracy observation
into a universal floating-point guarantee.

## Submission rule

After writing the worksheet, run whitespace checks only over the importer's own
output directory. Send the root agent a completion message with the file path
and the three most decision-relevant normalization choices. Do not read peer or
comparison output until both submissions are frozen.
