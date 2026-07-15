# K4-M1 import - petgraph depth-first traversal

Status: accepted under `dfs-A`

Date: 2026-07-15

## Identity

- Problem: `graph.reachable_traversal` (existing)
- Algorithm: `graph.dfs.traversal`
- Implementation: `graph.dfs.petgraph.0_8_3`
- Upstream: petgraph 0.8.3, `petgraph::visit::Dfs`
- License: MIT OR Apache-2.0

Primary implementation documentation:
[`Dfs`](https://docs.rs/petgraph/0.8.3/petgraph/visit/struct.Dfs.html).
Pinned dependency and license:
[`petgraph 0.8.3 Cargo.toml`](https://docs.rs/crate/petgraph/0.8.3/source/Cargo.toml.orig).

## Source contract

The upstream type performs a non-recursive depth-first traversal, emits nodes
in discovery preorder and restricts traversal to nodes reachable from the start.
Its public state consists of a node stack and a discovered-node map. Removing
nodes during iteration is unsupported; added nodes or edges are not guaranteed
to be visited.

These statements map to the existing Atlas problem without changing it:

- the start is `input.source`;
- each yielded upstream node is one item of `IncrementalTraversal<Node>`;
- the stack is the frontier and the visit map prevents duplicate yields;
- the graph stability precondition already excludes the documented mutation
  ambiguity.

## Fidelity

- **Bibliographic:** exact crate version, API path, license and pinned manifest
  are recorded.
- **Algorithmic:** Atlas retains iterative LIFO depth-first preorder and the
  discovered set; it does not rewrite DFS as recursion.
- **Representational:** registry effects name the upstream stack and discovered
  map. Internal generic types are summarized, not copied.
- **Executable:** the Atlas integration test calls upstream `Dfs::new` and
  `Dfs::next` directly on a petgraph graph.

Declared transformation: the test converts yielded `NodeIndex` values to
numeric indices only for assertions. No algorithm code is translated into
Atlas.

## Claims and evidence

- exact reachable-component behavior is tested on a cycle, a self-loop,
  disconnected nodes and an isolated source;
- repeated traversal over unchanged adjacency is tested for the same preorder;
- `O(V + E)` worst time and `O(V)` auxiliary memory are inferred from one
  discovered-map visit per node, adjacency iteration and bounded stack state;
- tests establish behavior, not those asymptotic claims;
- nondecreasing hop order and graph-shape-conditioned frontier bounds are not
  claimed.

## License boundary

Atlas links the already accepted MIT/Apache-2.0 petgraph dependency. Princeton
DFS material remains a separate explanatory source in the source review; no
GPL code or prose is copied into the implementation.

## Known limitation

The competing BFS and DFS implementations come from the same upstream project.
This controls representation and dependency effects but weakens independent-
project diversity. K4-M1 must not be treated as sufficient cross-source
selection evidence by itself.
