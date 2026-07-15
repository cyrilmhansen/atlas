# Phase 3 exit audit

Status: complete; Phase 3 closed supported under DEC-073

Date: 2026-07-15

Authority: DEC-071 through DEC-073 and `docs/phase3-explorer.md`

## Phase question

Can Atlas turn its qualified knowledge and bounded local executions into a
useful, inspectable static product without weakening provenance or presenting
execution and observation as theoretical evidence?

This audit maps the current evidence to every Phase 3 exit criterion and records
the human acceptance boundary used for closure.

## Evidence inventory

| Evidence | What it establishes | Limit |
|---|---|---|
| `docs/phase3/e-m1-review.md` | complete catalog coverage, exact relations, sourced claims and factual comparison | automated and maintainer-reviewed, not visitor comprehension |
| `docs/phase3/e-m2-protocol.md` | frozen tasks and rubric for executable and non-executable algorithms | used by the owner, not independently |
| `docs/phase3/e-m2-owner-trial.md` | real backend and evidence-category confusion, followed by targeted corrections and owner confirmation | owner is informed and non-independent |
| `scripts/check-web.sh` | deterministic build, projection, WASM correction, focused DOM checks and browser gates | verifies behavior, not comprehension |
| Chromium viewport inspections | no incoherent overlap at the accepted desktop and mobile widths | technical inspection, not a broad usability study |
| DEC-072 and Pages run `29424594876` | verified static bundle is publicly reachable from `main` | does not stabilize private Web formats |

## Exit-criteria matrix

| Phase 3 exit criterion | Verdict | Evidence and remaining gap |
|---|---|---|
| Visitors find an entity, traverse its chain and locate provenance without YAML | **Accepted with limitation** | E-M1 gates exact reachability and the owner completed the corrected path. DEC-073 accepts this informed evidence without claiming independent validation. |
| Same-kind comparison is factual and exposes missing information | **Supported** | E-M1 review and projection tests cover sourced values, absent facts and the absence of rankings. |
| Executable and non-executable entities are clearly distinguished | **Supported** | Negative coverage is explicit, the action names the WASM model, implementation evidence remains separate and the corrected owner confirmation passes. |
| Local execution is correct, bounded and separate from claims and timing observations | **Supported** | Native/WASM/differential gates remain green; workbench labels distinguish registry claims, the interactive model and local observation. |
| Mobile and desktop workflows are usable and verified | **Supported technically** | Accepted viewport inspections and browser gates pass. Independent usability remains unmeasured. |
| Bundle is reproducible, static and network-independent | **Supported** | The Web gate rebuilds deterministically and DEC-072 publishes only the verified static artifact. |
| Audit measures comprehension or task completion, not UI volume | **Accepted with limitation** | The owner trial produced decision-relevant failures and corrections. Its informed confirmation establishes owner task completion, not representative comprehension. |

## Verdict

Phase 3 is **supported and closed**. Its technical, provenance,
comparison, execution-boundary, responsive and distribution hypotheses have
direct evidence. The owner trial also falsified two initial presentation
assumptions, so the product trial has measured more than feature completion.

DEC-073 accepts the corrected owner confirmation as sufficient for this phase.
The result remains qualified: no independent or representative visitor evidence
was collected. Future visitor feedback may identify a concrete defect, but it
is no longer a Phase 3 closure gate.

## Closed boundary

- do not add an algorithm, runtime path, dataset mode or backend;
- do not add analytics, a feedback service or an in-product test harness;
- change Explorer only for a separately accepted phase or a blocking
  correctness defect.

Public task routes:

- executable case: `https://cyrilmhansen.github.io/atlas/?entity=algorithm%3Asequence.is_sorted.adjacent`;
- non-executable case: `https://cyrilmhansen.github.io/atlas/?entity=algorithm%3Agraph.traversal.breadth_first`.

## Post-phase recommendation

The highest-information next program is the bounded comparative
foreign-selection experiment deferred by DEC-071: keep Explorer public, keep
Execution Lab frozen and test whether a newly imported competitor becomes a
qualified candidate without scenario-specific planner changes.

This remains a recommendation, not Phase 4 activation. Its scope and acceptance
criteria require a separate human decision.
