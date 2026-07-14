# Phase 2 import-equivalence rubric

Status: K-M0 pilot rubric  
Protocol revision: `k-m0.1`

This rubric compares two independent worksheets after both are complete. It
does not reward textual identity. The comparison asks whether remaining
differences alter a knowledge decision.

## Comparison unit

Compare one source subject at a time, then compare cross-source normalization
for the same named algorithm. Preserve raw worksheets. A reviewer creates one
matrix without silently rewriting either proposal.

## Outcome labels

Use one label per dimension:

- `equivalent`: differences cannot change the assessed decision;
- `compatible`: one account is more specific, but both can coexist without a
  contradictory decision;
- `divergent`: the accounts can produce different decisions;
- `unresolved`: the source or current model cannot decide;
- `not-applicable`: explain why the dimension does not apply.

Do not convert these labels into a composite score during the pilot.

## Dimensions

### Identity equivalence

Compare problem boundary, algorithm boundary and implementation identity.

Questions:

- Do both imports recognize the same problem being solved?
- Do they distinguish the same strategy from its API/output variants?
- Do they separate source implementation from independently rewritten code?
- Would either proposal merge candidates that the other keeps distinct?

A disagreement is substantive when it changes lookup, substitution or evidence
attachment.

### Semantic equivalence

Compare inputs, preconditions, outputs, guarantees, invariants, failure regimes,
determinism and numeric assumptions.

A narrower valid domain may be `compatible`; omission of a condition that can
admit an incorrect execution is `divergent`.

### Taxonomic equivalence

Compare vocabulary, categories, variant boundaries and relationships.

Synonyms are documentary when normalization retains both source terms. They are
substantive when a category controls candidate discovery or collapses distinct
complexity/effect regimes.

### Operational equivalence

Evaluate concrete, written scenarios rather than field similarity:

1. Would both imports return the same candidate set for the same constraints?
2. Would both accept or reject the same substitution?
3. Would both accept or reject the same composition boundary?
4. Would both explanations cite compatible reasons and evidence?

This is the primary arbitration dimension. A semantic or taxonomic divergence
that cannot affect any current or proposed decision is recorded but does not
become an artificial operational failure.

### Documentary divergence

Wording, examples, locator granularity and explanatory decomposition may differ.
Such differences are acceptable when source fidelity remains auditable and no
identity, semantic, taxonomic or operational result changes.

## Review matrix

| Subject | Dimension | Import A | Import B | Outcome | Decision changed | Cause | Follow-up |
|---|---|---|---|---|---|---|---|
| | identity / semantic / taxonomic / operational / documentary | | | | lookup / selection / substitution / composition / none | source ambiguity / protocol ambiguity / model insufficiency / importer error | |

Every `divergent` or `unresolved` outcome needs one minimal discriminating case.
Examples include a graph with an unreachable vertex, an early Dijkstra goal, or
a union-find query whose compression mutates internal state.

## Cross-import adjudication

1. Verify both imports used the same source packet.
2. Separate source facts from inferred general knowledge.
3. Run the smallest current CLI or manual contract scenario that exposes the
   difference.
4. Classify the cause before changing either proposal.
5. Preserve both original accounts and record any adjudicated normalization.

Human adjudication is required for a decision-changing divergence. Agreement
produced only after one importer sees the other is not independent agreement.

## Escalation thresholds

- Fix a worksheet or protocol typo immediately when it cannot change the model.
- Revise the protocol once after the BFS/Dijkstra/union-find pilot, then freeze
  it for the first corpus batches.
- Record family-specific gaps as experimental annotations.
- Propose a public schema extension only when the same lossy mapping occurs in
  at least two structurally independent families.
- Pause corpus growth when unresolved differences repeatedly change selection,
  substitution or composition outcomes.

## Pilot acceptance

The pilot succeeds when it yields:

- six independent subject worksheets: three subjects by two importers;
- a completed five-dimension matrix;
- authoring effort at worksheet granularity;
- at least one minimal operational discriminant for every substantive
  divergence;
- explicit separation of source ambiguity, protocol ambiguity, model
  insufficiency and importer error;
- no public schema or AST change made to improve apparent agreement.
