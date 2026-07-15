# K-M5 bounded encoding-equivalence experiment

Status: initial DEC-069 checkpoint complete; condition-aware follow-up measured separately

Date: 2026-07-15

Compatibility: none; private and disposable with the rest of K-M5

## Question

Can explicit bounded equivalences reconcile the two independently observed
top-k encodings without canonicalizing either one or adding source-specific
logic?

## Representation

An equivalence contains two nonempty assertion sets and its own evidence:

```yaml
equivalences:
  - id: equivalence.example
    left:
      - { kind: capability, atom: result.fused }
    right:
      - { kind: capability, atom: result.base }
      - { kind: guarantee, atom: guarantee.exact }
    evidence:
      level: declared
      source: "docs:reviewed-mapping"
      proof: null
```

Allowed assertion kinds are `capability`, `guarantee`, `effect` and `cost`.
Costs carry the same operation, metric, regime and opaque bound as candidate
costs, but their `requires` list must be empty. Conditional cost equivalence is
rejected because the resolver does not transport request conditions.

The resolver checks direct candidate assertions only. It never uses the result
of one equivalence as input to another and never chains an equivalence with a
directional relation. Both the source evidence and mapping evidence must be in
the request's exact accepted-level set.

## Top-k fixture

`k-m5-normalization-b.yaml` contains two representations of the same registry
implementation:

- fused: `result.top_k.exact_occurrences` plus an allocation effect;
- decomposed: `result.top_k.occurrences`, `guarantee.exact` and an exact
  allocation-cost assertion.

Two equivalences connect the output encodings and allocation encodings. Four
cross-encoding requests produce:

| Request | Without equivalences | With equivalences |
|---|---|---|
| decomposed exact output | decomposed only | both encodings |
| fused exact output | fused only | both encodings |
| forbid allocation | decomposed incorrectly accepted | neither encoding |
| exact allocation profile | decomposed only | both encodings |

Removing the equivalences in the test produces the control column. No candidate
or source-family name occurs in the production resolver.

## Cost

| Component | Before DEC-069 | After DEC-069 |
|---|---:|---:|
| Overlay model, parser and validator | 596 | 761 non-test Rust lines |
| Evaluator | 230 | 271 non-test Rust lines |
| Equivalence resolver | 0 | 114 non-test Rust lines |
| Total private Rust experiment | 826 | 1,146 non-test Rust lines |
| Cross-encoding YAML fixture | 0 | 119 lines |

The evaluator remains below its original 300-line limit, but the compatibility
option adds 320 non-test Rust lines overall. Splitting the resolver into a
separate module is an ownership boundary, not a claim that this cost disappears.

## Result and limits

The initial bounded mapping corrects all four cross-encoding decisions and
respects mapping evidence independently from candidate evidence. Invalid atom
kinds and tautological sides are rejected. Conditional costs were rejected at
this checkpoint pending the separate heap falsifier.

This is evidence that explicit compatibility mappings can preserve multiple
source-faithful taxonomies. It is also evidence that the approach is materially
more expensive than canonicalization. This checkpoint covers one streaming
subject only and does not justify a public ontology or schema field.

Verification at this checkpoint:

- `cargo test -p atlas --locked`: 155 tests pass;
- 16 focused decision-overlay/evaluator tests pass;
- `cargo check -p atlas --all-targets --locked` passes;
- formatting and whitespace checks pass.

The heap spare-capacity follow-up is complete in
`k-m5-heap-condition-result.md`. It transports explicit cost conditions without
adding an assertion kind and raises the private Rust total to 1,169 lines. The
K-M5 exit recommendation is recorded in `k-m5-review.md`.
