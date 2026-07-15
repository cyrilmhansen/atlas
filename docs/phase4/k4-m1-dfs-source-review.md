# K4-M1 DFS source review

Status: source options reviewed; B1 selection pending

Date: 2026-07-15

## Target

Add one depth-first competitor for `graph.reachable_traversal` without changing
the problem identity, schema 0.1 or the K4-M0 requests.

The implementation must yield every node reachable from one valid source
exactly once, yield no other node and operate over a graph whose nodes and
adjacency remain stable during traversal.

## Option A - Existing petgraph 0.8.3 DFS

Use [`petgraph::visit::Dfs`](https://docs.rs/petgraph/0.8.3/petgraph/visit/struct.Dfs.html)
through the already pinned, default-features-disabled dependency. The upstream
type is non-recursive, yields nodes in discovery preorder, visits only nodes
reachable from the start, and exposes an explicit stack plus discovered map.
The crate is
[`MIT OR Apache-2.0`](https://docs.rs/crate/petgraph/0.8.3/source/Cargo.toml.orig).

Cost:

- one registry Algorithm and Implementation;
- one thin adapter test beside the existing petgraph BFS tests;
- no dependency or feature change;
- one source-faithful import worksheet and K4-M0 adjudication matrix.

Risks:

- BFS and DFS implementations come from the same upstream project, weakening
  implementation-source diversity;
- traversal order must be recorded as DFS preorder, not assumed to preserve
  breadth-first or shortest-hop order;
- the library documents `O(V)` structural storage through stack/discovered
  fields but does not itself state the desired conditioned frontier claim.

Reversibility: high. Removing the two registry entities and focused test fully
removes the competitor without changing a dependency.

## Option B - rs-graph 0.21 DFS

Use the iterator-based DFS in
[`rs_graph::search`](https://docs.rs/rs-graph/0.21.0/rs_graph/search/index.html).
This provides an independently maintained implementation and explicitly models
search algorithms as visit iterators.

Cost and risks:

- new graph library, graph representation and adapter boundary;
- transitive numeric dependencies unrelated to the experiment;
- the crate is GPL-3.0+, which is incompatible with Atlas's desired simple MIT
  distribution boundary for a linked shipped implementation;
- substantially more integration work may measure representation conversion
  rather than comparative selection.

Reversibility: medium. A test-only dependency could be removed, but license and
distribution review would precede even experimental integration.

## Option C - Atlas implementation from an external description

Write an independent iterative DFS adapter from a cited educational or primary
description. Princeton's
[`DepthFirstSearch`](https://algs4.cs.princeton.edu/41undirected/DepthFirstSearch.java.html)
states linear worst time and linear extra space, while its
[`algs4.jar` distribution is GPLv3](https://algs4.cs.princeton.edu/code/), so
Atlas would cite the source and not copy its code.

Cost and risks:

- no new dependency and strong control over the exact contract;
- the resulting implementation is written for Atlas, not a foreign
  implementation, weakening the central Phase 4 experiment;
- code review would need to establish that the translation is not presented as
  upstream code or as independent executable provenance.

Reversibility: high, but epistemic value is lower than Option A.

## Recommendation

Recommend **Option A** for K4-M1.

It changes the algorithmic strategy while keeping language, graph storage,
target and dependency constant. That controlled comparison isolates the Atlas
selection model instead of measuring FFI, conversion or licensing complexity.
The same-upstream limitation will be explicit, and K4-M2/K4-M3 must restore
source diversity rather than treating this first batch as sufficient evidence.

## Minimum experiment

1. Add DFS Algorithm and Implementation manifests with exact petgraph sources.
2. Add one focused adapter test covering empty-outside-component nodes, a cycle,
   a self-loop and deterministic repeated traversal.
3. Prove candidate discovery by enumerating `solves`/`implements`, not IDs.
4. Adjudicate all four frozen graph requests before choosing B2.
5. Stop K4-M1 after the two-candidate matrix and implementation-cost record.

Validation question: accept `dfs-A`, choose `dfs-B`, or choose `dfs-C`?
