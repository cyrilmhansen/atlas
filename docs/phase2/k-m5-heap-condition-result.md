# K-M5 conditioned heap-equivalence result

Status: reversible implementation checkpoint; retention decision pending

Date: 2026-07-15

## Question

Can the bounded equivalence resolver reconcile a conditioned allocation-cost
fact with a no-allocation guarantee without accepting either encoding when its
`spare_capacity` condition is absent?

This is the second structural-family test requested by the DEC-069 result. It
uses the existing Rust `BinaryHeap::push` implementation and K-M4 adjudication;
it imports no new corpus fact.

The condition-aware code is a reversible class B trial beyond DEC-069's initial
unconditional-cost boundary. Retaining it requires the K-M5 closure decision;
the accepted public schema and query behavior remain unchanged meanwhile.

## Fixture

`k-m5-normalization-b-heap.yaml` contains two encodings of
`priority_queue.push.rust.std.1_85`:

- cost encoding: worst allocation `none`, requiring `spare_capacity`;
- guarantee encoding: `no_allocation`, with `spare_capacity` as a candidate
  requirement.

One equivalence relates the exact conditioned cost to the guarantee. The
resolver now requires every condition listed by a source or target cost
assertion to be present in the request. It remains direct, bidirectional and
non-recursive.

## Decisions

| Request | Without equivalence | With equivalence |
|---|---|---|
| no-allocation guarantee, spare capacity | guarantee encoding only | both encodings |
| no-allocation guarantee, no spare capacity | neither | neither |
| allocation-cost `none`, spare capacity | cost encoding only | both encodings |
| allocation-cost `none`, no spare capacity | neither | neither |

The equivalence improves substitution only in the valid regime and introduces
no false acceptance at the boundary.

## Cost

| Component | DEC-069 checkpoint | Condition-aware checkpoint |
|---|---:|---:|
| Overlay model, parser and validator | 761 | 762 non-test Rust lines |
| Evaluator | 271 | 271 non-test Rust lines |
| Equivalence resolver | 114 | 136 non-test Rust lines |
| Total private Rust experiment | 1,146 | 1,169 non-test Rust lines |
| Additional heap fixture | 0 | 117 YAML lines |

Condition transport adds 23 non-test Rust lines. It does not add an assertion
kind, dependency, recursive rule, public command or schema field.

## Interpretation

The second family supports the same bounded equivalence mechanism and shows
that explicit cost conditions can be transported without a general expression
language. This is useful positive evidence for `normalization-B`.

It does not erase the larger cost: K-M5 now contains 1,169 private non-test Rust
lines plus several experimental YAML documents. The overlay facts are still
authored beside schema 0.1 rather than discovered from schema 0.1 manifests.
Retention and K-M5 closure therefore remain a human decision.
