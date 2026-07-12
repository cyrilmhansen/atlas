# DEC-035 - Generated comparison reports

## Status

Accepted on 2026-07-12 (`campaign-A`).

## Decision

Provide `atlas compare EXECUTION_ID EXECUTION_ID...` for two or more locally
retained, qualified benchmark observations. It writes an ignored
`atlas-comparison.experimental.0.2` YAML report
under `build/reports/` with source execution IDs, common dataset, context,
requested protocol, sorted observed medians, process peak memory, and a
context-bounded conclusion.

The command rejects dirty worktrees, different datasets, parameters, environments, contexts, requested protocols,
duplicate executions, duplicate implementations, correction observations, and
unqualified benchmark observations.

## Consequences

Reports are generated products, not registry entities or public formats. A
lowest observed median is stated only for the exact dataset, context, and
requested protocol; it is never a universal algorithm ranking.
