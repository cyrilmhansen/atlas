# DEC-058 - Extend MVP 5 with bounded semantic dynamics

## Status

Accepted on 2026-07-14 (`scope-A data-A dynamics-B`).

## Decision

Keep MVP 5 open and add a bounded semantic-dynamics stage before formal
closure. The interactive artifact will make working sequences visibly editable
and add deterministic local generation by profile, size and seed. Generated
inputs are ephemeral observations, not DatasetSpec or registry evidence.

Use two explicit regimes:

- Explore inputs contain at most 64 elements and may produce bounded semantic
  traces, pseudocode highlighting and step/run/reset controls.
- Scale inputs may contain up to the existing 4096-element Web limit and are
  intended for deterministic operation-growth views, not element animation.

The first dynamics adapter targets adjacent `is_sorted`. Every event must carry
an exact node ID from `ast.order.is_sorted.adjacent.v0` and a concordant
operation kind. The displayed pseudocode comes from the existing private
textual source whose parser equivalence is already tested. Native Rust remains
the correction oracle.

## Consequences

- The previous reproducible bundle audit remains a valid checkpoint, but no
  longer represents MVP 5 closure readiness.
- Larger interactive runs may illustrate counted growth under a selected data
  profile; they do not infer or prove asymptotic complexity.
- Reverse and insertion dynamics require separately tested AST/trace adapters.
  Their current aggregate counters must not be presented as execution traces.
- No public trace schema, stable Web API, general state-machine renderer or MIR
  browser runtime is introduced.

DEC-059 refines the insertion-sort implementation of this decision: its
interactive presentation uses a stateful WASM stepper bounded to 64 elements,
while its separately bounded 32-element trace remains a validation/analysis
oracle only.
