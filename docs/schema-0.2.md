# Atlas aggregate schema 0.2

Schema 0.2 is the authoritative UTF-8 YAML format for Atlas Knowledge. It keeps
the aggregate, in-memory validated document and the four separate `problems`,
`algorithms`, `implementations` and `executions` collections from schema 0.1.
It adds a `conditions` vocabulary and replaces scalar algorithm cost fields with
generic qualified profiles.

## Identity and claims

`Condition`, `Problem`, `Algorithm` and `Implementation` IDs share one global
namespace. IDs contain lowercase ASCII letters, digits, dots, underscores or
hyphens. `solves`, `implements` and cost-condition references must resolve to an
entity of the required kind.

Every non-structural property remains a claim with `value`, `level` and
`source`. Evidence levels and provenance-source validation are unchanged from
schema 0.1.

## Conditions

A condition gives a stable identity to an assumption supplied by a selection
request. Its statement is descriptive and sourced; Atlas does not execute it as
a predicate.

```yaml
conditions:
- id: state.spare_capacity
  statement:
    value: the destination state can accept the operation without growth
    level: declared
    source: definition:state.spare_capacity
```

Callers are responsible for asserting applicable conditions. Atlas checks exact
identity and never infers a condition from prose, measurements or a similar ID.

## Algorithm costs

Each Algorithm has a non-empty `costs` list. Every profile is independently
sourced:

```yaml
costs:
- value:
    metric: time
    regime: worst
    bound: O(log n)
    requires: [state.spare_capacity]
  level: inferred
  source: docs:phase2/k-m4-dual-import-comparison.md
```

Supported metrics are `time`, `auxiliary_memory`, `retained_memory` and
`allocation`. Supported regimes are `worst`, `expected` and `amortized`.
`requires` is an exact set of condition references; an empty set means the
profile is unconditional.

Bounds remain opaque sourced strings. Schema 0.2 defines neither asymptotic
parsing nor ordering, units or cross-metric comparison. Consumers may require
an exact profile but must not infer that one textual bound dominates another.

For continuity with schema 0.1, every Algorithm must contain worst-time and
worst auxiliary-memory profiles. Multiple profiles with distinct bounds or
conditions are valid. Exact duplicate profiles and duplicate condition
references are rejected.

## Other entities

Problem and Implementation fields are unchanged. Executions remain an empty
collection because archived execution records are still outside the
authoritative registry.

## Migration and derived formats

The accepted migration converts every schema 0.1 `time_worst`, optional
`time_expected` and `auxiliary_memory` claim without losing its value, evidence
level or source. Reviewed heap and hash-map qualifications are added explicitly.

SQLite projection version 2 adds condition entities, indexed cost profiles and
qualified `costs[i].requires` relations. The private Web projection is
`atlas-web-private-v1`. Both remain disposable and reconstructible from this
document.

Unknown fields and unsupported schema versions are rejected. Any future field
removal, meaning change or incompatible type change requires another versioned
migration and an explicit structural decision.
