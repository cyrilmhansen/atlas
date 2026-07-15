# E-M2 evidence-centered handoff protocol

Status: implementation candidate confirmed by the owner; independent visitor
run pending on 2026-07-15

Authority: DEC-071 and `docs/phase3-explorer.md`

## Question

Can a technically literate visitor distinguish registry knowledge, reviewed
local execution and ephemeral browser observation while moving between an exact
algorithm identity and the workbench?

This protocol evaluates comprehension. It does not evaluate visual appeal,
algorithm pedagogy or runtime performance.

## Frozen subjects

1. `order.is_sorted.adjacent`: schema 0.1 algorithm with a reviewed generated
   visual program and local WASM execution.
2. `graph.bfs.traversal`: foreign-corpus algorithm with sourced claims and a
   tested implementation, but no reviewed browser execution.

No new executable subject may be added during this experiment.

## Visitor tasks

For adjacent sortedness:

1. find the exact algorithm from the catalog;
2. report its worst-time claim, evidence level and provenance;
3. determine whether Atlas offers a reviewed local execution;
4. execute one DatasetSpec case and identify which displayed facts are local
   observations rather than theoretical claims;
5. return to the exact registry entity without searching again.

For BFS traversal:

1. find the exact algorithm and its implementation through the knowledge chain;
2. report its worst-time claim, evidence level and provenance;
3. determine whether Atlas offers a reviewed local execution;
4. explain whether absence of that execution changes the recorded theoretical
   claim.

## Success rubric

| Dimension | Pass condition |
|---|---|
| Identity | Exact algorithm and implementation IDs are retained |
| Evidence | Value, level and provenance are reported together |
| Availability | Available and unavailable execution states are both explicit |
| Boundary | Dataset/result/counters/timing are identified as local observation |
| Return path | Workbench returns to the exact originating algorithm |
| Restraint | Visitor does not infer that missing execution falsifies a claim |

A representative visitor run remains required before E-M2 closure. Automated
tests may protect interface facts but cannot satisfy this comprehension gate.

## Internal baseline

The pre-change interface audit is deliberately weaker than a visitor result:

- adjacent sortedness exposes sourced complexity and labels its result panel as
  an observation; DatasetSpec identity and local timing qualification are
  visible;
- the catalog-to-workbench handoff preserves the selected identity internally,
  but the workbench has no explicit command back to its exact registry fiche;
- BFS has no execution button, but this absence is silent and can be mistaken
  for a loading or permissions state;
- no evidence suggested that another execution backend, dataset or algorithm is
  needed.

The only authorized corrections for this checkpoint are therefore an explicit
availability state and an exact workbench-to-registry command. Theoretical
claims, DatasetSpec presentation and local observation remain otherwise
unchanged.

## Owner trial 1 correction checkpoint

`docs/phase3/e-m2-owner-trial.md` records the first human response. It exposed
two additional boundary failures:

- the interactive WASM model was confused with the registered Rust
  implementation;
- behavioral implementation tests were confused with, or expected to decide,
  the algorithm complexity claim.

The corrected interface names the WASM model explicitly, keeps implementation
evidence separate, and renders the complete evidence level and provenance for
workbench complexity claims. The owner confirmation passes all three corrected
dimensions. An independent confirmation remains required because the owner
trial is informed rather than blind.
