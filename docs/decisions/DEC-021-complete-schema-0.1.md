# DEC-021 - Complete implementation metadata before schema 0.1 freeze

- Status: accepted
- Class: C
- Date: 2026-07-11

## Decision

Before schema 0.1 is declared stable, every implementation records qualified
version, license, target, dependency, and ABI claims. Schema versioning rules
must distinguish additive compatible changes, breaking changes, and migrations.

## Consequences

The current unreleased schema remains 0.1 while these fields are added. All
twenty corpus implementations must be migrated together. A successful parse is
not sufficient for freeze; coverage and compatibility rules must also pass.
