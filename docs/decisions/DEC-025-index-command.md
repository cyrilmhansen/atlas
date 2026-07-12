# DEC-025 - Index command

- Status: accepted
- Class: B
- Date: 2026-07-12

## Decision

Use `atlas index [DB_PATH]`. The command validates the default authoritative
`registry/atlas.yaml`, rebuilds the derived database, and defaults to
`build/atlas.sqlite3` when no path is supplied.

## Consequences

The command prints entity, relation, and claim counts plus the logical digest.
Generated databases remain ignored by Git.
