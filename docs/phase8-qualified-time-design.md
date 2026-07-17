# Phase 8 - Qualified time design

Status: design complete; public schema choice pending (2026-07-17)

## Question

What is the smallest public representation that preserves the validity
conditions of time-complexity claims and lets Atlas distinguish the two
confirmed cases without introducing a general expression language?

## Required discriminants

The representation must encode all of these without family-specific fields:

1. heap push is worst `O(n)` without qualification and worst `O(log n)` when
   `state.spare_capacity` is supplied;
2. hash-map insert is expected `O(1)` only under
   `workload.nonadversarial_hash_distribution`;
3. merge sort remains unconditionally worst `O(n log n)`;
4. every individual profile retains its own evidence level and source;
5. absence of a required condition remains unsupported, never inferred.

The private overlay already verifies the decision semantics for exact condition
sets. The missing public concern is stable condition identity and provenance,
not another evaluator.

## Option A - Qualified time claims

Add first-class condition definitions and replace `time_worst` and
`time_expected` with a list of time profiles on each Algorithm:

```yaml
conditions:
  - id: state.spare_capacity
    statement:
      value: "the destination state can accept the operation without growth"
      level: declared
      source: "definition:state.spare_capacity"

time:
  - value:
      regime: worst
      bound: "O(log n)"
      requires: [state.spare_capacity]
    level: inferred
    source: "docs:phase4/imports/k4-m2/dary-heap-push.md"
```

`regime` is initially limited to `worst`, `expected`, and `amortized`.
`requires` is a set of references to globally unique condition IDs; an empty
set is unconditional. The list belongs to Algorithm, so an `operation` field is
redundant with its `solves` problem and is deliberately omitted.

This shape expresses all five discriminants. It adds only the concept required
by the two-family evidence and preserves Atlas claim provenance at profile
granularity.

## Alternatives

**Option B - Generic cost profiles.** Use the same condition references but add
`metric: time | auxiliary_memory | retained_memory | allocation`. This aligns
with the private overlay and may later unify resource selection. It is broader
than the demonstrated public need and immediately raises unresolved units and
bound semantics for non-time metrics.

**Option C - Keep costs in a derived overlay.** This avoids migration, but Phase
5 through 7 show that authoritative registry projection then remains lossy. It
cannot satisfy the goal of source-independent public selection.

## Recommendation and cost

Recommend **Option A** for a schema 0.2 experiment. It is narrower than the
generic overlay, represents both independent families, and does not require
parsing predicates: callers assert condition IDs, while Atlas checks identity,
evidence and set inclusion.

A real adoption is structural and breaking because schema 0.1 readers reject
unknown fields and because dual `time_worst`/`time_expected` authority would be
ambiguous. Adoption therefore requires:

- a deterministic whole-registry migration to schema 0.2;
- validation of condition IDs, references, regimes and duplicate profiles;
- updates to SQLite and Web projections plus CLI rendering;
- explicit conversion of every existing time claim to a profile with
  `requires: []` unless source review proves a qualification.

The design remains reversible until that migration is accepted. The main risk
is vocabulary quality: a condition ID can still hide an ambiguous statement.
The minimum pre-migration check is to encode heap push, hash-map insert and one
unconditional sort in a disposable fixture and confirm the five discriminants
with the unchanged evaluator.

## Verdict

The two-family evidence supports qualified time profiles, not yet generic cost
profiles or a predicate DSL. Phase 8 has produced a migration-ready design but
does not modify schema 0.1. Public adoption awaits the owner choice below.
