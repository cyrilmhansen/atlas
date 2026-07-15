# K-M5 experimental decision-overlay specification

Status: private parser/validator checkpoint complete under DEC-067; facts and evaluator pending

Compatibility: none; the format may be replaced or deleted after K-M5

Authority: `registry/atlas.yaml` and frozen import worksheets remain primary

## Experiment question

Can a small structured overlay change the seven accepted decisions correctly,
without candidate-specific logic, a general expression language or a public
schema migration, and at an authoring cost low enough to justify a schema 0.2
proposal?

## Control

Run every case first with schema 0.1 and existing generic query behavior. The
expected control results are:

| Case | Schema 0.1 result |
|---|---|
| Dijkstra path tree supplies all distances | unsupported: no directional projection relation |
| Misra-Gries candidate stage is not exact output | unsupported outside prose: no qualified-output relation |
| Heap push is allocation-free with spare capacity | unsupported conditionally: allocation/capacity regime is prose |
| Constructed state flows into a later operation | unsupported: state identity/continuity is prose |
| Bloom negative is sound but positive is approximate | unsupported as a generic qualification |
| Online moments finalizer and IEEE-754 order boundary | unsupported as typed selection conditions |
| Pairwise identity requires `proven` evidence | unsupported: no proof-mapping validation |

An existing exact-identity or coarse-property match may still work. That does
not count as support for the missing decision.

## Initial private document shape

The first implementation uses one YAML document with these closed sections:

```yaml
overlay_version: phase2-km5-0

atoms:
  - id: output.distances.all_reachable
    kind: capability

candidates:
  - id: candidate.example
    source: registry:algorithm.example
    provides:
      - atom: output.distances.all_reachable
        evidence: { level: declared, source: "docs:source", proof: null }
    requires: []
    guarantees: []
    effects: []
    consumes_state: null
    produces_state: null
    costs: []

relations:
  - from: output.path_tree.all_reachable
    to: output.distances.all_reachable
    kind: projects_to
    requires: []
    evidence: { level: declared, source: "docs:source", proof: null }

requests:
  - id: request.example
    accepts: output.distances.all_reachable
    provides_conditions: []
    requires_guarantees: []
    forbids_effects: []
    consumes_state: null
    maximum_costs: []
    accepted_evidence: [declared, inferred, tested, observed, proven]
```

This example defines shape, not accepted atoms or candidates. The implementation
must deserialize into closed Rust enums and reject unknown fields.

## Closed vocabulary

Atom kinds:

- `capability`;
- `condition`;
- `guarantee`;
- `effect`;
- `state`.

Relation kinds:

- `projects_to`: a richer output can soundly provide a narrower output;
- `specializes`: a strategy/contract is a conditioned specialization of another;
- `finalizes`: a state can produce an output under explicit domain conditions;
- `refines`: a stage produces a stronger guarantee than another stage.

Cost regimes:

- `worst`;
- `expected`;
- `amortized`.

Cost metrics in the first experiment are `time`, `retained_memory` and
`allocation`. Bounds remain opaque normalized atoms or strings; the evaluator
does exact matching and does not order asymptotic expressions.

Evidence levels retain schema 0.1 names but the experiment defines no total
ordering among them. A request lists the exact accepted levels. A fact at
`proven` additionally requires
a proof reference with artifact locator, exact claim locator and review method,
as required by DEC-068. The validator rejects incomplete proof metadata.

## Evaluation

For one request, the evaluator:

1. discovers every candidate from the overlay, without a hard-coded candidate
   list in Rust;
2. follows only explicitly allowed directional relations;
3. checks set inclusion for conditions, guarantees and forbidden effects;
4. checks exact state-atom equality when state is required;
5. checks an exact cost profile and membership in the request's accepted
   evidence levels;
6. returns accepted and rejected candidates with one reason per unsatisfied
   fact.

There is no negation beyond `forbids_effects`, disjunction, arithmetic,
variable binding, recursion, transitive closure, score or tie-breaker. A request
needing one of those features is an experiment failure, not permission to add it.

## Initial complexity budget

- at most 8 candidates;
- at most 32 atoms;
- exactly the 4 relation kinds above;
- exactly the 3 cost regimes and 3 cost metrics above;
- one evaluator module below 300 non-test Rust lines;
- no new runtime dependency;
- no public CLI command in the first slice.

Exceeding a limit pauses implementation and requires review. Tests may expose a
private library API; a later CLI demonstration is a separate reversible choice.

The initial typed model, structured errors and validator occupy 557 non-test
Rust lines. This count is recorded as experiment cost; it is not hidden by
splitting files or excluded from the K-M5 synthesis. The separate evaluator
budget remains below 300 non-test lines.

## Acceptance tests

- unknown fields, duplicate IDs, dangling atoms and invalid relation endpoints
  are rejected with field-specific errors;
- `proven` without complete proof metadata is rejected;
- candidates are discovered from YAML, not named in evaluator code;
- every control case remains unsupported without the overlay;
- the overlay produces the adjudicated result for all seven cases;
- a negative candidate reports the missing condition, guarantee, state, cost or
  evidence rather than disappearing silently;
- changing one YAML fact changes the decision without recompiling Rust;
- the registry digest and SQLite projection are unchanged.

## Cost and exit record

K-M5 records:

- authoring time and lines for overlay facts;
- number of reused versus candidate-specific atoms;
- parser, validator and evaluator non-test line counts;
- decisions corrected relative to the control;
- disagreements encountered when two authors encode one additional candidate.

Delete the overlay if it corrects no decision, needs source-specific evaluator
branches or exceeds the bounded language. Consider a public proposal only when
at least two relation/fact concepts improve decisions in two structural families
and independent authoring produces operationally compatible facts.
