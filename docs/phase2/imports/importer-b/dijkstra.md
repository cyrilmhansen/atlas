# K-M0 importer B worksheet: Dijkstra

Protocol revision: `k-m0.1`

## 1. Work record

- Importer identifier: `importer-b`
- Start and end time: 2026-07-14, approximately 23:07-23:17 Europe/Paris
- Active authoring minutes: approximately 6
- Source-reading minutes: approximately 2
- Atlas-modeling minutes: approximately 2
- Human interventions and their duration: none
- Tools used beyond browser, editor and existing Atlas CLI: web search/open tool
  for the pinned pages; no Atlas CLI command was needed

## 2. Source identity

- Source subject and source-local name: Dijkstra shortest paths; `dijkstra` in
  petgraph, `DijkstraSP` in algs4
- Authors or maintainers: petgraph maintainers; Robert Sedgewick and Kevin Wayne
- Work or project title: petgraph; *Algorithms, 4th Edition* and `algs4.jar`
- Edition, release, tag or commit: petgraph `0.8.3`; *Algorithms, 4th Edition*,
  2011
- Section, page, module or symbol: `petgraph::algo::dijkstra::dijkstra`;
  `edu.princeton.cs.algs4.DijkstraSP`, Section 4.4
- Stable URLs:
  - <https://docs.rs/petgraph/0.8.3/petgraph/algo/dijkstra/fn.dijkstra.html>
  - <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/DijkstraSP.java.html>
- Retrieval date: 2026-07-14
- Source class: library and book-associated educational library
- Code license: petgraph `MIT OR Apache-2.0`; algs4 GPLv3 or later
- Documentation license or copyright status: petgraph rustdoc accompanies the
  dual-licensed crate; algs4 page copyright Robert Sedgewick and Kevin Wayne
- Additional pages consulted: none

## 3. Source-faithful account

### petgraph 0.8.3

- Problem stated by the source: compute shortest-path length from `start` to
  every reachable node, optionally stopping once a specified goal cost is
  calculated (`dijkstra`, description and `goal` argument).
- Inputs and their representation: weighted graph `G: IntoEdges + Visitable`, a
  `NodeId` start, optional `NodeId` goal, and mutable edge-cost callback mapping
  each `EdgeRef` to a copyable `Measure` (`dijkstra`, signature and arguments).
- Preconditions and validity domain: edge costs returned by the callback must be
  nonnegative (`dijkstra`, description). Valid start/goal behavior and overflow
  conditions are not stated.
- Output or observable interface: a `hashbrown::HashMap<NodeId, K>` from each
  reached node to path cost (`dijkstra`, Returns). With a goal, the map may only
  cover nodes processed before termination; exact partial-map semantics beyond
  termination at calculated goal cost are not stated.
- Postconditions and guarantees: without early goal, it computes shortest-path
  lengths to every reachable node. It does not return predecessors or paths
  (`dijkstra`, description and Returns).
- Strategy: identified as Dijkstra's shortest path algorithm. Queue structure,
  relaxation invariant, and tie handling are not stated on the API page.
- Named invariants: not stated.
- Persistent and temporary state: function-local state is not documented;
  returned hash map persists. The edge-cost closure is `FnMut`, so its own state
  may be mutated (`dijkstra`, signature).
- Mutations, allocation, I/O and failure behavior: allocation of the returned
  hash map is observable from the return type; other allocation, graph mutation,
  I/O, panic, and error behavior are not stated.
- Time claims, including operation and case being bounded:
  O((|V| + |E|) log |V|) (`dijkstra`, Complexity).
- Space claims, including what is excluded: auxiliary O(|V| + |E|), with no
  exclusions stated (`dijkstra`, Complexity).
- Determinism, randomness or numerical assumptions: no randomness is stated.
  Determinism depends on graph iteration, cost type, callback behavior and tie
  handling, none of which is fully specified. `Measure` arithmetic conditions
  are not described on this page.
- Variants explicitly distinguished by the source: all-reachable (`goal=None`)
  and early-goal (`goal=Some`) modes.
- Ambiguities or internally inconsistent statements: the headline says lengths
  to every reachable node while optional-goal mode explicitly stops early; the
  all-reachable guarantee must therefore be conditioned on no goal. The
  O(|V|+|E|) auxiliary-space claim is not decomposed despite a node-cost map.

### Algorithms, 4th Edition

- Problem stated by the source: solve single-source shortest paths in an
  edge-weighted directed graph with nonnegative weights and construct a
  shortest-path tree to every vertex (`DijkstraSP.java`, lines 35-45, 67-75).
- Inputs and their representation: `EdgeWeightedDigraph` with integer vertex
  IDs, double-valued `DirectedEdge` weights, and one integer source (lines
  62-87).
