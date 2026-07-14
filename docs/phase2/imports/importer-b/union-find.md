# K-M0 importer B worksheet: union-find

Protocol revision: `k-m0.1`

## 1. Work record

- Importer identifier: `importer-b`
- Start and end time: 2026-07-14, approximately 23:17-23:27 Europe/Paris
- Active authoring minutes: approximately 6
- Source-reading minutes: approximately 2
- Atlas-modeling minutes: approximately 2
- Human interventions and their duration: none
- Tools used beyond browser, editor and existing Atlas CLI: web search/open tool
  for pinned source pages; no Atlas CLI command was needed

## 2. Source identity

- Source subject and source-local name: disjoint-set / union-find data structure;
  `UnionFind<K>` in petgraph, `UF` in algs4
- Authors or maintainers: petgraph maintainers; Robert Sedgewick and Kevin Wayne
- Work or project title: petgraph; *Algorithms, 4th Edition* and `algs4.jar`
- Edition, release, tag or commit: petgraph `0.8.3`; *Algorithms, 4th Edition*,
  2011
- Section, page, module or symbol: `petgraph::unionfind::UnionFind`;
  `edu.princeton.cs.algs4.UF`, Section 1.5
- Stable URLs:
  - <https://docs.rs/petgraph/0.8.3/petgraph/unionfind/struct.UnionFind.html>
  - <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/UF.java.html>
- Retrieval date: 2026-07-14
- Source class: library and book-associated educational library
- Code license: petgraph `MIT OR Apache-2.0`; algs4 GPLv3 or later
- Documentation license or copyright status: petgraph rustdoc accompanies the
  dual-licensed crate; algs4 page copyright Robert Sedgewick and Kevin Wayne
- Additional pages consulted: none

## 3. Source-faithful account

### petgraph 0.8.3

- Problem stated by the source: maintain membership of disjoint sets containing
  elements indexed `0..n-1` with an unsigned scalar index type
  (`UnionFind<K>`, main description).
- Inputs and their representation: initial size or capacity, unsigned `IndexType`
  element IDs, and pairs of IDs for equivalence/union operations
  (`new`, `with_capacity`, `find`, `equiv`, `union`).
- Preconditions and validity domain: indexes must be in bounds for unchecked
  forms; out-of-bounds `find`, `find_mut`, `equiv`, and `union` panic. Checked
  `try_` forms return `Option` or `Result`, with the documented special case
  that `try_union(x, x)` returns `Ok(false)` even for an out-of-bounds index
  (`find` through `try_union`). Capacity growth may fail or overflow
  (`try_reserve`, `reserve`).
- Output or observable interface: representatives from immutable or compressing
  find, checked variants, membership equivalence, boolean indication whether
  union changed the partition, length/capacity, dynamic `new_set`, and complete
  representative labeling (`UnionFind<K>`, methods).
- Postconditions and guarantees: `new(n)` creates n disjoint sets; `new_set`
  appends one singleton; `union` unifies two sets and reports false if already
  identical; `equiv` reports same-set membership; `into_labeling` maps elements
  to representatives (corresponding method descriptions).
- Strategy: disjoint-set structure. The page explicitly says `find_mut` writes
  back the found representative, flattening internal structure. The weighting
  rule used by union is not stated on the API page.
- Named invariants: elements are indexed consecutively and each tracked element
  belongs to a disjoint set. Internal representation invariants are not stated.
- Persistent and temporary state: persistent private forest-like state is
  implied; `find_mut` flattens it. Length and allocation capacity are separately
  observable. Exact arrays are not documented on the API page.
- Mutations, allocation, I/O and failure behavior: constructors and growth
  allocate; `new_set`, `union`, `find_mut`, reserves and shrinking mutate state;
  `find` and `equiv` take immutable access. Panicking and checked APIs are both
  supplied. No I/O is stated.
- Time claims, including operation and case being bounded: amortized O(alpha(n))
  per operation is stated in the type description; `new_set` amortized O(1).
  Per-operation worst cases and the operation set covered by the broad
  amortized claim are not precisely stated.
- Space claims, including what is excluded: not stated. Capacity APIs expose
  possible over-allocation.
- Determinism, randomness or numerical assumptions: no randomness. Representative
  identity after unions is not promised, so semantic determinism concerns the
  partition rather than exact labels.
- Variants explicitly distinguished by the source: immutable `find` versus
  compressing `find_mut`; panicking versus checked `try_` operations; fixed-size
  `new(n)` versus growable `new_empty`/`new_set`; capacity-management variants.
- Ambiguities or internally inconsistent statements: the broad amortized bound
  does not say whether allocation-management and labeling operations are
  included. The exact representative-selection rule is not a public guarantee.

### Algorithms, 4th Edition

- Problem stated by the source: model a partition of n elements, each in exactly
  one set, supporting `find`, `union`, and `count` (and a deprecated connected
  query) (`UF.java`, lines 27-57).
- Inputs and their representation: fixed `n` elements named `0..n-1` and integer
  element pairs for operations (lines 33-52, 87-103, 105-162).
