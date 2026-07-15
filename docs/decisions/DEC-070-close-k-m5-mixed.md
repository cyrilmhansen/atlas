# DEC-070 - Retain the private condition-aware trial and close K-M5 mixed

## Status

Accepted on 2026-07-15 (`close-km5-A`).

## Context

K-M5 demonstrates generic decisions over a private overlay, independent
authoring, bounded encoding equivalences and condition-aware heap allocation.
The condition-aware follow-up crosses DEC-069's initial unconditional-cost
boundary by 23 non-test Rust lines, but preserves all invalid-boundary
rejections.

The experiment also falsifies its stronger product hypothesis: schema 0.1 and
the existing CLI cannot author or consume the facts required for those generic
decisions. Continuing with more private families would increase infrastructure
without testing a new central hypothesis.

## Decision

Retain the condition-aware equivalence trial as private falsification evidence
and close K-M5 with verdict **mixed**.

- Freeze the overlay, equivalence resolver and K-M5 fixtures at their current
  bounded capability; no third-family extension is active.
- Keep schema 0.1, `registry/atlas.yaml`, SQLite projection and CLI behavior
  unchanged.
- Do not expose or promote the overlay as a stable format, public API or agent
  interface.
- Record 1,169 non-test Rust lines as the final K-M5 private implementation cost.
- Make K-M6 the next planned milestone, using the accepted existing CLI and
  textual output while withholding implementation source during selection.

Any later public promotion of overlay concepts requires a separate class C
schema decision, migration plan and evidence beyond K-M5.

## Consequences

- The positive generic-evaluator and two-family equivalence results remain
  reproducible.
- The inability of current manifests and CLI to use those facts remains an
  explicit K-M5 failure, not deferred polish.
- Atlas Knowledge remains active; Execution Lab and Explorer remain frozen or
  maintained as already decided.
- K-M6 must not use the private overlay to improve the assisted-agent outcome.

## Alternatives considered

- Revert condition transport and close mixed: rejected because the second-family
  result is sound, bounded and inexpensive relative to the retained experiment.
- Continue K-M5 into a third family: rejected for low expected information gain.
- Promote a schema 0.2 subset now: rejected as premature and outside K-M5.
