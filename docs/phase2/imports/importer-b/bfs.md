# K-M0 importer B worksheet: BFS

Protocol revision: `k-m0.1`

## 1. Work record

- Importer identifier: `importer-b`
- Start and end time: 2026-07-14, approximately 22:55-23:07 Europe/Paris
- Active authoring minutes: approximately 7
- Source-reading minutes: approximately 3
- Atlas-modeling minutes: approximately 2
- Human interventions and their duration: none
- Tools used beyond browser, editor and existing Atlas CLI: web search/open tool to
  retrieve the pinned public pages; no Atlas CLI command was needed

## 2. Source identity

- Source subject and source-local name: breadth-first search; `Bfs` in petgraph,
  `BreadthFirstPaths` in algs4
- Authors or maintainers: petgraph maintainers; Robert Sedgewick and Kevin Wayne
- Work or project title: petgraph; *Algorithms, 4th Edition* and `algs4.jar`
- Edition, release, tag or commit: petgraph `0.8.3`; *Algorithms, 4th Edition*,
  2011, current source page marked updated 2026-01-10
- Section, page, module or symbol: `petgraph::visit::Bfs`;
  `edu.princeton.cs.algs4.BreadthFirstPaths`, Section 4.1
- Stable URLs:
  - <https://docs.rs/petgraph/0.8.3/petgraph/visit/struct.Bfs.html>
  - <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/BreadthFirstPaths.java.html>
- Retrieval date: 2026-07-14
- Source class: library and book-associated educational library
- Code license: petgraph `MIT OR Apache-2.0`; algs4 GPLv3 or later
- Documentation license or copyright status: petgraph rustdoc accompanies the
  dual-licensed crate; algs4 page copyright Robert Sedgewick and Kevin Wayne
- Additional pages consulted: none

## 3. Source-faithful account

### petgraph 0.8.3

- Problem stated by the source: traverse, in breadth-first order, the nodes
  reachable from one starting node. The source calls this "A breadth first
  search (BFS) of a graph" and defines the reachability boundary
  (`petgraph::visit::Bfs`, main description).
- Inputs and their representation: a graph satisfying `GraphRef + Visitable`
  when creating the walker, a start `NodeId`, then a graph satisfying
  `IntoNeighbors` on each `next` call (`Bfs::new`, `Bfs::next`).
- Preconditions and validity domain: the start must be usable by the graph's
  visitor map. Removing nodes during iteration may make behavior incorrect;
  nodes or edges added during iteration are not guaranteed to be visited
  (`Bfs`, mutation note).
- Output or observable interface: each `next` call yields `Option<NodeId>`;
  `None` means traversal is complete (`Bfs::next`). It does not return paths or
  distances.
- Postconditions and guarantees: only nodes reachable from the start are
  traversed (`Bfs`, main description). A stronger ordering guarantee between
  nodes at equal depth is not stated.
- Strategy: nonrecursive breadth-first traversal, with a `VecDeque` named
  `stack` as the queue of nodes to visit and a visitor map named `discovered`
  (`Bfs`, description and public fields).
- Named invariants: not stated.
- Persistent and temporary state: walker state persists across calls in
  `stack: VecDeque<N>` and `discovered: VM`; the walker does not retain a graph
  borrow (`Bfs`, fields and description).
- Mutations, allocation, I/O and failure behavior: `next` mutates walker state;
  allocation behavior and failure behavior are not stated. The graph can be
  mutably accessed between steps because it is not borrowed by the walker,
  subject to the mutation caveat above. No I/O is stated.
- Time claims, including operation and case being bounded: not stated.
- Space claims, including what is excluded: not stated.
- Determinism, randomness or numerical assumptions: no randomness or numerical
  assumptions are stated. Equal-depth visitation order depends on neighbor
  iteration order; determinism is therefore not established by this page.
- Variants explicitly distinguished by the source: `Bfs` can be used through
  `new`/`next` or the `Walker` interface; no multi-source constructor is stated.
- Ambiguities or internally inconsistent statements: the public queue field is
  named `stack`, but is documented as a queue; this is terminological, not a
  behavioral inconsistency.

### Algorithms, 4th Edition

- Problem stated by the source: find shortest paths by number of edges from one
  source or a set of sources to every vertex in an undirected graph
  (`BreadthFirstPaths.java`, lines 44-49, 69-100).
- Inputs and their representation: an adjacency-list-style `Graph` whose
  vertices are integers `0..V-1`, plus either one integer source or a nonempty
  iterable of integer sources (lines 69-100, 110-139).
- Preconditions and validity domain: the single source must lie in `0..V-1`.
  The multi-source iterable must be non-null, nonempty, contain no null, and all
  vertices must be valid (lines 69-100, 222-244).
