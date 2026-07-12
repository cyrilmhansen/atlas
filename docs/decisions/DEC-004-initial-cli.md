# DEC-004 - Initial validation command

- Status: accepted
- Class: B
- Date: 2026-07-11

## Decision

The initial command is `atlas validate [PATH]`. Without `PATH`, it validates
`registry/atlas.yaml` relative to the current directory.

## Consequences

This intentionally narrow interface can be extended as the first real uses of
`list`, `show`, `search`, and `explain` are implemented.
