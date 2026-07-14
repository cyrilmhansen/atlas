# K-M0 independent-import comparison

Status: completed pilot comparison  
Compared: 2026-07-14  
Input protocol: `k-m0.1`  
Resulting protocol: `k-m0.2`

## Experiment

Two importers independently analyzed the same named subjects: BFS, Dijkstra and
union-find. Both received the K-M0 source selection, worksheet, schema 0.1 and
pre-import baseline. They were isolated from each other's files until all six
worksheets were complete.

Raw submissions are preserved under `docs/phase2/imports/importer-a/` and
`docs/phase2/imports/importer-b/`. No raw worksheet was edited during
adjudication.

| Worksheet | SHA-256 |
|---|---|
| A / BFS | `f25e8028701441473dd5dabec8ae1f310cbe8b23136b9876c05943d5732bba2d` |
| A / Dijkstra | `e4430409eeadad2a05e38cc6899ca23f4b7c088fda6a515b7ff5afd5e2c00ded` |
| A / union-find | `3b7112783375f6c202b6296b3ab41dc8a39ca043a343c8c05b0226f4118b40c6` |
| B / BFS | `405d587e436f53e643c3db0777f303c6ed79ec0c62e83399cd0f272001b8cf35` |
| B / Dijkstra | `ff54775ab0fb9de6e46bd8734eec8f40cfd81a5a0b62fbd43f686bea73707069` |
| B / union-find | `c0f31931b56d56ec4a126885ec712ce8036370ac9cad11cd896528fd68a21b97` |

## Equivalence matrix

The labels follow `docs/phase2/import-equivalence-rubric.md`. They are not
combined into a score.

| Subject | Dimension | Outcome | Decision changed | Cause |
|---|---|---|---|---|
| BFS | identity | equivalent | none | both split incremental traversal from materialized shortest paths |
| BFS | semantic | compatible | none in the tested requests | B is more cautious about petgraph exact-once and ordering guarantees |
| BFS | taxonomic | equivalent | none | identifier spelling differs; boundaries do not |
| BFS | operational | equivalent | none | both reject traversal as a path-producing substitute |
| BFS | documentary | compatible | none | API types and failure spelling differ acceptably |
| Dijkstra | identity | divergent | substitution | A normalizes a common all-distances projection; B retains exact distance-map and path-tree contracts |
| Dijkstra | semantic | compatible | none | both preserve cost, numeric, partial-goal and witness differences |
| Dijkstra | taxonomic | divergent | lookup, substitution | projection identity versus source-bounded variant identity |
| Dijkstra | operational | divergent | selection, substitution | algs4 is eligible for A's distance projection but has no generic one-way relation in B's model |
| Dijkstra | documentary | compatible | none | cost/queue types and tie witnesses may remain source-local |
| union-find | identity | compatible | none after adjudication | both choose one persistent operation-sequence problem |
| union-find | semantic | equivalent | none | both preserve partition, growth, query mutation and amortization boundaries |
| union-find | taxonomic | unresolved in raw imports | potential substitution | A inspected linked source; B did not infer rank/path-halving from the API page |
| union-find | operational | equivalent | none | both distinguish fixed/growable and mutating/nonmutating lookup |
| union-find | documentary | compatible | none | roots, arrays, errors and capacity APIs need not normalize |

## Adjudication

### BFS

Both imports independently reject a single broad problem with optional outputs.
The accepted pilot interpretation is two provisional contracts:

- incremental reachable-node traversal from one source;
- materialized unweighted shortest paths from one or more sources.

They may share a breadth-first strategy relation later, but schema 0.1 cannot
represent one algorithm serving several output contracts. No public relation is
introduced now. Remaining exact-once and tie-order questions belong to later
executable fidelity, not a schema change.

### Dijkstra

This is the substantive divergence. Importer A treats the all-reachable distance
mapping as a common projection: petgraph with no goal produces it directly,
while algs4's shortest-path tree contains it. Importer B keeps the source outputs
as two exact problems because Atlas cannot state that a richer path tree supplies
a distance-only request.

Neither normalization is dismissed. A is operationally useful but requires an
explicit projection or entailment fact that schema 0.1 lacks. B preserves source
boundaries but would reject a valid candidate if exact problem identity were the
only substitution mechanism. The provisional conclusion is:

