# K-M5 decision-overlay evaluator checkpoint

Status: retained private evidence; K-M5 closed mixed under DEC-070

Date: 2026-07-15

## Result

The private overlay describes 30 atoms, 8 candidates, 4 directional relations
and 10 requests spanning the seven accepted ontology discriminants. One generic
evaluator discovers all candidates from YAML and reports an accepted decision
or explicit rejection reasons. It contains no candidate ID, algorithm family or
source-family branch.

Five requests accept at least one candidate and five deliberately reject all
candidates:

| Request | Accepted candidates | Decision exercised |
|---|---|---|
| Dijkstra distances | path-tree candidate | directional output projection |
| exact heavy hitters | verified second-pass candidate | qualified stage output |
| heap push without allocation | heap-push candidate | conditioned allocation cost |
| individual heap push in `O(log n)` | none | amortized/worst and capacity boundary |
| disjoint-set union | union candidate | typed persistent-state continuity |
| exact Bloom membership | none | approximate output is not exact |
| definitive Bloom negative | Bloom-query candidate | one-sided guarantee projection |
| unbiased variance for `n < 2` | none | finalizer domain condition |
| bitwise-order-independent merge | none | numerical-order guarantee |
| proof-only pairwise merge | none | exact evidence-level filtering |

A test mutates the Bloom request in parsed YAML and changes the result without
recompiling Rust. Another test resolves each candidate source against either the
authoritative registry or a committed import worksheet. The overlay therefore
does not silently become a second catalog.

## Cost

| Component | Size |
|---|---:|
| Overlay YAML | 319 lines |
| Typed model, parser, errors and validator | 596 non-test Rust lines |
| Generic evaluator | 230 non-test Rust lines |
| Runtime dependencies added | 0 |
| Public commands or schema fields added | 0 |

The evaluator itself stays within the experiment's 300-line limit, but the full
typed experiment costs 826 non-test Rust lines. K-M5 must compare that cost with
independent authoring agreement and the decisions corrected; passing these
tests alone does not justify schema 0.2.

DEC-069 subsequently adds private encoding equivalences. The complete parser,
evaluator and resolver now total 1,146 non-test Rust lines; the incremental
result and control are in `k-m5-normalization-b.md`. The initial counts above
remain the pre-equivalence baseline.

## Verification

- At the pre-equivalence baseline, `cargo test -p atlas --locked`: 152 tests
  pass across the library and four integration suites;
- `cargo check -p atlas --all-targets --locked`: passes;
- `cargo fmt --all -- --check` and `git diff --check`: pass.

## Boundaries

- Schema 0.1 controls remain documented adjudications, not executable queries.
- Relations are direct and directional; there is no transitive closure.
- Cost bounds are exact strings and condition sets; there is no asymptotic
  ordering or arithmetic.
- Evidence levels form no implicit trust lattice.
- Candidate sources are resolved, but individual evidence locators are not yet
  dereferenced to exact source fragments.
- The overlay has no CLI, persistence promise or registry authority.

## Next experiment

The independent result in `k-m5-independent-authoring-result.md` motivated
DEC-069. Bounded bidirectional equivalences now reconcile both observed top-k
encodings across four cross-encoding requests, at an incremental cost of 320
non-test Rust lines. The conditioned allocation follow-up is now complete in
`k-m5-heap-condition-result.md`; `k-m5-review.md` recommends mixed closure.
