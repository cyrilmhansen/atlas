# B2 unchanged-evaluator transfer result

Status: mixed; Option A executed and stopped without evaluator changes

Date: 2026-07-15

Authority: `b2-A`, DEC-074 and `docs/phase4/k4-b2-options.md`

## Result

The separate `docs/phase4/k4-b2-graph-overlay.yaml` loads and executes through
the unchanged K-M5 parser, validator, source resolver and evaluator. It contains
five atoms, two candidates and the four frozen graph requests in 97 YAML lines.
A 43-line focused test supplies exact accepted sets and rejection reasons. No
non-test Rust or public surface changed.

## Reproduced decisions

| Frozen request | Evaluator result | Agreement with K4-M1 |
|---|---|---|
| exact reachable component | BFS and DFS accepted | complete |
| no traversal allocation | no candidate; forbidden-allocation reason for both | complete |
| frontier bound for known graph shape | no candidate; missing exact conditioned cost for both | complete at the missing-fact level |
| nondecreasing hop order | no candidate; missing guarantee for both | accepted set agrees, reasons lose a decision-relevant distinction |

The experiment demonstrates that the K-M5 vocabulary transfers across
families for positive guarantees, forbidden effects and conditioned costs.

## Falsified assumptions

### Missing and contradicted guarantees collapse

The authoritative graph result distinguishes:

- BFS: no explicit nondecreasing-hop guarantee is registered;
- DFS: a tested counterexample refutes the universal guarantee.

K-M5 represents supplied guarantees but not explicit counterevidence or a
refuted guarantee. Both candidates therefore receive the same `missing
guarantee` reason. Encoding the DFS counterexample as an `effect` would be a
semantic category error and was rejected.

### Candidate discovery remains two-stage

K4-M1 independently proves relation-driven discovery from `solves` and
`implements`. The K-M5 evaluator does not consume that discovery result: its
phase-local YAML manually declares the same two candidates and their facts.
Source resolution proves that their registry entities exist, not that overlay
facts were projected from authoritative claim paths.

The evaluator is data-driven within its overlay, but this experiment does not
establish manifest-driven qualified selection end to end.

## Cost

| Increment | Size |
|---|---:|
| Phase 4 overlay | 97 YAML lines |
| Focused test | 43 Rust lines |
| Non-test Rust | 0 |
| Dependencies | 0 |
| Schema/CLI changes | 0 |

The previously measured 1,169 K-M5 non-test lines remain sunk experimental
infrastructure and are not counted as new Phase 4 implementation.

## Stop-condition outcome

Option A required no evaluator code or family-specific branch, so its minimum
transfer experiment passes. It does not reproduce every reason or automatic
registry-to-evaluator projection, so it cannot be promoted or described as the
Phase 4 selection engine.

## Follow-up options

### Option A - Retain it as a partial control and proceed to K4-M2

Keep the overlay and test as evidence. Use the frozen documentary outcome
classes alongside it, then ask whether the same two limitations recur for
priority queues.

Cost: no new infrastructure now. Risk: executable and reviewed adjudications
remain separate. Reversibility: complete.

### Option B - Build a Phase 4 evaluator now

Add explicit accepted/rejected/unsupported classes, negative guarantees and
registry-derived candidate projection before the second family.

Cost: new model, projection and evaluator code. Risk: designing from one
foreign competition. Reversibility: high while private, but infrastructure
ratio rises immediately.

### Option C - Extend K-M5

Add counterevidence and registry projection to the old overlay model.

Cost: changes a deliberately frozen experiment and expands its 1,169-line
surface. Risk: turns retained Phase 2 evidence into a de facto internal
platform. Reversibility: lower because old experiment semantics change.

## Recommendation

Recommend **Option A (`b2-follow-A`)**. A second structurally different family
is the minimum evidence needed before adding explicit negative facts or a
registry-to-qualification projection. Preserve the mixed graph result and move
to K4-M2 source review without pretending B2-A supplied end-to-end selection.
