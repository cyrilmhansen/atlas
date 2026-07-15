# Phase 3 provisional exit audit

Status: provisional; one independent visitor trial remains blocking

Date: 2026-07-15

Authority: DEC-071, DEC-072 and `docs/phase3-explorer.md`

## Phase question

Can Atlas turn its qualified knowledge and bounded local executions into a
useful, inspectable static product without weakening provenance or presenting
execution and observation as theoretical evidence?

This audit maps the current evidence to every Phase 3 exit criterion. It does
not close the phase or replace the independent visitor gate.

## Evidence inventory

| Evidence | What it establishes | Limit |
|---|---|---|
| `docs/phase3/e-m1-review.md` | complete catalog coverage, exact relations, sourced claims and factual comparison | automated and maintainer-reviewed, not visitor comprehension |
| `docs/phase3/e-m2-protocol.md` | frozen tasks and rubric for executable and non-executable algorithms | protocol only until used independently |
| `docs/phase3/e-m2-owner-trial.md` | real backend and evidence-category confusion, followed by targeted corrections and owner confirmation | owner is informed and non-independent |
| `scripts/check-web.sh` | deterministic build, projection, WASM correction, focused DOM checks and browser gates | verifies behavior, not comprehension |
| Chromium viewport inspections | no incoherent overlap at the accepted desktop and mobile widths | technical inspection, not a broad usability study |
| DEC-072 and Pages run `29424594876` | verified static bundle is publicly reachable from `main` | does not stabilize private Web formats |

## Exit-criteria matrix

| Phase 3 exit criterion | Verdict | Evidence and remaining gap |
|---|---|---|
| Visitors find an entity, traverse its chain and locate provenance without YAML | **Partial, blocking** | E-M1 gates exact reachability and the owner completed the corrected path; one fresh independent visitor must attempt the frozen tasks. |
| Same-kind comparison is factual and exposes missing information | **Supported** | E-M1 review and projection tests cover sourced values, absent facts and the absence of rankings. |
| Executable and non-executable entities are clearly distinguished | **Supported** | Negative coverage is explicit, the action names the WASM model, implementation evidence remains separate and the corrected owner confirmation passes. |
| Local execution is correct, bounded and separate from claims and timing observations | **Supported** | Native/WASM/differential gates remain green; workbench labels distinguish registry claims, the interactive model and local observation. |
| Mobile and desktop workflows are usable and verified | **Supported technically** | Accepted viewport inspections and browser gates pass. Independent comprehension of the same paths remains folded into the first blocking criterion. |
| Bundle is reproducible, static and network-independent | **Supported** | The Web gate rebuilds deterministically and DEC-072 publishes only the verified static artifact. |
| Audit measures comprehension or task completion, not UI volume | **Partial, blocking** | The owner trial produced decision-relevant failures and corrections, but its informed confirmation cannot establish representative comprehension. |

## Provisional verdict

Phase 3 is **provisionally supported, not closed**. Its technical, provenance,
comparison, execution-boundary, responsive and distribution hypotheses have
direct evidence. The owner trial also falsified two initial presentation
assumptions, so the product trial has measured more than feature completion.

The remaining experiment is deliberately small: one technically literate
visitor with no prior Atlas context must use the unchanged E-M2 protocol on the
published artifact. A pass supports closure while retaining the stated
single-visitor limitation. A failure must be recorded before any correction and
may justify only the smallest task-blocking change.

## Freeze while evidence is pending

- do not add an algorithm, runtime path, dataset mode or backend;
- do not change the E-M2 tasks or rubric after seeing a response;
- do not add analytics, a feedback service or an in-product test harness;
- change Explorer only if an observed defect prevents completion of a frozen
  task.

Public task routes:

- executable case: `https://cyrilmhansen.github.io/atlas/?entity=algorithm%3Asequence.is_sorted.adjacent`;
- non-executable case: `https://cyrilmhansen.github.io/atlas/?entity=algorithm%3Agraph.traversal.breadth_first`.

## Post-phase recommendation

If the independent gate passes, close Phase 3 before activating more Explorer
work. The highest-information next program is the bounded comparative
foreign-selection experiment deferred by DEC-071: keep Explorer public, keep
Execution Lab frozen and test whether a newly imported competitor becomes a
qualified candidate without scenario-specific planner changes.

This is a recommendation, not Phase 4 activation. Its scope and acceptance
criteria require a separate human decision after the Phase 3 verdict.
