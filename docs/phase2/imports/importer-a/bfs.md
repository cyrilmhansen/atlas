# K-M0 import A: breadth-first search

Protocol: `k-m0.1`
Status: independent documentary import; no registry entity created

## 1. Work record

- Importer identifier: `importer-a`.
- Start and end time: 2026-07-14, one authoring session.
- Active authoring minutes: approximately 45 minutes for this worksheet.
- Source-reading minutes: approximately 20 minutes.
- Atlas-modeling minutes: approximately 25 minutes.
- Human interventions and their duration: source-pair selection was fixed before
  the import; no intervention during authoring.
- Tools used beyond browser, editor and existing Atlas CLI: `curl` and `pandoc`
  to inspect the pinned rustdoc page and the source page directly linked from it.

## 2. Source identity

### petgraph

- Source subject and source-local name: `Bfs<N, VM>`.
- Authors or maintainers: petgraph maintainers.
- Work or project title: `petgraph`.
- Edition, release, tag or commit: release `0.8.3`.
- Section, page, module or symbol: `petgraph::visit::Bfs`.
- Stable URL: <https://docs.rs/petgraph/0.8.3/petgraph/visit/struct.Bfs.html>.
- Retrieval date: 2026-07-14.
- Source class: library.
- Code license: `MIT OR Apache-2.0`.
- Documentation license or copyright status: distributed with the crate under
  the crate's dual license.
- Additional page consulted: source linked by rustdoc,
  <https://docs.rs/petgraph/0.8.3/src/petgraph/visit/traversal.rs.html#256-309>.

### Algorithms, 4th Edition

- Source subject and source-local name: `BreadthFirstPaths`.
- Authors or maintainers: Robert Sedgewick and Kevin Wayne.
- Work or project title: *Algorithms, 4th Edition* and `algs4.jar`.
- Edition, release, tag or commit: fourth edition; source page updated
  2026-01-10.
- Section, page, module or symbol: Section 4.1; class
  `edu.princeton.cs.algs4.BreadthFirstPaths`.
- Stable URL:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/BreadthFirstPaths.java.html>.
- Retrieval date: 2026-07-14.
- Source class: book and educational library.
- Code license: GPLv3 or later.
- Documentation license or copyright status: copyrighted booksite material;
  only independently worded facts are recorded here.
- Additional pages consulted: none.

## 3. Source-faithful account

### petgraph account

- Problem stated by the source: traverse, in breadth-first order, the nodes
  reachable from one start node. It is explicitly a traversal rather than a
  shortest-path result object (`Bfs` rustdoc, description and `next`).
- Inputs and their representation: a graph providing `Visitable` when the
  walker is constructed and `IntoNeighbors` for each step, plus one `NodeId`;
  node IDs are copyable and the discovered state is a graph visitor map
  (`Bfs::new`, `Bfs::next`).
- Preconditions and validity domain: the start must be meaningful for the graph;
  the page does not state an explicit invalid-index result. Removing nodes while
  iterating may make behavior incorrect, and newly added nodes or edges are not
  guaranteed to be visited (`Bfs` rustdoc, mutation note).
- Output or observable interface: each `next(&graph)` yields the next node ID or
  `None`; the `Walker` item is a node ID (`Bfs::next`, `Walker` implementation).
- Postconditions and guarantees: only nodes reachable from the start are
  traversed; each node is admitted once through the visit map. The source does
  not promise distances, predecessors or paths (`Bfs` description; source
  lines 286-305).
- Strategy: enqueue the start, dequeue from the front, discover unvisited
  neighbors and enqueue them at the back (source lines 286-305).
- Named invariants: not stated by name. The implementation exposes a queue of
  nodes to visit and a map of discovered nodes (`Bfs` fields).
- Persistent and temporary state: the reusable walker owns `VecDeque<N>` and a
  visitor map; it does not own or continuously borrow the graph (`Bfs` fields
  and description).
- Mutations, allocation, I/O and failure behavior: `next` mutates the queue and
  visit map. Construction obtains a visit map and constructs a queue. No I/O is
  stated. Invalid start behavior is not stated; graph mutation has the caveats
  above.
- Time claims, including operation and case being bounded: not stated.
- Space claims, including what is excluded: not stated.
- Determinism, randomness or numerical assumptions: no randomness. Exact node
  order depends on the graph's neighbor iteration order; determinism is not
  claimed explicitly.
- Variants explicitly distinguished by the source: `Bfs` is nonrecursive and
  is distinct from other traversal walkers in the module; no multi-source
  constructor is exposed on this type.
- Ambiguities or internally inconsistent statements: the public field is named
  `stack` although its type and use are FIFO queue semantics. This is vocabulary,
  not a semantic contradiction.

### Algorithms, 4th Edition account

