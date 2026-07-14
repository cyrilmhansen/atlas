# K-M1 graph corpus batch

Status: complete  
Recorded: 2026-07-14  
Protocol: `k-m0.2`

## Scope

K-M1 imports four exact graph contracts derived from the independent K-M0
worksheets:

| Problem | Algorithm | Source boundary |
|---|---|---|
| `graph.reachable_traversal` | `graph.bfs.traversal` | petgraph incremental walker from one source |
| `graph.unweighted_shortest_paths` | `graph.bfs.shortest_paths` | algs4 eager reachability, distances and paths from one or more sources |
| `graph.nonnegative_shortest_distances` | `graph.dijkstra.distances` | petgraph specialized to `goal = None` |
| `graph.nonnegative_shortest_path_tree` | `graph.dijkstra.shortest_path_tree` | algs4 eager distances and reconstructible directed paths |

The batch raises the authoritative corpus from 10/15/20 to 14 problems, 19
algorithms and 22 implementations. The two new implementation records are
petgraph 0.8.3 APIs tested through a thin integration adapter. No algs4 code is
copied or linked.

## Source transformations

### petgraph BFS

The registry keeps the incremental walker boundary. Neighbor iteration order is
part of the represented input, and graph topology must remain stable during the
walk. The adapter checks reachable-component membership on a cyclic component,
a disconnected component and an isolated source. It does not promote visit
order or a universal complexity theorem to tested evidence.

The selected source does not state BFS complexity on its API page. The registry
therefore marks O(V + E) time and O(V) memory as `inferred`, not `declared` or
`tested`.

### algs4 BFS

The registry keeps the eager multi-source undirected shortest-path contract.
Distances and one witness path are required outputs, so this algorithm is not
registered as another implementation of incremental traversal. Complexity
claims retain their source-declared Theta notation and exclusion boundary.

### petgraph Dijkstra

The upstream function accepts an optional goal. K-M1 registers the explicit
specialization `goal = None`, which returns distances for all reachable nodes.
This transformation is visible in the implementation entrypoint and CLI
explanation. The adapter checks a cheaper indirect path, zero source distance
and omission of an unreachable node.

The cost callback is `FnMut`; its captured state may be mutated. This conditional
effect is recorded rather than treating the callback as pure.

### algs4 Dijkstra

The registry preserves the floating-point, directed shortest-path-tree contract
and its no-rounding/no-overflow validity condition. It remains an algorithm
without a registered implementation because K-M1 does not copy or link GPLv3
algs4 code.

## Experimental projections

These relationships are review findings, not schema 0.1 entities or relations:

| Source contract | Requested contract | Conditions | Status |
|---|---|---|---|
| algs4 Dijkstra shortest-path tree | all reachable distances | discard predecessors; translate unreachable infinity to absence | plausible, unmodeled |
| algs4 BFS shortest paths | reachable-node set | discard distances and witnesses | plausible, but not incremental traversal |
| nonnegative shortest distances with unit costs | unweighted minimum edge counts | identical graph direction and source cardinality | plausible, unmodeled |

The first projection is the decision-changing K-M0 divergence. K-M1 retains
both exact contracts, so current generic lookup cannot use that projection. A
second structural family must demonstrate the same need before any public
capability or projection relation is proposed.

## Model-friction matrix

| Mismatch | Severity | Lossiness | Current consequence | Provisional location |
|---|---|---|---|---|
| exact output contract versus one-way projection | high | decision-changing | valid richer candidate is not generically substitutable | this report |
| one strategy serving related problems | high | decision-changing | separate algorithm identities are required by singular `solves` | this report |
| graph direction, source cardinality and topology | high | lossy prose | `qualify` cannot filter these requirements | problem claims |
| numerical validity domain | high | lossy prose | invalid arithmetic implementation cannot be generically rejected | problem requirements |
| conditional determinism from iteration/tie order | medium | lossy boolean | order is moved into input/requirements as a bounded workaround | algorithm requirements |
| callback may mutate captured state | medium | conditional effect flattened | effect is visible but not generically queryable | implementation effects |
| differing complexity accounting conventions | medium | preserved but incomparable | strings remain source-specific | algorithm claims |
| bibliographic/source identity | medium | documentary | local worksheets carry structured fidelity outside schema | K-M0 reports |
| graph operations absent from AST | low for Knowledge | complete | no executable representation or visualization | recorded only |

No mismatch is silently repaired. The high-severity items are protected by
separate exact problem identities, at the cost of rejecting some valid
substitutions.

## Current CLI behavior

- `search` finds exact imported IDs and both breadth-first declared names;
- `show` and `explain` retain requirements, evidence, allocation and callback
  mutation;
- `qualify` cannot express graph direction, source cardinality, numeric domain
  or output capability;
- composition remains scenario-specific and cannot discover graph candidates.

The last two limits are phase evidence. K-M1 does not add graph-specific CLI
flags or composer branches.

## External implementation boundary

`petgraph = 0.8.3` is an exact, default-feature-disabled dev-dependency of the
`atlas` crate. It does not enter `atlas-algorithms`, the shipped CLI dependency
set, MIR, WASM or the public schema. Cargo locks petgraph and its three required
transitive crates for deterministic tests.

The adapter is deliberately test-only. It verifies real upstream behavior but
does not present an Atlas rewrite as the external implementation.

## Acceptance

- source terminology and Atlas mappings are reviewable: complete;
- representation-dependent preconditions are explicit: complete;
- at least one external implementation uses a thin tested adapter: complete for
  petgraph BFS and Dijkstra;
- every observed schema/AST mismatch has severity and lossiness: complete;
- schema, AST, visual machine and composition scenarios unchanged: complete.

K-M1 closes on this evidence. K-M2 should import union-find and at least one heap
and collision-aware hash-table family. It must test persistent state and
amortized operation costs without reusing graph-specific projection vocabulary.
