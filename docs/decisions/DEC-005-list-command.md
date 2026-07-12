# DEC-005 - List command

- Status: accepted
- Class: B
- Date: 2026-07-11

## Context

The first registry inspection command should expose the separate entity kinds
without adding a query language.

## Decision

Use `atlas list [problem|algorithm|implementation]`. With no kind, the command
lists all three kinds. Each output line is `kind<TAB>id`, in manifest order.

Execution records are omitted until schema 0.1 defines a real execution shape.
The command reads the default `registry/atlas.yaml`; selecting another registry
can be added when a concrete use requires it.
