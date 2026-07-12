# DEC-002 - Aggregate YAML schema 0.1

- Status: accepted
- Date: 2026-07-11

## Context

The registry needs a textual, versioned, diffable source format whose complete
integrity can be checked on demand.

## Options considered

- Separate YAML files per entity.
- One aggregate YAML document containing the entity collections.
- Separate JSON files per entity.

## Decision

Use one aggregate YAML document and validate it wholly in memory. Problems,
algorithms, implementations, and executions remain distinct collections linked
by IDs.

## Consequences

The format is simple to load and validate for the MVP 1 corpus. Memory usage and
diff contention are accepted for now. Reconsider sharding when the aggregate
file becomes materially difficult to load, review, or merge; changing the
persistent format will require a new class C decision and migration plan.
