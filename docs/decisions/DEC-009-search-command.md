# DEC-009 - Text search command

- Status: accepted
- Class: B
- Date: 2026-07-11

## Context

MVP 1 needs a small discovery command before introducing structured constraint
queries or algorithm selection.

## Options considered

- Search identifiers only.
- Search identifiers and declared names, ignoring case.
- Introduce structured `field=value` filters immediately.

## Decision

Use `atlas search <term>`. Match the term case-insensitively against every entity
ID and against algorithm `name.value` claims. Emit matching entities as
`kind<TAB>id`, in the same order as `list`.

An empty term is an invocation error because `atlas list` already lists all
entities. No matches is a successful result with empty standard output.

## Consequences

This is text discovery, not constraint-based selection. Searchable fields can be
extended when real corpus usage justifies them; the output remains shared with
`list`.
