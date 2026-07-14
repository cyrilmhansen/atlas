# Phase 2 bounded ontology review

Status: decisions accepted under DEC-067 and DEC-068

Scope: Atlas Knowledge only

Public schema change: none in this review

## Question

What is the smallest model change that lets Atlas make source-independent
selection, substitution and composition decisions on the imported corpus,
without turning schema 0.1 into a universal ontology or silently changing the
meaning of existing claims?

This review is triggered by the accepted K-M4 pause condition. It synthesizes
observed failures; it does not infer requirements from an imagined future
framework.

## Evidence base

| Decision Atlas cannot currently make generically | Independent cases | Consequence |
|---|---|---|
| Richer output satisfies a narrower request through a projection | Dijkstra path tree to distances; Misra-Gries candidate stage to verified result has the inverse boundary | valid substitution rejected, or inexact output accepted |
| One strategy or state transformation relates distinct contracts | BFS traversal/path materialization; incremental/pairwise moments; Bloom insert/query | duplicated identities or hidden composition |
| State produced by one operation is valid input to another | union-find operations; heap/map mutation; Bloom construction/query; online moments | composition cannot prove state continuity |
| A cost claim depends on operation, sequence, capacity or workload | union-find amortization; heap resize/order; hash collision distribution; Bloom positive/negative queries | incomparable or wrongly selected costs |
| Determinism depends on an explicit source of variation | graph tie order; reservoir random source/seed; Bloom hash model; IEEE-754 evaluation order | replay and substitution decisions are unsafe |
| Output correctness is qualified rather than simply exact | Dijkstra projection; top-k exactness; Bloom one-sided error; Misra-Gries candidate superset | exact and approximate candidates collapse |
| Numeric or probabilistic guarantees have a validity domain | graph weights; Welford/Pébay finalizer cardinality; Bloom error assumptions; reservoir uniformity | invalid candidates cannot be rejected |
| Evidence strength has no operational promotion rule | K-M4-W published proposition; tested adapters versus source declarations | `proven` cannot safely drive selection |

Every proposed public concept must cite at least two rows from structurally
different families. AST coverage, MIR lowering and Explorer presentation are
not ontology requirements.

## Three layers

### Import protocol

Some divergences do not require a schema field:

- normalize a generic `Problem` around observable requirements and outputs;
  keep representation and orientation in `Algorithm` or implementation
  requirements unless they change the requested result;
- retain the narrow source contract; record totalization, specialization and
  pedagogical repair as transformations rather than source guarantees;
- contextual literature may define vocabulary but does not prove a claim about
  a versioned implementation;
- an inaccessible historical source may remain bibliographic while a readable
  later source supports a neutral algorithm identity.

These rules belong in protocol `k-m0.3` after the representation strategy is
selected. They should reduce importer variance without changing stored facts.

### Knowledge semantics

The recurring decision-relevant needs are:

1. **Contract relations** between exact source boundaries: projection,
   specialization, guarded finalization and multi-stage refinement. Direction
   and conditions must be explicit; a generic undirected `related_to` has no
   selection value.
2. **Qualified claims** whose scope identifies the operation and cost/guarantee
   regime, while conditions preserve capacity, workload, arithmetic, random
   source or parameter assumptions.
3. **State flow** identifying the persistent state consumed and produced by an
   operation separately from scratch, allocation and result values.
4. **Output qualification** distinguishing exact, sound-superset,
   possibly-positive/definitively-negative and distributional results.
5. **Evidence semantics** defining when `declared`, `inferred`, `tested`,
   `observed` and `proven` can be used for a selection threshold.

This is a requirements list, not an accepted field list. A representation that
cannot explain a minimal discriminant below is insufficient; one that models
unobserved concepts is too broad for this phase.

### Selection engine

K-M5 must consume structured facts without an implementation ID or
source-family branch. It should not parse prose into semantics at query time.
The engine needs only conjunction over explicit facts and directional contract
relations for the first experiment. Ranking, theorem proving, planning and a
general expression language remain excluded.

## Minimal discriminants

The next representation experiment must answer these cases without custom code
per candidate:

1. accept a richer Dijkstra path-tree result for an all-distances request only
   through an explicit sound projection;
2. reject a Misra-Gries candidate-only stage for exact heavy hitters, while
   allowing it when a bounded superset is requested;
3. accept heap push as allocation-free only under a supplied spare-capacity
   condition and reject its individual O(log n) guarantee including resize;
4. connect a compatible constructed state to a later union-find or Bloom
   operation and reject a mismatched state family;