- retain exact source contracts during import;
- record the all-distances projection experimentally;
- test the same enriched-output relation in streaming before proposing a public
  capability or projection relation.

### Union-find

Both imports reject stateless per-operation problems and normalize a persistent
operation sequence. They agree that exact representative IDs are not portable
partition semantics, and that growth, count/label output, checked failure and
representation-only mutation affect eligibility.

Importer A followed the rustdoc source link and found petgraph's rank balancing
and path halving. Importer B deliberately did not infer these from the API page.
This is compatible source coverage after adjudication, but exposes a protocol
defect: optional supplemental pages make the effective packet unequal. Protocol
`k-m0.2` separates mandatory and supplemental evidence.

## Minimal discriminating requests

These are manual contract evaluations, not executed Atlas CLI tests.

### BFS

| Request | petgraph `Bfs` | algs4 `BreadthFirstPaths` | Discriminant |
|---|---|---|---|
| consume reachable vertices incrementally | eligible | not exact | incremental state/output |
| obtain all minimum edge counts | not eligible | eligible | distance output |
| reconstruct one shortest path per reachable vertex | not eligible | eligible | witness output |
| accept several sources in one run | not eligible as documented | eligible | source cardinality |

### Dijkstra

| Request | petgraph `dijkstra` | algs4 `DijkstraSP` | Discriminant |
|---|---|---|---|
| all reachable distances | eligible with no goal | semantically eligible by projection | projection relation |
| stop after one goal cost is settled | eligible | not exact | early termination |
| reconstruct paths to all reachable vertices | not eligible | eligible | witness tree |
| satisfy the source's exact-integer premise | depends on selected `Measure` | eligible within algs4's stated bound | arithmetic domain |

The first row is the decision-changing A/B case. Exact source contracts alone
reject algs4; a verified output projection accepts it.

### Union-find

| Request | petgraph `UnionFind` | algs4 `UF` | Discriminant |
|---|---|---|---|
| add elements after initialization | eligible | not eligible | growable state |
| query without representation mutation | eligible with `find`/`equiv` | not eligible with `find` | physical mutation |
| obtain maintained component count directly | not eligible | eligible | output capability |
| report invalid index without panic/exception | eligible with checked API | not eligible | failure contract |

## Model findings

The following gaps recur, but so far only within this graph/dynamic-structure
pilot:

- one strategy serving related output contracts;
- typed output capabilities and one-way projections;
- conditional determinism tied to traversal or tie order;
- operation- or state-indexed complexity and effects.

Persistent state and amortized sequences are demonstrated by union-find only.
Numeric validity is demonstrated by Dijkstra only. Neither can justify a public
field under DEC-066. AST failure is complete for all three subjects and remains
non-blocking; the sequence AST is not extended.

## Effort evidence

Importer B recorded a coherent batch timeline and per-worksheet estimates.
Importer A recorded per-worksheet estimates whose sum exceeds the observable
single concurrent batch interval. Raw values are preserved but are not
comparable, and no aggregate authoring-cost conclusion is accepted.

This invalidates the self-estimated effort portion of `k-m0.1`; it does not
invalidate the semantic results. Protocol `k-m0.2` records one externally
observed batch start/end and permits `unavailable` for activity subdivisions.

## Protocol revision

The one allowed post-pilot revision makes three changes:

1. each mandatory source page is enumerated and shared identically;
2. supplemental pages and facts are isolated from mandatory-source comparison;
3. elapsed effort is observed at batch level; retrospective active-minute
   estimates cannot support conclusions.

The revision is frozen for the first corpus batches. It adds no importer,
metric service, schema field or agent API.

## K-M0 result

K-M0 is **mixed but informative**:

- six isolated worksheets and the five-dimensional matrix are complete;
- BFS and union-find show strong decision-level convergence;
- Dijkstra exposes one real normalization divergence;
- source-packet and effort defects are identified and corrected once;
- no public schema, AST, registry entity or runtime change was made;
- effort is unavailable rather than presented as a valid measure.

The next milestone is K-M1. It should import exact graph source contracts and
experimental projection notes before any manifest entity is accepted. The first
non-graph batch must then test whether output projections and conditional
contracts recur.
