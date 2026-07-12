# DEC-033 - Regenerable execution observations

## Status

Accepted on 2026-07-12 (`recipe-B`).

## Decision

Execution observations are generated products, not part of the Git-authoritative
knowledge registry. Schema 0.1 keeps `executions: []`. Experimental execution
files are written under ignored `build/executions/` and may be deleted at any
time.

The recipes needed to reproduce them remain versioned as Rust code, dataset
specifications, and scripts. Each generated observation records its recipe,
implementation and dataset identities, dataset content digest, parameters,
commit, compiler, target, result, provenance, and a content-derived identity.
Correction outputs use an ordered name/value map because sorting and
partitioning expose materially different results. The generated format advances
to `atlas-execution.experimental.0.2`; old generated files are regenerated, not
migrated.

## Consequences

The experimental YAML representation is an internal generated format, not a
public registry schema or a second source of truth. A first correction recipe
must demonstrate deletion and deterministic regeneration before benchmark
observations use the same boundary. Environment changes legitimately produce a
different execution identity.

A declarative campaign format remains deferred until at least two materially
different versioned recipes expose requirements that code and scripts do not
represent adequately.
