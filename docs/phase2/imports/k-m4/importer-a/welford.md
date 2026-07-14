# K-M4 import A: Welford online corrected moments

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` with DOI content negotiation.
- Subject supplied by the packet: Welford online corrected sums of squares.
- Author/work: B. P. Welford, “Note on a Method for Calculating Corrected
  Sums of Squares and Products,” *Technometrics* 4(3), August 1962,
  pages 419-420.
- DOI: `10.1080/00401706.1962.10490022`.
- Source class: journal paper.
- Required locator read as bibliographic metadata:
  <https://doi.org/10.1080/00401706.1962.10490022>.
- Access result: Crossref CSL JSON was readable through content negotiation;
  the publisher landing page and PDF link returned HTTP 403. No article body,
  abstract, equations or examples were available.
- Supplemental pages consulted: none.
- Retrieval date: 2026-07-15.
- Code license: not applicable; no code was available.
- Article/documentation copyright: not stated in the accessible metadata.

## Source-faithful account

The only source-bounded substantive information available is the bibliographic
record and title. The source packet calls the subject “Welford online corrected
sums of squares”; the accessible DOI metadata itself does not expose “online,”
a recurrence, input type or correctness claim.

- Problem: calculating corrected sums of squares and products (mandatory DOI
  title). A more exact problem boundary is **not stated in accessible material**.
- Inputs and representation: **not stated**.
- Preconditions and numerical domain: **not stated**.
- Output/interface: corrected sums of squares and products are named by the
  title; exact definitions and normalization are **not stated**.
- Postconditions/guarantees: **not stated**.
- Strategy, recurrence and invariant: **not available**.
- Persistent/temporary state: **not stated**.
- Mutation, allocation, I/O and failure: **not stated**.
- Time and space claims: **not stated**.
- Determinism, rounding and arithmetic assumptions: **not stated**.
- Variants: **not stated**.
- Ambiguity: it is impossible from the accessible mandatory material to
  distinguish population/sample normalization, sums of squares versus variance,
  univariate versus product/covariance outputs, or exact versus floating-point
  arithmetic.

No common recurrence attributed to Welford is imported from memory or from an
unfrozen secondary source.

## Proposed Atlas normalization

This is deliberately a bibliographic placeholder, not an importable algorithm.

- Provisional `Problem` identity:
  `statistics.corrected_sums_of_squares_and_products`. It follows the paper title
  and avoids claiming a variance convention.
- Exact `input`: `not stated in accessible mandatory source`.
- Exact `requires`: `not stated in accessible mandatory source`.
- Exact `output`: `corrected sums of squares and products`, with mathematical
  definition unresolved.
- Exact `ensures`: `not stated in accessible mandatory source`.
- Provisional `Algorithm` identity: `statistics.welford_corrected_moments`, for
  documentary lookup only; it must not enter selection.
- Algorithm requirements, determinism, time and memory: all `not stated`.
- Proposed implementation identity: none; no executable source is available.
- Evidence: author/title/venue/date/pages/DOI may be `declared`; no algorithmic
  claim has sufficient evidence level.
- Candidate relationships: the packet suggests an online-moments family, but no
  relation is proposed without source text.
- Information intentionally left documentary: all source-packet terminology
  beyond the DOI metadata, plus the HTTP access failure and timestamp.

Schema 0.1 requires algorithm `deterministic`, `time_worst` and
`auxiliary_memory`. Filling them would fabricate evidence. The least-lossy
temporary representation is this rejected documentary placeholder outside the
public manifest.

## Fidelity

### Bibliographic fidelity

Author, title, journal, volume, issue, date, pages and DOI are preserved from
Crossref metadata. Article copyright and a readable archival edition are absent.
Assessment: **preserved** for metadata.

### Algorithmic fidelity

No algorithm body or conditions were available, so strategy and correctness
cannot be compared. Assessment: **unresolved**.

### Representational fidelity

Only title vocabulary is retained; no equation is translated. Assessment:
**unresolved**, rather than lossy reconstruction.

### Executable fidelity

No upstream implementation or source examples were accessible. No oracle can be
defined source-faithfully from metadata alone. Nothing was executed. Assessment:
**not assessed**.

### Declared transformations

- Translation: title phrase mapped to a provisional Atlas ID only.
- Specialization/generalization: none.
- Type/representation adaptation: none.
- API decomposition/aggregation: none.
- Bug correction: none.
- Pedagogical simplification: none.
- Other: source-packet word “online” is not promoted to a source claim.

## Model friction

| Source fact or absence | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| Paper exists with named subject | claim source/provenance | lossy | identity, documentary only |
| Exact mathematical output unavailable | problem output/ensures | absent | selection, substitution |
| Numeric domain/rounding unavailable | requirements | absent | selection |
| Determinism/time/memory unavailable | required algorithm claims | absent | identity, selection |
| Transformation from paper to code unavailable | no structured provenance | absent | substitution |

Any concrete candidate would become indistinguishable from an unsupported
attribution if forced into schema 0.1. Invalid numerical implementations could
be selected, while a valid implementation could not be recognized. Numerical
domain gaps also arise independently in probabilistic counters and hash-derived
methods, but source access must be repaired before model conclusions are drawn.

## Selection requests

1. “Select the paper's exact online recurrence for corrected sums of squares and
   products.” **Reject for now**: the recurrence was not accessible.
2. “Require a documented floating-point error or stability guarantee.”
   **Reject**: no numerical premise or bound is source-visible.
3. “Choose an executable one-pass implementation with O(1) state and a correction
   oracle.” **Reject**: neither one-pass/O(1) claims nor implementation/oracle can
   be sourced from the accessible material.

These are evidence failures, not evidence that the algorithm lacks the requested
properties.

## Ambiguities and conclusion

- Source ambiguity: all semantic fields remain unresolved because the article
  body was inaccessible; this is primarily source availability, not contradictory
  wording.
- Protocol ambiguity: the frozen packet provides no readable author copy or
  excerpt for this subject, unlike three other paper subjects.
- Model ambiguity: even with the article, schema 0.1 has no structured numeric
  error/arithmetic-domain model.
- Recommended normalization: retain only a bibliographic worksheet until an
  authorized readable primary copy is added to a future frozen packet.
- Minimal next experiment: obtain a lawful readable primary copy, freeze its
  locator, and repeat this worksheet independently before any comparison.
- Public schema change requested: **none**. No model change should be inferred
  from a source-access failure.
