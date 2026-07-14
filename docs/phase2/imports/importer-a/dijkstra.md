# K-M0 import A: Dijkstra shortest paths

Protocol: `k-m0.1`
Status: independent documentary import; no registry entity created

## 1. Work record

- Importer identifier: `importer-a`.
- Start and end time: 2026-07-14, one authoring session.
- Active authoring minutes: approximately 50 minutes for this worksheet.
- Source-reading minutes: approximately 25 minutes.
- Atlas-modeling minutes: approximately 25 minutes.
- Human interventions and their duration: source-pair selection was fixed before
  the import; no intervention during authoring.
- Tools used beyond browser, editor and existing Atlas CLI: `curl` and `pandoc`
  to inspect the pinned documentation and directly linked release source.

## 2. Source identity

### petgraph

- Source subject and source-local name: function `dijkstra`.
- Authors or maintainers: petgraph maintainers.
- Work or project title: `petgraph`.
- Edition, release, tag or commit: release `0.8.3`.
- Section, page, module or symbol: `petgraph::algo::dijkstra::dijkstra`.
- Stable URL:
  <https://docs.rs/petgraph/0.8.3/petgraph/algo/dijkstra/fn.dijkstra.html>.
- Retrieval date: 2026-07-14.
- Source class: library.
- Code license: `MIT OR Apache-2.0`.
- Documentation license or copyright status: distributed with the crate under
  the crate's dual license.
- Additional page consulted: source linked by rustdoc,
  <https://docs.rs/petgraph/0.8.3/src/petgraph/algo/dijkstra.rs.html#13-101>.

### Algorithms, 4th Edition

- Source subject and source-local name: `DijkstraSP`.
- Authors or maintainers: Robert Sedgewick and Kevin Wayne.
- Work or project title: *Algorithms, 4th Edition* and `algs4.jar`.
- Edition, release, tag or commit: fourth edition; source page updated
  2026-01-10.
- Section, page, module or symbol: Section 4.4; class
  `edu.princeton.cs.algs4.DijkstraSP`.
- Stable URL:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/DijkstraSP.java.html>.
- Retrieval date: 2026-07-14.
- Source class: book and educational library.
- Code license: GPLv3 or later.
- Documentation license or copyright status: copyrighted booksite material;
  only independently worded facts are recorded here.
- Additional pages consulted: none.

## 3. Source-faithful account

### petgraph account

- Problem stated by the source: compute the length of a shortest path from a
  start node to every reachable node, optionally stopping once one goal cost is
  calculated (rustdoc description).
- Inputs and their representation: a weighted graph implementing `IntoEdges`
  and `Visitable`, a start node ID, an optional goal node ID, and a mutable edge
  cost callback returning a copyable `Measure` (`dijkstra` signature).
- Preconditions and validity domain: edge costs must be non-negative. The source
  does not state behavior for an invalid start/goal, overflow, non-total cost
  ordering or non-finite values (rustdoc description and bounds).
- Output or observable interface: a `hashbrown::HashMap<NodeId, K>` mapping
  reached node IDs to path costs; an unreachable node is absent. No predecessor
  or path witness is returned (`Returns` and example).
- Postconditions and guarantees: with no goal, costs are produced for every
  reachable node. With a goal, termination occurs once that goal's cost is
  calculated; the returned map then reflects the explored prefix rather than an
  all-reachable contract (rustdoc description).
- Strategy: maintain a visit map, score map and binary min-heap, pop the least
  scored node and relax outgoing edges; stale/already visited entries are
  skipped in the remaining source (source lines 178 onward, directly linked by
  the public function source).
- Named invariants: not stated by name. Settled nodes are tracked separately
  from tentative scores by the visit map.
- Persistent and temporary state: result score hash map; temporary visit map and
  binary heap. The graph and edge-cost callback are borrowed inputs.
- Mutations, allocation, I/O and failure behavior: allocates the hash map, visit
  map and heap; mutates them and invokes `FnMut` for edge costs. No I/O is stated.
  Invalid-node and arithmetic-failure behavior are not stated.
- Time claims, including operation and case being bounded:
  O((|V| + |E|) log |V|) (`Complexity`). The page does not qualify reachable
  subgraph versus whole input when a goal is supplied.
- Space claims, including what is excluded: auxiliary O(|V| + |E|), as stated
  by the page. It does not separately identify returned score storage or graph
  visitor-map representation.
- Determinism, randomness or numerical assumptions: no randomness. Cost type is
  generic; numerical and tie-order assumptions are not stated. Costs should be
  invariant even if equal-cost processing order varies, provided the unstated
  arithmetic/order assumptions hold.
- Variants explicitly distinguished by the source: all-reachable calculation
  (`goal=None`) and early termination at a specified goal. The source module also
  offers a dynamic-goal form, but that symbol is not part of this selected
  subject and is not normalized here.
- Ambiguities or internally inconsistent statements: the headline says every
  reachable node, but optional-goal mode intentionally stops earlier. The output
  contract must therefore be parameter-dependent.

### Algorithms, 4th Edition account

