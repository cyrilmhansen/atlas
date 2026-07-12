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

The first benchmark adapter advances the generated format to
`atlas-execution.experimental.0.5`. It writes raw samples as exact decimal
strings because the YAML parser cannot round-trip `u128` numeric scalars,
alongside requested protocol,
observed convergence, summary, context, process resident and peak memory, and
before/after diagnostics only when the existing quality gate accepts the
measured series. Allocation count and traversed-volume metrics are explicitly
`unavailable` until measured rather than inferred.

`atlas replay EXECUTION_ID` searches only the local generated directory,
validates the content-derived identity, and dispatches an allow-listed versioned
recipe. Benchmark replay additionally requires an explicit `--cpu N`.

## Consequences

The experimental YAML representation is an internal generated format, not a
public registry schema or a second source of truth. A first correction recipe
must demonstrate deletion and deterministic regeneration before benchmark
observations use the same boundary. Environment changes legitimately produce a
different execution identity.

A declarative campaign format remains deferred until at least two materially
different versioned recipes expose requirements that code and scripts do not
represent adequately.
