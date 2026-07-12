# DEC-026 - Textual evidence references

## Status

Accepted on 2026-07-12 (`evidence-A`).

## Decision

Keep the schema 0.1 `source` field textual and define a deliberately restricted
MVP 1 grammar. A source consists of a recognized scheme, `:`, and one or more
non-empty targets separated by `;`. All targets in one source use the same
scheme.

The recognized schemes are `file`, `test`, `tests`, `implementation`,
`implementations`, `definition`, `vision`, `command`, `analysis`, and `docs`.
Local files, implementation IDs, and Rust test symbols are resolved. The other
schemes are documentary and receive syntax validation only.

File targets are workspace-relative and cannot be absolute or contain `..`.
Rust test targets use `module::test_name` and currently refer to tests in the
`atlas-algorithms` source modules.

## Consequences

The validator detects local evidence drift without network access or a generic
URI framework. Semicolons cannot occur inside a target in schema 0.1. Moving to
structured evidence later requires a deterministic source-field migration.
