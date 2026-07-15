# K4-M1 graph reachability competition result

Status: complete; first foreign competition supported with qualifications

Date: 2026-07-15

Authority: DEC-074, `dfs-A` and `docs/phase4/k4-m0-protocol.md`

## Result

`graph.dfs.traversal` and `graph.dfs.petgraph.0_8_3` are registered as a second
Algorithm and Implementation for the existing `graph.reachable_traversal`
Problem. No problem, dependency, schema field, CLI branch or runtime capability
was added.

The registry now discovers two algorithms and two implementations for the
problem exclusively through `solves` and `implements`. The focused adapter
calls upstream petgraph DFS and verifies exact component traversal,
single-visit behavior over cycles and a self-loop, an isolated source and
repeatable preorder.

## Frozen-request adjudication

| Request | BFS | DFS | Evidence boundary |
|---|---|---|---|
| `reach.exact_component` | `accepted` | `accepted` | both solve the exact problem and their upstream adapter tests yield only the reachable component once |
| `reach.no_allocation` | `rejected` | `rejected` | both Implementation effects declare allocated frontier and discovered-node storage |
| `reach.frontier_for_known_shape` | `unsupported-fact` | `unsupported-fact` | both expose only unconditional `O(V)` auxiliary memory; neither conditions retained frontier on width or depth |
| `reach.non_decreasing_hops` | `unsupported-fact` | `rejected` | BFS traversal has no explicit hop-order claim in the registry; a tested DFS counterexample visits a depth-two node before another depth-one node |

The matrix was not changed after import. It demonstrates all three
decision-relevant states present in this batch: acceptance, contradiction and
missing authoritative qualification.

## Candidate-discovery falsifier

The discovery test:

1. filters Algorithms by `solves == graph.reachable_traversal`;
2. filters Implementations whose `implements` refers to those Algorithms;
3. observes two Algorithms and two Implementations.

The discovery logic contains no BFS or DFS candidate identifier. Expected
counts and versions are assertions after discovery, not branches selecting the
candidates. The automatic-discovery falsifier therefore passes for K4-M1.

## Source fidelity and limits

- petgraph 0.8.3 code is executed directly; Atlas only converts `NodeIndex` to
  integers in assertions;
- DFS preorder, explicit stack and discovered map remain visible in the
  manifest and import record;
- asymptotic claims remain inferred and are not upgraded by behavioral tests;
- BFS and DFS share the same upstream project, so this is controlled strategy
  diversity rather than independent-project diversity;
- graph-shape-conditioned memory remains absent and supplies the first Phase 4
  `unsupported-fact` evidence, but one family cannot justify schema change.

## Cost

The implementation adds one manifest Algorithm, one manifest Implementation,
one source-faithful worksheet and two focused upstream-adapter tests. It adds no
production Rust, dependency, feature or public interface.

The derived index contains 109 entities, 78 relations and 702 claims. Its
logical SHA-256 is
`e013e76e7c0f7b12ade8140996ced428f49814a23bcee1891455a7ca636ba04c`.

## Consequence

K4-M1 supports automatic foreign candidate discovery and disciplined
adjudication. It does not yet prove a shared executable qualification model.
Before K4-M2, B2 must select whether to reuse the unchanged K-M5 evaluator with
phase-local data, build a smaller disposable evaluator or keep the matrix
documentary until a second family is imported.
