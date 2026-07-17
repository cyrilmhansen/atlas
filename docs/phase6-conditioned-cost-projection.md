# Phase 6 - Conditioned cost projection

Status: closed, informative unsupported result (2026-07-17)

## Question

Can schema 0.1 select an implementation of `priority_queue.push` whose worst
time is `O(log n)` when the caller guarantees spare capacity?

## Experiment

The private test projector discovers candidates only through `solves` and
`implements`. It projects each algorithm's authoritative `time_worst` claim,
then submits the same request to the unchanged K-M5 evaluator: exact push
contract, spare-capacity condition and worst time `O(log n)` under that
condition.

Expected falsifier: a candidate is accepted from public facts alone. Expected
boundary: both candidates are rejected because schema 0.1 records only their
unconditioned one-call worst time and cannot attach a condition to a cost.

## Result and verdict

Both registered implementations are discovered. Both expose
`O(n) for one call when capacity growth reallocates`; neither produces the
exact conditioned profile, so both are rejected as unsupported.

Atlas therefore preserves the conservative decision boundary, but schema 0.1
cannot project the useful `O(log n)` spare-capacity fact already exercised in
the private Phase 4 overlay. This single-family result does not justify a
public schema or evaluator extension. Phase 6 closes as informative and
unsupported for conditioned-cost selection.

Verification:

```text
cargo test -p atlas conditioned_cost_projection_reports_public_schema_boundary --locked --offline
```
