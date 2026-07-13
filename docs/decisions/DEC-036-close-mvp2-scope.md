# DEC-036 - Close MVP 2 empirical qualification scope

## Status

Accepted on 2026-07-13 (`mvp2-close`).

## Decision

Close MVP 2 with the demonstrated deterministic datasets, regenerable
correction and benchmark observations, replay, bounded qualification, process
memory diagnostics, and a single-pass procedure for a locally generated
comparison report.

MVP 2 does not require a retained clean comparison report. Such a report can be
captured later using the documented procedure, but remains a non-normative,
generated local observation and is not a closure condition.

MVP 2 also does not include an algorithm-only allocation counter, traversed
volume measurement, numeric memory-limit query, or a query that joins generated
observations with registry claims. These absences are explicit limits, not
negative results or inferred measurements.

## Consequences

- The process resident and peak-resident values remain diagnostics for the
  complete benchmark process; they must not be described as exact algorithm
  memory use.
- `allocation: none` remains a declared implementation property used by the
  bounded `qualify` command. It is not replaced by an empirical allocation
  claim.
- Future work needing numerical resource constraints must first define the
  measurement boundary, provenance, and interpretation. It may not silently
  promote current process metrics into algorithm properties.
- Joining registry knowledge with generated observations, comparison across
  broader domains, and composition selection remain future MVP work. MVP 3 is
  not activated by this decision.