- Output or observable interface: after eager construction, `hasPathTo(v)`
  reports reachability, `distTo(v)` returns shortest edge count or
  `Integer.MAX_VALUE`, and `pathTo(v)` returns a vertex sequence or `null`
  (lines 142-179).
- Postconditions and guarantees: every reachable vertex has a shortest path in
  number of edges; the internal check verifies source distance zero, edge
  feasibility, and that each predecessor edge advances distance by one (lines
  181-219).
- Strategy: enqueue source vertices, mark them and set distance zero; repeatedly
  dequeue a vertex, scan its adjacency list, and discover each unmarked neighbor
  while recording predecessor and distance (lines 102-140).
- Named invariants: `marked[v]` means an s-v path exists, `edgeTo[v]` records the
  preceding vertex on a shortest path, and `distTo[v]` is the shortest edge
  count (lines 64-67). The source does not give these a collective invariant
  name.
- Persistent and temporary state: persistent arrays `marked`, `edgeTo`, and
  `distTo`; a temporary FIFO `Queue<Integer>` during construction (lines
  64-67, 102-140).
- Mutations, allocation, I/O and failure behavior: construction allocates three
  V-sized arrays and a queue and mutates them. Query methods validate vertices
  and may throw `IllegalArgumentException`. Core construction has no I/O; the
  demonstration `main` reads a graph and prints results (lines 75-100,
  148-179, 222-272).
- Time claims, including operation and case being bounded: constructor worst
  case Theta(V + E); each instance method Theta(1) according to the class
  documentation (lines 49-54). The latter statement is ambiguous for
  `pathTo(v)`, which explicitly builds a sequence proportional to path length
  (lines 164-179).
- Space claims, including what is excluded: Theta(V) extra space excluding the
  graph (lines 49-54).
- Determinism, randomness or numerical assumptions: no randomness or numerical
  assumptions. Which shortest predecessor is selected depends on adjacency and
  source iteration order; a unique path is not guaranteed.
- Variants explicitly distinguished by the source: single-source and
  multi-source constructors (lines 69-100).
- Ambiguities or internally inconsistent statements: the blanket Theta(1)
  instance-method claim conflicts with the explicit traversal and allocation in
  `pathTo`; this should not be normalized as a proven query bound.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason: `graph.reachable_traversal_from_source`
  for petgraph's observable contract, and
  `graph.unweighted_shortest_paths_from_sources` for algs4. They must remain
  separate because one yields an incremental traversal while the other
  materializes reachability, distances, and reconstructible paths; substituting
  the former for the latter loses required outputs.
- Proposed `Algorithm` identity and reason: one conceptual
  `graph.breadth_first_search` strategy may relate to both problems, but schema
  0.1 permits only one `solves` reference. For comparison I would use two
  source-bounded algorithm records, `graph.bfs_traversal` and
  `graph.bfs_shortest_paths`, with a documentary "same breadth-first discovery
  strategy" relation rather than claim identity.
- Proposed implementation identity, if executable source is in scope: none;
  upstream code is documentary-only in K-M0.
- Existing Atlas entity that may be synonymous: none.
- Proposed problem `input`, `requires`, `output`, `ensures`:
  - traversal: input `graph and one start node`; requires `start is a valid graph
    node; node set is not removed during traversal`; output `incremental stream
    of reachable node identifiers`; ensures `each yielded node is reachable and
    traversal terminates after the reachable region is exhausted`. Exact-once
    visitation is strongly implied by `discovered` but not expressly stated, so
    keep it documentary pending executable verification.
  - paths: input `undirected graph and one valid source or nonempty valid source
    set`; output `per-vertex reachability, minimum edge count, and one
    reconstructible shortest path`; ensures `distance is minimum edge count and
    absent vertices are reported unreachable`.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  traversal has no imported complexity claim and determinism is `uncertain`;
  paths requires valid sources, is deterministic only relative to graph/source
  iteration order, takes Theta(V + E) construction time and Theta(V) auxiliary
  memory excluding graph. Do not import algs4's blanket Theta(1) query claim for
  path reconstruction.
- Proposed implementation effects and tests: documentary only. Later probes
  should test disconnected graphs, cycles, self-loops, duplicate edges,
  single/multiple sources, invalid sources, equal-length path ties, and mutation
  of petgraph node weights versus removal during walking.
- Proposed evidence level for each claim and why: source contracts and
  complexities `declared`; algorithmic strategy `declared`; any correction
  result later `tested`; no `proven` claims.
- Candidate relationships to other imported subjects: unweighted shortest
  paths are a specialization of shortest paths and can be compared with
  Dijkstra using unit nonnegative weights. BFS traversal is an enabling strategy
  but not a substitute for path materialization.