- Preconditions and validity domain: all edge weights are nonnegative and the
  source lies in `0..V-1`; invalid cases throw `IllegalArgumentException`
  (lines 67-87). Correctness additionally assumes arithmetic without
  floating-point rounding error or overflow; integer weights and intermediate
  results no greater than 2^52 are a stated sufficient condition (lines 47-53).
- Output or observable interface: eager object with `distTo(v)`, `hasPathTo(v)`,
  and `pathTo(v)`; unreachable distance is positive infinity and unreachable
  path is `null` (lines 110-149).
- Postconditions and guarantees: distances and predecessor edges form a
  shortest-path tree. Optimality checks require source distance zero, consistent
  reachability state, every edge relaxed, and equality along tree edges (lines
  150 onward, especially 150-180).
- Strategy: binary-heap Dijkstra; initialize distances to infinity except source
  zero, repeatedly delete the minimum-distance vertex, and relax outgoing edges,
  inserting or decreasing a priority-queue key (lines 39-45, 80-108).
- Named invariants: `distTo[v]` is shortest distance, `edgeTo[v]` the last edge
  on a shortest path, and `pq` the indexed minimum-priority queue (lines 62-65).
  The check states edge feasibility and tree-edge tightness (lines 150-180).
- Persistent and temporary state: V-sized `distTo` and `edgeTo` arrays remain as
  object state; an indexed minimum priority queue is used during construction
  (lines 62-65, 80-108).
- Mutations, allocation, I/O and failure behavior: construction scans every
  edge for negative weights, allocates arrays and priority queue, mutates them
  during relaxation, and throws for negative weights or invalid vertices. Core
  class does not perform I/O; its demonstration `main` does.
- Time claims, including operation and case being bounded: constructor worst
  case Theta(E log V) using a binary heap; each instance method claimed Theta(1)
  (lines 39-45). As with BFS, the latter is ambiguous for `pathTo`, which builds
  a path sequence (lines 133-149).
- Space claims, including what is excluded: Theta(V) extra space excluding the
  edge-weighted digraph (lines 43-45).
- Determinism, randomness or numerical assumptions: no randomness. Numerical
  assumptions are explicit as above. Choice among equal-cost paths is not
  specified.
- Variants explicitly distinguished by the source: this class is complete
  single-source directed shortest paths; no optional-goal variant is offered.
- Ambiguities or internally inconsistent statements: blanket Theta(1) for each
  instance method is inconsistent with materializing an iterable path of
  nonconstant length; do not normalize that claim for `pathTo`.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason:
  `graph.nonnegative_weight_single_source_shortest_distance` for petgraph and
  `graph.nonnegative_weight_single_source_shortest_paths` for algs4. A distance
  map is not a substitute for reconstructible paths, and early-goal changes
  output completeness.
- Proposed `Algorithm` identity and reason: common family identity
  `graph.dijkstra_nonnegative_weights`, with source-local variants for
  `all_distances`, `goal_distance`, and `shortest_path_tree`. Schema 0.1 cannot
  encode that family/variant relation. For comparison I would create two
  algorithm proposals tied to their exact problem contracts and retain the
  family identity as an experimental annotation.
- Proposed implementation identity, if executable source is in scope: none;
  upstream code is documentary-only in K-M0.
- Existing Atlas entity that may be synonymous: none.
- Proposed problem `input`, `requires`, `output`, `ensures`:
  - distance form: input `weighted graph, source, optional goal, edge-cost
    function`; requires `valid nodes and nonnegative costs`; output `cost map for
    all reachable nodes when no goal, or a source-defined partial map including
    the calculated goal when present`; ensures `reported target costs are
    shortest-path costs under the callback`.
  - path-tree form: input `double-weighted directed graph and source`; requires
    `valid source, nonnegative weights, and arithmetic without relevant rounding
    or overflow`; output `reachability, distances, and reconstructible path for
    every vertex`; ensures edge feasibility and tight predecessor edges.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  nonnegative costs; for algs4 also explicit safe-arithmetic domain. Petgraph
  declared time O((V+E) log V), auxiliary O(V+E); algs4 binary-heap constructor
  Theta(E log V), auxiliary Theta(V) excluding graph. Preserve claims separately
  because their accounting and graph assumptions differ. Determinism is
  `uncertain` for returned predecessor choice; distances are deterministic only
  under deterministic exact cost arithmetic and callback.
- Proposed implementation effects and tests: later tests should cover
  unreachable nodes, zero-weight edges, equal-cost ties, parallel edges,
  directed cycles, optional early goal, invalid/negative weights, integer safe
  arithmetic, floating rounding sensitivity, and predecessor-path consistency.
- Proposed evidence level for each claim and why: source contracts, conditions,
  and bounds `declared`; source code's assertions do not make Atlas evidence
  `tested`; later independent differential checks may become `tested`.
- Candidate relationships to other imported subjects: BFS shortest paths are a
  specialization for unit weights. Neither petgraph distance output nor algs4
  tree output is a full substitute for the other in both directions.
