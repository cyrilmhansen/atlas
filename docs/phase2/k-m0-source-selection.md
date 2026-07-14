# K-M0 source selection

Status: working selection for the Phase 2 pilot  
Recorded: 2026-07-14  
Scope: BFS, Dijkstra and union-find only

This document fixes the material presented to import authors. It does not add a
dependency, import upstream code, or make a source authoritative over Atlas.
The selection is intentionally reversible after the pilot.

## Selected sources

### petgraph 0.8.3

Source class: established Rust library.

- Project: `petgraph`, release `0.8.3`.
- Repository: <https://github.com/petgraph/petgraph>
- Release documentation: <https://docs.rs/petgraph/0.8.3/petgraph/>
- BFS: <https://docs.rs/petgraph/0.8.3/petgraph/visit/struct.Bfs.html>
- Dijkstra: <https://docs.rs/petgraph/0.8.3/petgraph/algo/dijkstra/fn.dijkstra.html>
- Union-find:
  <https://docs.rs/petgraph/0.8.3/petgraph/unionfind/struct.UnionFind.html>
- License: `MIT OR Apache-2.0`.
- Retrieval date: 2026-07-14.

The release number, rather than the moving default branch, is the pilot pin.
No Cargo dependency is added during K-M0. A later executable-fidelity test may
use the released crate only after its test boundary is proposed.

### Algorithms, 4th Edition booksite

Source class: independently maintained educational reference and Java library.

- Authors: Robert Sedgewick and Kevin Wayne.
- Work: *Algorithms, 4th Edition*, Addison-Wesley, 2011,
  ISBN `0-321-57351-X`.
- Code index: <https://algs4.cs.princeton.edu/code/>
- BFS:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/BreadthFirstPaths.java.html>
- Dijkstra:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/DijkstraSP.java.html>
- Union-find:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/UF.java.html>
- Code license: GPLv3 or later; the code index describes `algs4.jar` as GPLv3.
- Retrieval date: 2026-07-14.

Atlas may cite and independently describe these materials. GPL code is not
copied, translated line by line, linked into the MIT workspace, or used as an
Atlas implementation in this pilot. Book prose and diagrams are not copied.
Source examples may inform independently authored correction cases when their
origin and transformation are recorded.

## Why this pair

The pair covers the same three named subjects but exposes different boundaries:

| Subject | petgraph 0.8.3 | Algorithms, 4th Edition | Pressure on Atlas |
|---|---|---|---|
| BFS | Incremental nonrecursive walker yielding reachable nodes | Eager path object with reachability, distance and predecessor queries | Is traversal the same problem as shortest unweighted paths? |
| Dijkstra | Function returning reachable-node costs, with optional early goal | Object retaining distances and a shortest-path tree | Are output shape and early termination variants or different problems? |
| Union-find | Growable generic structure; distinct immutable and compressing lookup APIs | Fixed-size weighted union by rank with path compression by halving | How are persistent state, amortization and mutating queries represented? |

These differences are decision-relevant before syntax or language differences.
They prevent the pilot from manufacturing agreement by presenting two sources
with identical API decomposition.

## Source packet shown to each importer

Each independent importer receives these mandatory pages:

1. the six exact algorithm URLs listed under the two selected sources;
2. the two code/license index URLs listed above;
3. this source-selection document;
4. `docs/phase2/import-worksheet.md` without another importer's answers;
5. schema 0.1 and the current CLI documentation;
6. `docs/phase2/current-model-baseline.md`;
7. the instruction to report loss rather than extend the schema or AST.

The initial `k-m0.1` pilot allowed importers to follow directly linked pages.
That produced unequal effective source coverage for petgraph union-find. Under
`k-m0.2`, a followed page is supplemental: its URL and every fact it adds are
recorded separately and adjudicated only after both raw submissions are frozen.

Importers may not consult an existing Atlas draft for these cases.

## Licensing boundary

K-M0 produces documentary analysis only. The later corpus batch uses these
rules:

- bibliographic facts, independently worded contracts and citations may enter
  Atlas;
- compatible upstream code requires its own implementation provenance and
  license review;
- GPL algs4 code remains external reference material unless a later decision
  defines an isolated, license-compatible test arrangement;
- absence of a clear license blocks code reuse, not study of published facts;
- source transformations must never be hidden by an Atlas-native identifier.

## Initial hypotheses, not conclusions

- A single Atlas `Algorithm` identity may be too coarse for BFS traversal versus
  breadth-first path materialization.
- Dijkstra's numerical validity conditions may not fit cleanly in the current
  free-text requirement and complexity claims.
- Union-find may expose the absence of persistent-state and amortized-operation
  contracts.
- The current AST is not expected to represent any of the three faithfully.

The import experiment must preserve or refute these hypotheses. They are not
permission to change the public model.
