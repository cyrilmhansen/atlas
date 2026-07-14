# K-M3 streaming and approximation corpus batch

Status: complete  
Recorded: 2026-07-15  
Protocol: `k-m0.2`

## Scope

K-M3 imports four streaming subjects through six explicit contracts. The
registry grows from 25/30/34 to 31 problems, 36 algorithms and 40
implementations.

| Subject | Result class | Persistent budget | Principal pressure |
|---|---|---|---|
| bounded top-k | exact | `O(k)` retained values | output projection and tie multiplicity |
| online population moments | floating approximation to exact real moments | `O(1)` | rounding, overflow and conditioning |
| reservoir Algorithm R | exact uniform distribution under ideal random draws | `O(k)` | randomness and seed-conditioned replay |
| Bloom membership | one-sided approximate answer | `O(m)` bits | false-positive probability and hash assumptions |

Top-k, moments and reservoir sampling are whole-stream problems. Bloom remains
a stateful family with separate construct, insert and query contracts. No AST,
runtime, visual program or Web behavior is added.

## Sources

- Rust 1.85 API baseline
  [`BinaryHeap`](https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html)
  for the bounded minimum-heap adapter;
- B. P. Welford, “Note on a Method for Calculating Corrected Sums of Squares
  and Products”, *Technometrics* 4(3), 1962,
  [DOI 10.1080/00401706.1962.10490022](https://doi.org/10.1080/00401706.1962.10490022);
- Jeffrey S. Vitter, “Random Sampling with a Reservoir”, *ACM TOMS* 11(1),
  1985, [DOI 10.1145/3147.3165](https://dl.acm.org/doi/10.1145/3147.3165);
- Burton H. Bloom, “Space/Time Trade-offs in Hash Coding with Allowable
  Errors”, *Communications of the ACM* 13(7), 1970,
  [DOI 10.1145/362686.362692](https://dl.acm.org/doi/10.1145/362686.362692).

The papers are bibliographic and semantic sources. Their prose or code is not
copied. The executable adapters are independently written, test-only MIT code
and are not presented as unchanged external implementations.

## Exact bounded top-k

The adapter keeps a minimum heap of at most `k` elements. A new value replaces
the retained minimum only when strictly greater. The output preserves duplicate
occurrences and contains the greatest `min(k, n)` input occurrences.

- worst-case time: `O(n log k)` inferred from at most one heap replacement per
  item;
- persistent working storage: `O(k)`;
- deterministic under a total order;
- capacity zero and `k > n` are tested explicitly.

The returned descending sequence is a presentation choice of the adapter, not
a universal top-k contract. The problem guarantees membership, not output
ordering.

## Online moments and numerical failure regimes

The Welford adapter maintains `(count, mean, M2)` and derives population
variance as `M2 / count`. Under exact real arithmetic the recurrence produces
the corresponding moments. The executable implementation uses binary64 and
does not promote that theorem to tested bit-exact evidence.

Two tests distinguish those levels:

- a small integer-valued fixture produces mean `5` and variance `4` exactly;
- a `10^12` offset with variance `4` produces approximately
  `3.9999618530273438`, while remaining substantially more accurate than the
  cancellation-prone `E[x^2] - E[x]^2` formula.

Visible failure conditions include non-finite inputs, overflow in deltas or
`M2`, loss of exact integer count conversion above the binary64 significand
range and ill-conditioned data whose spread is tiny relative to its offset.
No universal floating-point error bound is claimed by the registry.

## Reservoir randomness

The adapter implements Vitter Algorithm R and supplies random integers using a
seeded SplitMix64 stream plus rejection sampling. It retains at most `k` values
and processes the input once.

The semantic algorithm is randomized: with independent uniform draws, every
input position has the same inclusion probability. The concrete adapter is
reproducible when its seed, generator and input order are fixed. Tests establish
same-seed replay, different observed samples for two selected seeds, bounded
storage and membership in the input. They do not statistically prove generator
quality or uniformity.

Schema 0.1 has only one `deterministic` boolean. The algorithm is therefore
recorded `false`; seed-conditioned determinism remains in the problem guarantee,
implementation signature and this report.

## Bloom one-sided approximation

The filter allocates exactly `m` persistent bits, sets `h` positions per insert
and tests the same positions per query. The contract distinguishes
`DefinitelyAbsent` from `PossiblyPresent` rather than returning an exact
membership boolean.

- inserted values have no false negatives while state and hash behavior remain
  valid;
- false positives are explicitly allowed;
- under independent uniform hash positions, the recorded conventional estimate
  is `(1 - exp(-h n / m))^h`;
- construction uses `O(m)` bits, insertion and query use `O(h)` time and `O(1)`
  auxiliary storage outside the filter.

The executable adapter uses deterministic salted SplitMix64 positions. Its test
proves no false negatives for the fixture and observes a false positive in an
intentionally saturated 32-bit filter. That observation does not validate the
general probability formula or independence assumption.

## Fidelity and transformations

### Bibliographic fidelity

Authors, titles, venues, years and DOI locators are preserved. The standard
library API is versioned to the project baseline.

### Algorithmic fidelity

The Welford recurrence, Algorithm R replacement rule and Bloom bit-field
semantics are retained. Top-k is an Atlas normalization over standard heap
operations rather than an algorithm named by the Rust documentation.

### Representational fidelity

Paper pseudocode and notation are normalized to problem contracts and small Rust
adapters. Randomness becomes an explicit input. Bloom construction, insertion
and query stay separate. Numerical and probabilistic qualifications remain
textual because schema 0.1 has no typed equivalents.

### Executable fidelity

Tests cover exact fixtures, bounded state, seed replay, a numerical stress case,
one-sided Bloom behavior and an observed false positive. Distributional and
floating-point theorems remain declared source claims, not test claims.

### Declared transformations

- population variance is selected instead of sample variance;
- Algorithm R is paired with a specific seeded pseudo-random generator and
  unbiased range reduction not prescribed as the universal algorithm;
- Bloom hashing uses deterministic salts and does not claim cryptographic or
  HashDoS resistance;
- adapters materialize finite slices for deterministic tests even though the
  algorithm contracts are streaming.

## Model-friction matrix

| Missing typed concept | Severity | Decision consequence |
|---|---|---|
| seed-conditioned determinism and random-source identity | high | replay compatibility cannot be selected generically |
| one-sided versus two-sided approximation | high | exact and approximate candidates cannot be safely substituted by query |
| error probability with assumptions and parameters | high | accuracy targets cannot qualify candidates |
| floating format, rounding mode and numerical error bound | high | numerically unsuitable implementations cannot be rejected generically |
| symbolic memory budget parameter (`k`, `m`) | medium | numeric limits remain prose |
| stream cardinality known/unknown and single-pass requirement | medium | composition cannot prove streaming compatibility |

K-M1 already exposed numerical-domain prose, K-M2 exposed untyped cost/state
scope, and K-M3 independently confirms decision-changing losses for numerical,
probabilistic and parameterized guarantees. This is sufficient evidence to
prepare schema alternatives, but not to select one without the K-M4 dual-import
experiment.

## Acceptance

- deterministic, randomized and seed-conditioned guarantees distinct:
  complete, with the schema limitation explicit;
- accuracy/error and memory bounds retain provenance: complete;
- numerical assumptions and failure regimes visible: complete;
- schema inability recorded before extension: complete;
- no schema, AST, MIR, WASM, Explorer or planner extension: complete.

K-M3 closes on this evidence. K-M4 should now measure whether independent
importers normalize these decision-relevant distinctions consistently before a
public schema proposal is drafted.
