# DEC-023 - Normalized SQLite projection

- Status: accepted
- Class: C
- Date: 2026-07-12

## Decision

Projection version 1 uses four tables: `projection_meta`, `entities`,
`relations`, and `claims`. Claims retain entity ID, path, canonical typed value,
evidence level, provenance source, and ordinal. Relations retain source,
relation name, target, and ordinal.

## Consequences

The projection is generic over schema claims but remains limited to the current
entity model. It is rebuilt transactionally from validated YAML and never edited
as a source. Projection-version changes do not change source schema versions.