5. reject Bloom membership for exact authorization while accepting its
   definitive negative result under a declared error model;
6. reject total unbiased variance for `n < 2` and reject IEEE-754 bitwise
   order-independence for pairwise moments;
7. distinguish a source-declared theorem from an Atlas claim accepted at level
   `proven`.

Use the unchanged schema 0.1 behavior as the control. The experiment is useful
only if it records both the new correct decisions and its additional authoring
cost.

## Decision 1: representation strategy

### Option A - Experimental decision overlay

Keep schema 0.1 and the aggregate registry authoritative. Add a small,
explicitly disposable Phase 2 overlay containing only facts needed by the seven
discriminants. Run K-M5 first against schema 0.1 as a control and then against
the overlay. Promote nothing until the comparison shows that the structure
changes correct decisions at acceptable authoring cost.

- Cost: one experimental format, validator and narrow query path.
- Risks: temporary duplication and pressure for the overlay to become a hidden
  permanent schema.
- Reversibility: high; delete or regenerate it without migrating the registry.
- Minimum experiment: encode at most eight existing candidates spanning graph,
  dynamic-state, approximate streaming and numerical aggregation; run all seven
  discriminants.

### Option B - Direct public schema 0.2

Design public relation, qualified-claim and state-contract fields now, migrate
the aggregate YAML and make K-M5 use them.

- Cost: schema design, deterministic migration, validator/index/query changes
  and documentation across the full registry.
- Risks: stabilizing abstractions derived from a still-small corpus; schema 0.2
  may merely encode current worksheets.
- Reversibility: low after external use.
- Minimum experiment: a complete migration branch plus backward/forward
  rejection tests and the seven discriminants.

### Option C - Keep schema 0.1 throughout Phase 2

Run K-M5 with exact identities and prose only. Record generic discovery as
falsified wherever structured semantics are absent; defer all model work to the
phase synthesis.

- Cost: minimal implementation work.
- Risks: K-M5 repeats already-known limitations and cannot compare a candidate
  remedy against the baseline.
- Reversibility: complete.
- Minimum experiment: execute the seven discriminants manually and record the
  unsupported outcomes.

### Recommendation

Accepted: **Option A** (`ontology-A`). It is the only option that measures both target value
(changed decisions) and meta cost (facts and code required) before a public
commitment. Its hard boundary is that the overlay is neither registry authority
nor a compatibility promise. If the experiment needs candidate-specific fields
or more than a narrow conjunction/relationship evaluator, stop and report the
model as unsupported rather than generalizing it.

## Decision 2: meaning of `proven`

This decision is independent of the representation choice.

### Option A - Auditable proof artifact

Reserve `proven` for a claim mapped to an inspectable proof artifact and an
explicit Atlas review or verifier result. A paper containing a proof supports
`declared` claim “the source proves X” until that mapping is imported and
checked.

- Cost: published proofs remain conservatively `declared` during Phase 2.
- Risk: under-crediting sound external proofs.
- Reversibility: the claim can be promoted later without changing its value.

### Option B - Source-asserted proof

Allow `proven` when a citable primary source presents a proof of the exact
normalized claim, after human review of source fidelity.

- Cost: a review checklist and exact locator are required.
- Risk: `proven` conflates “proof published” with “proof checked by Atlas” and
  can vary between importers.
- Reversibility: demotion changes trust behavior for consumers.

### Option C - Suspend `proven`

Reject `proven` in new Phase 2 imports until a future proof subsystem defines
it. Existing schema syntax remains but the phase protocol forbids new uses.

- Cost: loses a useful distinction even for future auditable artifacts.
- Risk: postpones rather than resolves the semantics.
- Reversibility: high at protocol level.

### Recommendation

Accepted: **Option A** (`proven-A`). It keeps `proven` operationally stronger than
`declared`, preserves reversible promotion and avoids making peer review a
machine-checkable property it is not. The proof artifact need not be generated
by a proof assistant, but its exact claim mapping and review method must be
inspectable.

## Acceptance for the next slice

The accepted next slice is not a schema migration. It should:

- record the accepted decisions;
- revise only the import protocol rules that are independent of representation;
- freeze the schema 0.1 control cases and expected failures;
- specify the bounded overlay and its deletion/promotion criteria if Option A is
  accepted;
- avoid registry migration until K-M5 evidence is available.

Open questions intentionally deferred: universal type expressions, units,
symbolic algebra, automated source extraction, proof checking, query ranking,
planner search and AST representation.