- Problem stated by the source: find shortest paths by number of edges from one
  source, or any of several sources, to every other vertex in an undirected
  graph (class documentation, lines 44-58).
- Inputs and their representation: an adjacency-list `Graph` whose vertices are
  integers `0..V-1`, and either one integer source or a nonempty iterable of
  integer sources (constructors, lines 69-100).
- Preconditions and validity domain: every source and queried vertex must lie in
  `0..V-1`; a multi-source iterable must be non-null, nonempty and contain no
  nulls (constructors and validation, lines 69-100 and 222-244).
- Output or observable interface: eager object with reachability, edge-count
  distance and one shortest vertex path queries. Unreachable distance is
  `Integer.MAX_VALUE` and unreachable path is `null` (`hasPathTo`, `distTo`,
  `pathTo`, lines 142-179).
- Postconditions and guarantees: marked vertices are exactly reachable vertices;
  returned distances are shortest edge counts; predecessor edges reproduce a
  path satisfying a unit-distance step relation (internal check, lines 181-219).
- Strategy: initialize source distance to zero, mark and enqueue sources, then
  dequeue vertices and discover each unmarked adjacent vertex while recording
  predecessor and distance (`bfs`, lines 102-140).
- Named invariants: no prose name is given. The executable check records source
  distance zero, edge feasibility, agreement of reachability across an edge and
  tight predecessor edges (lines 181-219).
- Persistent and temporary state: persistent `marked`, `edgeTo`, and `distTo`
  arrays; temporary FIFO queue during construction (lines 63-67 and 102-140).
- Mutations, allocation, I/O and failure behavior: construction allocates three
  `V`-length arrays and a queue and mutates them. The data type itself performs
  no I/O. Invalid vertices throw `IllegalArgumentException`.
- Time claims, including operation and case being bounded: constructor worst
  case is Theta(V + E); each instance query is Theta(1), though materializing
  the iterable path necessarily walks its predecessor chain in the shown code
  (class documentation lines 49-54; `pathTo` lines 171-179). This is an internal
  tension in the source's blanket query claim.
- Space claims, including what is excluded: Theta(V) extra, excluding the input
  graph (class documentation lines 49-54).
- Determinism, randomness or numerical assumptions: no randomness; which one of
  multiple shortest paths is retained depends on adjacency iteration order.
- Variants explicitly distinguished by the source: single-source and
  multi-source construction.
- Ambiguities or internally inconsistent statements: the claim that every
  instance method is Theta(1) conflicts with `pathTo` constructing a stack along
  a path of nonconstant length. Atlas should not silently promote that blanket
  claim.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason: two identities:
  `graph.reachable_vertices_from_source` for incremental reachable-node
  enumeration and `graph.unweighted_shortest_paths_from_sources` for eager
  distances and witness paths. Their observable outputs change whether a
  candidate can satisfy a caller, so collapsing them would make substitution
  unsafe.
- Proposed `Algorithm` identity and reason: respectively
  `graph.breadth_first_traversal` and
  `graph.breadth_first_shortest_paths`. They share the breadth-first strategy,
  but schema 0.1 binds an algorithm to one problem and cannot record that the
  second derives more outputs from the first strategy.
- Proposed implementation identity, if executable source is in scope: none in
  K-M0. Future candidates could be `rust.petgraph.bfs.0_8_3` and
  `java.algs4.breadth_first_paths`, but neither source is imported or linked.
- Existing Atlas entity that may be synonymous: none; current entities are
  sequence-only.
- Proposed problem `input`, `requires`, `output`, `ensures`: traversal input is a
  graph plus valid start, output is a lazy sequence of each reachable vertex
  once; shortest-path input is an undirected graph plus one or more valid
  sources, output is reachability, minimum edge count and a witness path for
  every vertex. The latter ensures each returned witness joins a source to its
  vertex and has the stated minimum edge count.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  valid graph/source and a stable graph topology during traversal; deterministic
  only relative to neighbor iteration order. Record Theta(V + E) time and
  Theta(V) auxiliary space for the algs4 eager form as source-declared. Leave
  petgraph complexity undeclared because its page states none.
- Proposed implementation effects and tests: walker mutates and allocates its
  own frontier/discovery state but does not mutate the graph; eager form
  allocates O(V) persistent result arrays. Future tests should cover disconnected
  graphs, cycles, isolated source, multiple shortest paths, multi-source ties and
  invalid source IDs.
- Proposed evidence level for each claim and why: source contracts and
  complexities are `declared`; strategy details independently read from source
  remain `declared`, not `tested`; no claim is `observed`, `tested` or `proven`
  in this worksheet.
- Candidate relationships to other imported subjects: unweighted shortest-path
  distances can be related to Dijkstra with unit edge costs, but schema 0.1 has
  no variant/equivalence relation.
- Information intentionally left documentary: source-local APIs, adjacency-order
  tie breaking, the mutable-graph caveat, the query-complexity tension and the
  derivation between traversal and path materialization.

