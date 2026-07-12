# DEC-013 - Expected time cost

- Status: accepted, implemented
- Class: C
- Date: 2026-07-11

## Context

Hash-based algorithms have an expected running-time claim that differs from
their worst-case claim. Schema 0.1 currently records only `time_worst`.

## Decision

Add an optional qualified `time_expected` algorithm claim when the hash-based
deduplication slice introduces the first real need.

## Consequences

Worst-case and expected complexity will remain distinct claims with provenance.
No expected cost may be silently represented as a worst-case guarantee.