- Preconditions and validity domain: `n >= 0`; operation elements must be in
  `0..n-1`; invalid inputs throw `IllegalArgumentException` (lines 87-118,
  143-162).
- Output or observable interface: `find(p)` returns canonical element,
  `count()` number of sets, `connected(p,q)` (deprecated) compares finds, and
  `union(p,q)` mutates without returning whether it changed the partition
  (lines 104-162).
- Postconditions and guarantees: find returns the same canonical element iff
  two elements share a set; union replaces two distinct sets by their union;
  count decreases on a successful merge (lines 43-57, 151-162). The prose also
  claims the canonical element cannot change during `find` or `count` (lines
  55-57).
- Strategy: weighted quick-union by rank with path compression by halving
  (lines 59-60). `find` assigns each visited node's parent to its grandparent;
  `union` attaches lower-rank root to higher-rank root and increments rank on a
  tie (lines 111-117, 151-162).
- Named invariants: `parent[i]` is i's parent, `rank[i]` is root-subtree rank and
  never exceeds 31, `count` is component count (lines 81-85). Each element is in
  exactly one set (lines 33-40).
- Persistent and temporary state: fixed arrays `parent` and `rank` and integer
  `count` persist for the object's lifetime (lines 81-103).
- Mutations, allocation, I/O and failure behavior: constructor allocates and
  initializes arrays; `find` mutates parent pointers through compression despite
  being described as a query; `union` mutates parents/ranks/count. Invalid
  inputs throw. Core API has no I/O; demonstration `main` reads pairs and prints.
- Time claims, including operation and case being bounded: constructor Theta(n);
  union and find worst-case Theta(log n); count Theta(1); an intermixed sequence
  of m union/find operations from n singleton sites takes O(m alpha(n)) (lines
  61-70).
- Space claims, including what is excluded: not stated directly; two n-sized
  arrays are visible in the source.
- Determinism, randomness or numerical assumptions: no randomness. Exact roots
  follow rank/tie rules, but callers should rely on partition equivalence rather
  than stable representative identity.
- Variants explicitly distinguished by the source: QuickUnionUF, QuickFindUF,
  and WeightedQuickUnionUF are named as alternative implementations of the same
  API (lines 72-76); `connected` is deprecated in favor of two finds (lines
  127-140).
- Ambiguities or internally inconsistent statements: lines 55-57 state that a
  canonical element cannot change during `find`, yet `find` performs path
  compression. This is consistent only if compression preserves the root; it
  does here, but the distinction between semantic canonical identity and
  internal mutation is unstated. For `n=0`, `count()` returns 0, conflicting
  with its documented return range 1..n (lines 87-103, 119-125).

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason: `set.dynamic_disjoint_partition` as a
  persistent state-machine contract over operations create, find, equivalent,
  union, and count/label. Petgraph's growth capability and algs4's fixed universe
  should be contract variants, because a fixed-size implementation cannot
  substitute for a request requiring later insertion.
- Proposed `Algorithm` identity and reason: source-bounded variants
  `set.disjoint_forest_petgraph_0_8_3` and
  `set.weighted_quick_union_rank_path_halving`. A shared documentary family
  `disjoint-set forest with path compression` is plausible, but petgraph's union
  weighting rule is not stated on the API page and should not be inferred.
- Proposed implementation identity, if executable source is in scope: none;
  upstream code remains documentary-only during K-M0.
- Existing Atlas entity that may be synonymous: none.
- Proposed problem `input`, `requires`, `output`, `ensures`: input `initial
  element universe and an operation sequence`; requires `element IDs valid for
  each operation`; output `operation results plus final partition`; ensures
  `union monotonically coarsens the partition, find/equivalence identify current
  classes, and query-side representation changes do not change the partition`.
  Optional capabilities `grow`, `count`, checked failure, and labeling must stay
  documentary/experimental because schema 0.1 cannot type them.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  requires consecutively indexed elements in the source-specific integer domain.
  Semantic partition evolution is deterministic for an operation sequence;
  exact representatives are not a portable output guarantee. Preserve algs4
  constructor Theta(n), per-find/union worst Theta(log n), count Theta(1), and
  sequence O(m alpha(n)) separately. Preserve petgraph's declared amortized
  O(alpha(n)) per operation and `new_set` amortized O(1) without widening their
  scope. Memory is documentary only.
- Proposed implementation effects and tests: distinguish semantic state mutation
  (`union`, growth) from representation-only mutation (`find_mut` or algs4
  `find`). Later tests should compare partitions rather than roots, include n=0,
  repeated/self unions, invalid indexes, checked/unchecked behavior, growth,
  interleaved find/union, labeling, count, and allocation failure boundaries.
- Proposed evidence level for each claim and why: source API/complexity claims
  `declared`; visible strategies `declared`; later operation-sequence oracle
  comparisons `tested`; no claims `proven`.
- Candidate relationships to other imported subjects: union-find could supply
  incremental connectivity but does not replace BFS reachability for arbitrary
  graph queries without a separately maintained edge-update protocol.
