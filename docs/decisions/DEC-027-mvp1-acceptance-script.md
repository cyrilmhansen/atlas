# DEC-027 - MVP 1 acceptance script

## Status

Accepted provisionally on 2026-07-12 as a reversible class B decision.

## Decision

Use `scripts/check-mvp1.sh` as the single local MVP 1 acceptance command. The
script is a small POSIX shell orchestrator over existing Cargo and Atlas
commands; it does not duplicate validation logic.

The gate checks formatting, the core/alloc/hash algorithm profiles, workspace
tests, Clippy, registry validation, and two identical logical SQLite index
rebuilds. Commands are locked and offline so acceptance cannot silently change
the dependency graph or require network access.

## Consequences

CI can call the same script once a CI platform is selected. The script relies on
an already provisioned Rust toolchain and system SQLite development metadata.
Replacing it with a task runner remains straightforward if a second substantial
automation workflow later justifies that dependency.