- Information intentionally left documentary: graph trait bounds, vertex
  exception types, concrete arrays/queue, tie ordering, source-local API shape,
  and the mutation caveat.

Alternative normalization: collapse both under one problem with an output mode.
I reject it for the comparison because schema 0.1 has no typed output mode and
would allow a traversal-only candidate to qualify for a path-producing request.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: both source-local type names, pinned
  petgraph release, book edition, class source and section are recorded.
- Missing or unstable source identity: algs4 page has an update date but no
  commit identifier; petgraph docs URL is release-pinned but not commit-pinned.
- Assessment: partial.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: FIFO discovery,
  discovered/marked state, reachability boundary, source validation, and algs4
  predecessor/distance guarantees are retained separately.
- Semantic details lost or altered: equal-depth order and exact graph mutation
  semantics remain documentary; exact-once visitation is not elevated beyond
  what the source states.
- Assessment: preserved when represented as two contracts; incompatible if
  collapsed as one substitutable algorithm entity.

### Representational fidelity

- Source vocabulary and decomposition retained: `Bfs` walker and
  `BreadthFirstPaths` eager data type remain explicit.
- Normalized or collapsed concepts: Rust graph traits and Java object/query
  syntax become textual graph contracts.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: yes for both, under their
  respective licenses.
- Correction oracle proposed: compare reachable node set; for algs4 also compare
  distances and validate returned path edges and length.
- Behavior actually checked during this worksheet: none in K-M0.
- Assessment: not assessed.

### Declared transformations

- translation: Rust/Java API statements to independently worded Atlas claims.
- specialization or generalization: split generic BFS naming into traversal and
  unweighted shortest-path contracts.
- type or representation adaptation: graph traits and integer vertices become
  documentary graph/node types.
- API decomposition or aggregation: algs4 constructor plus queries aggregate to
  one materialized-output contract; petgraph remains incremental.
- bug correction: none.
- pedagogical simplification: concrete containers are not part of the problem
  contract.
- other: reject the algs4 blanket constant-time claim for path reconstruction as
  internally ambiguous rather than silently importing it.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Walker versus materialized paths | Separate Problem/Algorithm prose | ambiguous | identity / substitution | worksheet |
| One strategy solves two contracts | `Algorithm.solves` | absent | identity / composition | experimental annotation |
| Graph and node types | problem input/requires prose | lossy | selection / composition | worksheet |
| Multi-source variant | problem input/requires prose | lossy | selection / substitution | experimental annotation |
| Iteration-order-dependent tie result | determinism prose | lossy | substitution | worksheet |
| Mutation during incremental traversal | requirements/effects split across entities | ambiguous | selection / composition | worksheet |
| Reachability, distance and path output facets | problem output prose | lossy | selection / substitution | experimental annotation |
| Graph operations in AST | private AST | absent | documentary only | worksheet |

If traversal and materialization are collapsed, two non-substitutable candidates
become indistinguishable and an invalid traversal-only candidate could be
selected. Conversely, enforcing path materialization globally would reject a
valid low-state traversal. Typed graph contracts, output capabilities, and
multi-source cardinality could affect composition. These issues are graph-family
specific until another family tests incremental versus materialized outputs;
streaming top-k would be a useful second structural family before a schema
proposal.

## 7. Operational probes

- identity lookup: unsupported; no entities exist and source-local aliases are
  not modeled.
- search by the proposed vocabulary: unsupported until textual records exist;
  after import, free-text search could find words but not normalize aliases.
- qualification by current properties: would require source-specific logic for
  graph direction, source cardinality, incremental output, and path capability.
- substitution between source variants: unsupported; current relations cannot
  state that materialized shortest paths provide traversal information but not
  vice versa.
- composition with a stated precondition/effect: would require source-specific
  logic; graph/node contracts and walker mutation restrictions are untyped.

All outcomes are hypothetical; no CLI probe was executed.

## 8. Importer conclusion

- Recommended normalization: two problem contracts and two source-bounded
  algorithm records, related documentarily by the breadth-first discovery
  strategy.
- Unresolved questions: whether exact-once and equal-depth ordering are public
  guarantees of petgraph; how Atlas should represent algorithms applicable to
  several output contracts.
- Decision-relevant loss: collapsing traversal and shortest-path materialization
  changes valid selection and substitution decisions.
- Documentary divergence that should remain acceptable: container names,
  language API shape, exception types, and chosen predecessor among ties.
- Proposed next minimal experiment: hand-evaluate four requests (visit reachable
  nodes, obtain distances, reconstruct paths, accept multiple sources) against
  both records and verify the expected acceptance matrix before any schema work.
- Schema or AST change requested now: none.
