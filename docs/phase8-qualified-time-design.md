# Phase 8 - Qualified cost design

Status: Option B accepted; disposable fixture supported (2026-07-17)

## Question

What is the smallest public representation that preserves validity conditions
and provenance across time and resource-cost claims without introducing a
general expression language?

## Required discriminants

The representation must encode all of these without family-specific fields:

1. heap push is worst `O(n)` without qualification and worst `O(log n)` when
   `state.spare_capacity` is supplied;
2. hash-map insert is expected `O(1)` only under
   `workload.nonadversarial_hash_distribution`;
3. merge sort remains unconditionally worst `O(n log n)` with `O(n)` auxiliary
   memory;
4. heap push performs no allocation when `state.spare_capacity` is supplied;
5. every individual profile retains its own evidence level and source;
6. absence of a required condition remains unsupported, never inferred.

The private overlay already verifies the decision semantics for exact condition
sets. The missing public concern is stable condition identity and provenance,
not another evaluator.

## Accepted shape - Generic cost profiles

The owner selected Option B. Add first-class condition definitions and replace
the scalar time and auxiliary-memory fields with cost profiles on each
Algorithm:

```yaml
conditions:
  - id: state.spare_capacity
    statement:
      value: "the destination state can accept the operation without growth"
      level: declared
      source: "definition:state.spare_capacity"

costs:
  - value:
      metric: time
      regime: worst
      bound: "O(log n)"
      requires: [state.spare_capacity]
    level: inferred
    source: "docs:phase4/imports/k4-m2/dary-heap-push.md"
```

`metric` is initially limited to `time`, `auxiliary_memory`,
`retained_memory`, and `allocation`. `regime` is initially limited to `worst`,
`expected`, and `amortized`. `requires` is a set of references to globally
unique condition IDs; an empty set is unconditional. The list belongs to
Algorithm, so an `operation` field is redundant with its `solves` problem and
is deliberately omitted.

Bounds remain sourced strings: this design structures scope and identity but
does not claim algebraic comparison, common units or total ordering. Every
profile retains Atlas claim provenance.

## Alternatives

**Option A - Qualified time only.** Narrower, but it leaves the already observed
capacity-conditioned allocation fact in a second representation.

**Option C - Keep costs in a derived overlay.** This avoids migration, but Phase
5 through 7 show that authoritative registry projection then remains lossy. It
cannot satisfy the goal of source-independent public selection.

## Fixture result and cost

A disposable three-candidate fixture uses the unchanged evaluator for five
positive decisions: heap time and allocation with spare capacity, hash-map
expected time with nonadversarial hashing, and unconditional merge-sort time and
auxiliary memory. Removing either supplied condition rejects the corresponding
request. Per-profile evidence is retained. Adding `auxiliary_memory` to the
private metric vocabulary is the only evaluator-model change; matching logic is
unchanged.

A real adoption is structural and breaking because schema 0.1 readers reject
unknown fields and because dual `time_worst`/`time_expected` authority would be
ambiguous. Adoption therefore requires:

- a deterministic whole-registry migration to schema 0.2;
- validation of condition IDs, references, regimes and duplicate profiles;
- updates to SQLite and Web projections plus CLI rendering;
- explicit conversion of every existing time claim to a profile with
  `requires: []` unless source review proves a qualification.

The design remains reversible until migration is accepted. Main risks are
condition-vocabulary ambiguity and misleading cross-metric comparisons. Metric
and bound comparisons must remain exact unless a later independent experiment
justifies more semantics.

## Verdict

The accepted generic profile represents all six discriminants without a
predicate DSL or evaluator branch per metric. Phase 8 closes supported at the
disposable-fixture level. Schema 0.1 remains unchanged; an authoritative schema
0.2 migration is a separate structural action.

Verification:

```text
cargo test -p atlas generic_cost_fixture_covers_time_memory_and_allocation --locked --offline
```