- Problem stated by the source: solve single-source shortest paths in an
  edge-weighted directed graph with non-negative weights (class documentation,
  lines 35-45).
- Inputs and their representation: `EdgeWeightedDigraph`, integer source in
  `0..V-1`, directed edges with `double` weights (constructor, lines 67-99).
- Preconditions and validity domain: every edge weight must be non-negative and
  the source must be valid. Correctness additionally assumes no floating-point
  rounding error or arithmetic overflow; the source gives integer weights and
  intermediate sums at most 2^52 as a sufficient condition and bounds sums by
  `V*C` (class documentation, lines 47-53; constructor lines 72-79).
- Output or observable interface: eager data type retaining distance and
  predecessor arrays; queries return distance, reachability and one iterable
  shortest path of directed edges. Unreachable distance is positive infinity
  and path is `null` (`distTo`, `hasPathTo`, `pathTo`, lines 110-149).
- Postconditions and guarantees: a shortest-path tree from the source to all
  reachable vertices. The executable checker requires source distance zero,
  unreachable/predecessor consistency, feasibility of every edge and tightness
  of every tree edge (lines 150-194).
- Strategy: binary indexed min-priority queue keyed by current distance; remove
  the minimum-distance vertex and relax outgoing edges, decreasing or inserting
  keys as needed (lines 88-108).
- Named invariants: the source calls the result a shortest-path tree. Its checker
  names feasibility and tightness conditions in comments (lines 150-194).
- Persistent and temporary state: persistent `distTo` and `edgeTo` arrays in the
  result object; priority queue retained as a field although empty after eager
  construction (lines 63-65 and 80-98).
- Mutations, allocation, I/O and failure behavior: allocates two `V` arrays and
  an indexed heap; mutates them during relaxation. The data type performs no I/O.
  Negative edges and invalid source/query vertices throw
  `IllegalArgumentException`.
- Time claims, including operation and case being bounded: constructor worst
  case Theta(E log V) using a binary heap; each instance method is claimed
  Theta(1) (class documentation lines 39-45). As with BFS, constructing the
  iterable returned by `pathTo` walks a predecessor chain, so the blanket query
  claim is questionable for that method.
- Space claims, including what is excluded: Theta(V) extra, excluding the
  weighted digraph (class documentation lines 43-45).
- Determinism, randomness or numerical assumptions: deterministic under a fixed
  graph/iteration/priority-queue behavior, but the specific witness among equal
  shortest paths is not guaranteed. The floating-point assumptions above are
  explicit.
- Variants explicitly distinguished by the source: this class is the directed,
  non-negative-weight, all-destinations, binary-heap variant. Other shortest-path
  classes are outside the selected subject.
- Ambiguities or internally inconsistent statements: blanket Theta(1) query
  claim versus the path reconstruction loop; exact floating-point equality is
  used by the optional assertion checker under the stated no-rounding premise.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason: use
  `graph.single_source_shortest_path_distances_nonnegative` as a common core for
  petgraph with `goal=None` and the distance projection of algs4. Define neither
  early-goal nor witness-path materialization as interchangeable with this core;
  record them as candidate related contracts until relations are modeled.
- Proposed `Algorithm` identity and reason: `graph.dijkstra_nonnegative` for the
  common settle-minimum-and-relax strategy. This comparison identity is valid
  only for all-reachable distance output; current schema cannot say that one
  source additionally materializes a shortest-path tree.
- Proposed implementation identity, if executable source is in scope: none in
  K-M0. Future candidates could be `rust.petgraph.dijkstra.0_8_3` specialized to
  `goal=None` and `java.algs4.dijkstra_sp`, subject to dependency and license
  boundaries.
- Existing Atlas entity that may be synonymous: none.
- Proposed problem `input`, `requires`, `output`, `ensures`: directed weighted
  graph and valid source; non-negative edge costs in a domain whose addition and
  ordering preserve path-cost comparisons; output a mapping from every reachable
  vertex to minimum path cost, excluding unreachable vertices or marking them
  explicitly. Ensures source cost zero and all output distances minimal.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  non-negative costs plus valid arithmetic/order domain; deterministic distance
  mapping, with witness/tie order deliberately unspecified. Source-specific
  declared complexities remain separate: petgraph O((V+E) log V), O(V+E)
  auxiliary; algs4 Theta(E log V), Theta(V) extra excluding graph.
- Proposed implementation effects and tests: allocate distance map/array,
  visited state, priority queue and, for algs4, predecessors. Tests should cover
  unreachable vertices, zero-weight edges, equal shortest paths, self-loops,
  parallel edges, negative-edge rejection, near-overflow integer sums and
  floating-point cases outside the exactness premise.
- Proposed evidence level for each claim and why: contracts, strategies and
  complexity are `declared` from the pinned source; future cross-source examples
  would be `tested`; mathematical correctness and complexity are not `proven`
  by this import.
- Candidate relationships to other imported subjects: unit edge costs relate
  this problem to BFS shortest distances; early-goal is a specialization; path
  witnesses are an enriched output. None is structurally expressible now.
