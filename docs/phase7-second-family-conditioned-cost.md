# Phase 7 - Second-family conditioned cost

Status: closed, recurring public-model boundary (2026-07-17)

## Question and experiment

Does the conditioned-cost limitation found for heap push recur in a structurally
different family?

The existing private projector discovers `associative_map.insert` candidates
through `solves` and `implements`, projects `time_expected`, and asks the
unchanged evaluator for expected `O(1)` insertion under a nonadversarial hash
distribution. The frozen imports already distinguish this assumption from
collision-heavy and adversarial behavior.

The falsifier is acceptance from authoritative public facts. Conservative
rejection is expected if schema 0.1 cannot attach the hash-distribution condition
to its declared `O(1)` expected claim.

## Result and verdict

The hashbrown implementation is discovered and its exact `O(1)` expected value
is projected. It is nevertheless rejected because the public claim carries no
structured condition matching the request.

The same modeling boundary now occurs in two independent families and across
two regimes: worst-case heap insertion under spare capacity, and expected hash
map insertion under a nonadversarial distribution. This is evidence for a
general conditioned-cost requirement, but not yet a schema proposal: the
experiment does not establish the smallest public representation or migration
cost. Phase 7 closes with that boundary confirmed.

Verification:

```text
cargo test -p atlas conditioned_expected_cost_boundary_recurs_for_hash_map_insert --locked --offline
```