- Information intentionally left documentary: concrete Rust trait bounds,
  binary heap/indexed PQ representation, Java exception type, floating-point
  sufficient bound 2^52, tie-selected predecessor, and exact partial-map
  contents under early termination.

Alternative normalization: one problem with optional requested output and
optional goal. I reject it for this pilot because free-text options cannot stop
distance-only and partial-output candidates from being treated as complete
path-tree substitutes.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: function/class symbols, pinned release,
  edition and Section 4.4 are retained.
- Missing or unstable source identity: no algs4 commit; petgraph release docs are
  not commit-pinned.
- Assessment: partial.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: nonnegative-cost
  Dijkstra, algs4 binary heap and relaxation conditions, petgraph optional-goal
  behavior, and arithmetic validity constraints remain explicit.
- Semantic details lost or altered: petgraph's exact early-result map and
  numerical `Measure` behavior are unresolved; tie choice is not modeled.
- Assessment: preserved for separately scoped contracts; partial if represented
  only by a single family label.

### Representational fidelity

- Source vocabulary and decomposition retained: cost callback/map versus
  directed graph/path-tree object remains visible.
- Normalized or collapsed concepts: concrete generic traits and Java arrays/PQ
  become documentary input/state descriptions.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: yes under source licenses.
- Correction oracle proposed: compare exact reachable distances in integer
  cases; validate every returned path and its summed weight; separately test
  partial early-goal behavior and floating-point boundary cases.
- Behavior actually checked during this worksheet: none in K-M0.
- Assessment: not assessed.

### Declared transformations

- translation: independently worded contracts from Rust and Java interfaces.
- specialization or generalization: distinguish all-distance, goal-distance,
  and shortest-path-tree variants within a documentary Dijkstra family.
- type or representation adaptation: generic `Measure` and Java `double` remain
  separate numerical domains rather than being coerced to one type.
- API decomposition or aggregation: algs4 constructor/query object is treated as
  one eager result; petgraph function remains a map-producing operation.
- bug correction: none.
- pedagogical simplification: concrete queue mechanics are algorithm/implementation
  detail, not part of the problem identity.
- other: ambiguous constant-time `pathTo` claim is not imported.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Distances versus paths/tree output | problem output prose | lossy | identity / substitution | experimental annotation |
| Optional goal and partial result | input/output prose | ambiguous | selection / substitution / composition | worksheet |
| Nonnegative generic `Measure` domain | algorithm requires prose | lossy | selection | worksheet |
| Floating rounding/overflow validity | algorithm requires prose | lossy | selection / substitution | experimental annotation |
| Cost callback may carry mutable state | implementation effects | ambiguous | composition | worksheet |
| Several complexity operations/accounting conventions | one time and memory claim each | lossy | selection | worksheet |
| Directed graph and weighted edges | problem input prose | lossy | selection / composition | worksheet |
| Dijkstra family/variant relation | no relation field | absent | identity / substitution | experimental annotation |
| Graph, map, heap, weights in AST | private AST | absent | documentary only | worksheet |

Collapsing output variants makes distance-only, early partial, and path-tree
candidates indistinguishable and can select an invalid substitute. Arithmetic
domain loss can select a floating implementation where exactness is required.
The graph/type portions are family-specific, but output capability and numeric
validity recur in streaming/numerical families; at least one such second family
is needed before schema extension.

## 7. Operational probes

- identity lookup: unsupported; no source aliases or family relations exist.
- search by the proposed vocabulary: unsupported until records exist; lexical
  search would not equate shortest-distance and shortest-path-tree contracts.
- qualification by current properties: would require source-specific logic for
  directedness, nonnegative numerical domain, output completeness, early goal,
  and path reconstruction.
- substitution between source variants: unsupported; free-text outputs cannot
  establish the one-way capability relationships.
- composition with a stated precondition/effect: would require source-specific
  logic for weighted graph compatibility, cost callback purity, and numeric
  safety.

All outcomes are hypothetical; no CLI probe was executed.

## 8. Importer conclusion

- Recommended normalization: keep two exact problem/algorithm contracts and a
  non-normative Dijkstra family relation; do not collapse distance, early-goal,
  and path-tree outputs.
- Unresolved questions: exact petgraph map contents under early goal; `Measure`
  overflow/ordering requirements; how to express numerical validity without
  making prose look selectable.
- Decision-relevant loss: output capability and arithmetic domain directly
  alter selection and substitution.
- Documentary divergence that should remain acceptable: generic traits versus
  Java concrete types, chosen heap, exception spelling, and equal-cost
  predecessor selection.
- Proposed next minimal experiment: construct a manual acceptance matrix for
  four requests (all distances, one goal cost, all reconstructible paths, exact
  integer result) and test each source contract without running upstream code.
- Schema or AST change requested now: none.