- Information intentionally left documentary: concrete arrays and rank byte,
  exact root/tie identity, panic/exception classes, allocator capacity controls,
  the petgraph `try_union(x,x)` invalid-index special case, and alternative algs4
  implementations.

Alternative normalization: treat each operation as an independent Atlas
problem. I reject it for comparison because it loses persistent state,
interleaved amortization, and representation invariants, making composition
appear valid without an owned partition state.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: source type names, petgraph release, book
  edition, class source and Section 1.5 are retained.
- Missing or unstable source identity: algs4 lacks a commit identifier;
  petgraph release docs are not tied here to a repository commit.
- Assessment: partial.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: operation-sequence
  semantics, path compression distinction, algs4 rank weighting, index domains,
  growth difference, and complexity scopes are retained.
- Semantic details lost or altered: petgraph union weighting is unresolved;
  exact representatives are deliberately excluded from cross-source guarantees.
- Assessment: partial, because Atlas schema 0.1 cannot represent a persistent
  multi-operation contract structurally.

### Representational fidelity

- Source vocabulary and decomposition retained: petgraph immutable/compressing
  APIs and algs4 fixed object API remain distinct.
- Normalized or collapsed concepts: Rust checked/panicking methods and Java
  exceptions are reduced to documentary failure modes; roots are normalized to
  partition semantics.
- Assessment: lossy but decision-relevant losses are recorded.

### Executable fidelity

- Upstream implementation or examples available: yes under source licenses.
- Correction oracle proposed: replay identical valid operation sequences and
  compare equivalence relations/count after every operation, never exact root
  labels; separately check source-specific growth and failure APIs.
- Behavior actually checked during this worksheet: none in K-M0.
- Assessment: not assessed.

### Declared transformations

- translation: Rust/Java stateful APIs to an independently worded operation
  sequence contract.
- specialization or generalization: common partition semantics generalized;
  fixed/growable and checked/panicking capabilities remain source-local.
- type or representation adaptation: unsigned generic and Java integer indexes
  become bounded element identifiers.
- API decomposition or aggregation: all operations aggregate into one stateful
  problem rather than false independent stateless problems.
- bug correction: none; n=0 count documentation discrepancy is reported only.
- pedagogical simplification: exact root shape and rank representation are not
  portable output guarantees.
- other: representation-only mutation is distinguished from semantic partition
  mutation.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Persistent operation sequence | problem input/output prose | lossy | identity / composition | experimental annotation |
| Fixed versus growable universe | problem requires prose | lossy | selection / substitution | experimental annotation |
| Query can mutate representation only | implementation effects | ambiguous | selection / composition | worksheet |
| Per-operation worst and sequence amortized costs | time claims | absent | selection | experimental annotation |
| Partition equality versus representative identity | ensures prose | lossy | substitution / testing | worksheet |
| Checked versus panicking failures | effects/tests prose | lossy | selection / composition | worksheet |
| Count and labeling capabilities | output prose | lossy | selection / substitution | experimental annotation |
| Capacity/allocation controls | implementation effects | lossy | selection | worksheet |
| Family and alternative implementations | no relation field | absent | identity / substitution | experimental annotation |
| Parent forest and mutable object state in AST | private AST | absent | documentary only | worksheet |

Independent-operation modeling could select operations that cannot share state
or accept a fixed universe where growth is required. Treating any mutation as a
semantic effect could reject valid substitution of compressing find; ignoring it
could violate composition assumptions about physical immutability. Stateful
contracts recur in streaming algorithms and hash tables, so one such family is
required before a schema proposal; amortized operation-sequence costs should
also be tested there.

## 7. Operational probes

- identity lookup: unsupported; no stateful data-structure or source alias
  entities exist.
- search by the proposed vocabulary: unsupported until records exist; lexical
  search cannot normalize union-find/disjoint-set/quick-union terminology.
- qualification by current properties: would require source-specific logic for
  growth, checked failure, count/label capability, and per-operation costs.
- substitution between source variants: unsupported; fixed/growable and
  immutable/compressing lookup capabilities are not structural.
- composition with a stated precondition/effect: would require source-specific
  logic to thread persistent partition ownership and distinguish semantic from
  representation-only mutation.

All outcomes are hypothetical; no CLI probe was executed.

## 8. Importer conclusion

- Recommended normalization: one stateful disjoint-partition problem family
  with explicit source-bounded variants retained experimentally; do not split
  operations into stateless problems and do not promise exact root identity.
- Unresolved questions: petgraph's public union weighting guarantee; scope of its
  broad amortized claim; eventual structural representation of persistent state
  and representation-only mutation.
- Decision-relevant loss: growth capability, state threading, amortized sequence
  bounds, and query mutation all alter selection or composition.
- Documentary divergence that should remain acceptable: exact root labels,
  array/rank layout, allocator APIs, and language-specific error mechanisms.
- Proposed next minimal experiment: manually qualify both source contracts
  against fixed versus growing operation sequences, pure-query requirements,
  checked failures, and count/label outputs; compare partitions after each step.
- Schema or AST change requested now: none.
