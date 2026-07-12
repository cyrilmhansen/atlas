# MVP 2 review

Review date: 2026-07-12. Active scope: empirical qualification without making
generated executions part of the Git-authoritative registry.

## Demonstrated capabilities

- Deterministic sort and partition dataset matrices cover three sizes and four
  materially different input profiles each.
- Correction recipes regenerate YAML observations under ignored
  `build/executions/` for sorting and partitioning.
- A qualified sorting benchmark recipe records raw timing evidence, requested
  protocol, observed convergence, context, and diagnostics. It rejects quality
  warnings rather than writing a candidate observation.
- `atlas qualify` can answer the concrete query "stable in-place sort with no
  allocation" while preserving the evidence level and source of every matched
  property.

## Exit criteria

| Criterion | Status | Evidence or gap |
|---|---|---|
| Re-run an execution from its identifier | Complete while the local generated observation exists | `atlas replay EXECUTION_ID` validates and dispatches an allow-listed recipe; a repeated benchmark may legitimately fail its new quality gate. Deleted observations remain intentionally unreplayable by ID. |
| Distinguish theory, declarations, tests, and observations | Partial | Registry claims preserve levels; generated observations are clearly separate, but no common query spans both. |
| Query a stable sort under a memory limit | Partial | `qualify` supports stability, in-place, and `allocation: none`; it has no numeric memory-limit predicate. |
| Signal aberrant or non-comparable measurements | Complete for the sorting harness | The quality gate rejects dispersion, drift, outliers, and position bias before serialization. |
| Capture machine, system, compiler, options, commit, and seed | Complete for the sorting harness | Generated benchmark observations contain these values and diagnostics. |
| Measure time, peak memory, allocations, and traversed volume | Partial | Time and process resident/peak memory are captured; allocation count and traversed volume are explicitly unavailable. |
| Compare implementations and report observed domains | Deferred | The harness can compare in memory, but persistent cross-process comparison and a non-generalizing report are intentionally absent. |

## Local gate

`scripts/check-mvp2.sh` runs the MVP 1 gate, generates the dataset and two
correction observations, and verifies the constrained qualification query. It
does not run a benchmark because timing is an explicit human experiment with a
selected CPU and a qualified environment.

## Recommended closure order

1. Decide whether numeric memory and allocation measurements are necessary for
   MVP 2 exit, or whether explicit allocation classes are sufficient.
2. If time comparison remains in scope, define how a multi-implementation
   generated campaign is represented without archiving measurements in Git.
3. Add a bounded report that names dataset, environment, qualification verdict,
   and limits without implying a universal ranking.
4. Reassess the exit criteria, then ask for an explicit MVP 2 closure decision.
