# DEC-003 - Rust workspace with Serde and YAML

- Status: accepted
- Date: 2026-07-11

## Context

The registry, CLI, and pilot implementations need one typed implementation
stack.

## Decision

Use a Rust workspace with Serde and `serde_yaml_ng` 0.10 as the YAML parser.
The crate is imported behind the local dependency name `serde_yaml` so this
secondary library remains replaceable. Defer SQLite until the source registry
and validation behavior are demonstrated.

## Consequences

Rust types are the validation model for schema 0.1. Cap'n Proto remains a
possible future universal serialization mechanism, but it is neither a source
format nor a dependency in MVP 1 without a separate class C decision.