- Information intentionally left documentary: source numeric type, queue type,
  optional-goal partial map, witness tie breaking, source-specific failure APIs,
  and conflicting complexity accounting conventions.

Alternative A, used above, compares the common all-reachable distance contract
and records richer outputs separately. Alternative B creates separate problems
for distances, witness paths and source-to-goal distance; this preserves
selection but forces multiple Atlas algorithm identities because `solves` is
singular. Alternative C uses one broad shortest-path problem with optional goal
and optional witness outputs; it is rejected for comparison because current
qualification could then select a result shape that does not satisfy the caller.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: crate release and symbol; authors, edition,
  section and Java class; URLs and licenses.
- Missing or unstable source identity: no separate petgraph commit recorded;
  algs4 source is page-dated rather than release-tagged.
- Assessment: preserved.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: non-negative weights,
  minimum-priority processing, relaxation, reachability and minimum distance.
- Semantic details lost or altered: comparison specializes petgraph to
  `goal=None` and projects algs4 to distances, intentionally excluding early
  stopping and predecessor witnesses from equivalence.
- Assessment: partial for the full APIs; preserved for the common projection.

### Representational fidelity

- Source vocabulary and decomposition retained: function/map versus eager
  shortest-path-tree object remain explicit in documentary fields.
- Normalized or collapsed concepts: generic `Measure` and Java `double` become a
  language-neutral valid cost domain; heap and storage types are implementation
  details.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: yes for both sources.
- Correction oracle proposed: independently enumerate simple paths on small
  graphs with a bounded exhaustive oracle; compare distance projections and
  separately validate algs4 witness edges.
- Behavior actually checked during this worksheet: none.
- Assessment: not assessed.

### Declared transformations

- translation: Rust and Java APIs to a common weighted-directed-graph contract.
- specialization or generalization: petgraph fixed to `goal=None`; algs4 output
  projected to distances for equivalence.
- type or representation adaptation: `Measure` and `double` normalized to an
  abstract non-negative additive ordered cost domain, while source-specific
  numeric restrictions remain documented.
- API decomposition or aggregation: early-goal and witness-path capabilities are
  separated from the common comparison.
- bug correction: none; query-cost tension is flagged.
- pedagogical simplification: none.
- other: no source code copied.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Generic versus floating-point cost domain | `Problem.input` / `requires` text | lossy | selection, substitution | experimental annotation |
| No rounding/overflow premise | requirement text | lossy | selection | worksheet |
| All-reachable versus early-goal output | `Problem.output` text | ambiguous | identity, substitution | experimental annotation |
| Costs-only versus predecessor witnesses | `Problem.output` text | ambiguous | selection, substitution, composition | experimental annotation |
| One strategy serves related output contracts | singular `Algorithm.solves` | absent | identity, composition | worksheet |
| Complexity accounting differs by source | algorithm complexity claims | ambiguous | selection | worksheet |
| Conditional witness determinism | `deterministic` boolean | lossy | selection | worksheet |
| Priority queue/map/graph semantics in AST | generic calls only | absent | documentary only | worksheet |

Cost-domain loss can select a numerically invalid implementation. Output-shape
loss can select petgraph where paths are requested, or reject its useful
distance projection where paths are irrelevant. These are not only graph issues:
numerical algorithms stress arithmetic domains, while streaming top-k stresses
partial versus materialized outputs. At least one such second family is needed
before a schema proposal. AST absence is not an import acceptance gate.

## 7. Operational probes

- identity lookup: **unsupported** before entities exist; exact normalized ID
  lookup would work after a manifest import, source-local aliases would not.
- search by the proposed vocabulary: **supported** for literal text only; not for
  a typed relation such as unit-weight Dijkstra equivalent to BFS.
- qualification by current properties: **unsupported** for numeric domain,
  early-goal behavior, witness availability and arithmetic exactness.
- substitution between source variants: **unsupported** generically; safe only
  after manually projecting both to the all-reachable distance contract.
- composition with a stated precondition/effect: **would require
  source-specific logic** for graph direction, non-negative costs, result shape
  and current hard-coded candidate sets.

All outcomes are hypothetical; no CLI test was executed.

## 8. Importer conclusion

- Recommended normalization: compare one common all-reachable distance problem
  and Dijkstra strategy, while refusing to treat early-goal and witness-producing
  APIs as substitutable capabilities.
- Unresolved questions: how to type arithmetic validity; how to relate enriched
  output contracts; whether returned storage is included in auxiliary space.
- Decision-relevant loss: numeric exactness and output shape both change valid
  selection; singular `solves` obscures one strategy across related contracts.
- Documentary divergence that should remain acceptable: map versus arrays,
  generic measure versus `double`, indexed versus ordinary heap, absence versus
  infinity for unreachable vertices.
- Proposed next minimal experiment: submit three requests over the same graph:
  all distances, one target cost with early stopping, and explicit witness path;
  verify distinct candidate eligibility and reject an inexact numeric-domain
  candidate near its stated arithmetic limit.
- Schema or AST change requested now: none.