Alternative normalization A, used for comparison above, separates the two
observable contracts. Alternative B would use one broad shortest-reachability
problem with optional outputs; schema 0.1 cannot express optional requested
outputs and could select a walker where witness paths are required. Alternative
C would normalize only the common reachable-set result; it is simpler but
intentionally discards decision-relevant distance and path capabilities.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: release, crate symbol, class, edition,
  section, authors, URLs and licenses.
- Missing or unstable source identity: petgraph release commit is not recorded
  separately from the immutable docs.rs release; algs4 has a page update date
  rather than a release tag.
- Assessment: preserved.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: FIFO frontier,
  discovery-on-enqueue, source domains, reachable-set semantics and algs4
  distance/predecessor guarantees.
- Semantic details lost or altered: none in the two proposed contracts; their
  common conceptual relation is not structurally preserved.
- Assessment: preserved separately, partial if forced into one Atlas identity.

### Representational fidelity

- Source vocabulary and decomposition retained: `Bfs` walker and
  `BreadthFirstPaths` result object remain distinct in documentary text.
- Normalized or collapsed concepts: graph traits and Java array/data-type details
  become free-text contracts.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: yes for both sources.
- Correction oracle proposed: compare yielded reachable set for petgraph; compare
  reachability, edge-count distances and witness validity for algs4 on the same
  small disconnected graphs.
- Behavior actually checked during this worksheet: none.
- Assessment: not assessed.

### Declared transformations

- translation: Rust and Java terminology to language-neutral graph contracts;
  reason: compare semantics without importing code.
- specialization or generalization: petgraph is kept single-source; algs4 keeps
  both single- and multi-source forms.
- type or representation adaptation: node traits and integer vertex IDs become
  abstract valid vertex identities.
- API decomposition or aggregation: the walker and eager path object are split
  into separate Atlas problems.
- bug correction: none; the algs4 blanket Theta(1) query claim is flagged rather
  than corrected.
- pedagogical simplification: none.
- other: no source code copied.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Walker versus eager path object | `Problem.output` free text | ambiguous | identity, substitution | worksheet |
| Same breadth-first strategy serves two contracts | one `Algorithm.solves` | absent | identity, composition | experimental annotation |
| Graph topology and vertex validity | `Problem.input` / `requires` text | lossy | selection | worksheet |
| Output is incremental and stateful | `Problem.output` text | lossy | substitution, composition | experimental annotation |
| Determinism depends on neighbor order | `Algorithm.deterministic` boolean | lossy | selection | worksheet |
| Graph may be mutated subject to traversal caveats | implementation `effects` text | lossy | composition | worksheet |
| Source-local aliases and API boundaries | no structured destination | absent | identity, documentary only | worksheet |
| Graph operations in AST | generic calls only | absent | documentary only | worksheet |

The first two rows can make walker and path-producing candidates
indistinguishable, select an invalid candidate for a witness-path request, or
reject a useful reachable-set substitution. The graph contract rows can also
admit an invalid directed/multi-source candidate if prose is ignored. These
issues are partly graph-family-specific, but incremental versus materialized
outputs recur in streaming algorithms; top-k streaming is the best second
family before any schema proposal. Conditional determinism also recurs in hash
tables and priority queues. AST absence is not an import gate.

## 7. Operational probes

- identity lookup: **unsupported** before entities exist; after a manifest-only
  import, exact ID lookup would be supported but source aliases would not.
- search by the proposed vocabulary: **supported** only for literal normalized
  terms present in claims; source-local alias mapping is unsupported.
- qualification by current properties: **unsupported** for graph direction,
  lazy/eager output, multi-source support and neighbor-order determinism.
- substitution between source variants: **unsupported** generically; treating
  the two APIs as interchangeable would be incorrect for path requests.
- composition with a stated precondition/effect: **would require
  source-specific logic** because current composition candidates are hard-coded
  and cannot reason over graph topology or incremental state.

All outcomes are hypothetical; no CLI test was executed.

## 8. Importer conclusion

- Recommended normalization: retain two problem/algorithm identities while
  documenting their common breadth-first strategy.
- Unresolved questions: whether algorithm identity should be independent of
  output materialization; how conditional determinism and lazy state should
  become selectable without graph-specific fields.
- Decision-relevant loss: collapsing the outputs changes substitution and
  composition; free-text topology requirements cannot qualify candidates.
- Documentary divergence that should remain acceptable: `stack` versus queue,
  node/vertex vocabulary, Rust traits versus integer-indexed Java graph, and
  exact result-object layout.
- Proposed next minimal experiment: ask for (1) reachable-node streaming and
  (2) all shortest witness paths over the same disconnected graph and verify
  that candidate selection distinguishes them without API-name rules.
- Schema or AST change requested now: none.
