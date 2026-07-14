# K-M0 import A: union-find

Protocol: `k-m0.1`
Status: independent documentary import; no registry entity created

## 1. Work record

- Importer identifier: `importer-a`.
- Start and end time: 2026-07-14, one authoring session.
- Active authoring minutes: approximately 55 minutes for this worksheet.
- Source-reading minutes: approximately 25 minutes.
- Atlas-modeling minutes: approximately 30 minutes.
- Human interventions and their duration: source-pair selection was fixed before
  the import; no intervention during authoring.
- Tools used beyond browser, editor and existing Atlas CLI: `curl` and `pandoc`
  to inspect the pinned documentation and directly linked release source.

## 2. Source identity

### petgraph

- Source subject and source-local name: `UnionFind<K>`.
- Authors or maintainers: petgraph maintainers.
- Work or project title: `petgraph`.
- Edition, release, tag or commit: release `0.8.3`.
- Section, page, module or symbol: `petgraph::unionfind::UnionFind`.
- Stable URL:
  <https://docs.rs/petgraph/0.8.3/petgraph/unionfind/struct.UnionFind.html>.
- Retrieval date: 2026-07-14.
- Source class: library.
- Code license: `MIT OR Apache-2.0`.
- Documentation license or copyright status: distributed with the crate under
  the crate's dual license.
- Additional page consulted: source linked by rustdoc,
  <https://docs.rs/petgraph/0.8.3/src/petgraph/unionfind.rs.html#1-230>.

### Algorithms, 4th Edition

- Source subject and source-local name: `UF`, described as union-find or
  disjoint-sets data type.
- Authors or maintainers: Robert Sedgewick and Kevin Wayne.
- Work or project title: *Algorithms, 4th Edition* and `algs4.jar`.
- Edition, release, tag or commit: fourth edition; source page updated
  2026-01-10.
- Section, page, module or symbol: Section 1.5; class
  `edu.princeton.cs.algs4.UF`.
- Stable URL:
  <https://algs4.cs.princeton.edu/code/edu/princeton/cs/algs4/UF.java.html>.
- Retrieval date: 2026-07-14.
- Source class: book and educational library.
- Code license: GPLv3 or later.
- Documentation license or copyright status: copyrighted booksite material;
  only independently worded facts are recorded here.
- Additional pages consulted: none.

## 3. Source-faithful account

### petgraph account

- Problem stated by the source: maintain membership in disjoint sets for
  elements indexed `0..n-1`, with an unsigned scalar index type
  (`UnionFind` description).
- Inputs and their representation: construction with `n`, empty/capacity-based
  construction followed by `new_set`, and operations on unsigned `IndexType`
  values. State is two vectors, parent and rank (description and source lines
  17-27, 51-93).
- Preconditions and validity domain: element indices must be in bounds for
  panicking operations. `try_*` methods report invalid indices, except
  `try_union(x, x)` returns `Ok(false)` before validating even when out of bounds
  (`find`, `equiv`, `union`, and `try_union` documentation).
- Output or observable interface: representative lookup in immutable or
  compressing mutable form; equivalence query; union returning whether a merge
  occurred; length/capacity operations; dynamic singleton insertion; consuming
  conversion to a representative label vector (`UnionFind` method list).
- Postconditions and guarantees: initially each element represents its own set;
  union combines two equivalence classes and returns false if already equal;
  `find_mut` preserves membership while flattening paths; `into_labeling`
  returns a representative for every element (source lines 55-60, 121-152,
  174-230).
- Strategy: parent forest balanced by rank. `try_union` first performs
  compressing finds, attaches lower rank under higher rank and increments rank
  on a tie. `find_mut` performs path halving by writing a grandparent while
  walking (source lines 144-152 and 190-216).
- Named invariants: each root stores its own index; parent links form equivalence
  classes, each with a unique representative; rank approximates tree depth and
  remains logarithmic (source lines 17-27).
- Persistent and temporary state: persistent parent and rank vectors; no
  component count. Capacity is managed independently of logical length.
- Mutations, allocation, I/O and failure behavior: constructors/new-set/reserve
  allocate; union and mutable find rewrite parents/ranks; immutable find and
  equivalence do not mutate. Panicking and fallible variants coexist; reserve
  may panic on capacity overflow and `try_reserve` exposes allocation failure.
  No I/O.
- Time claims, including operation and case being bounded: amortized O(alpha(n))
  per operation is stated for the data structure; `new_set` is amortized O(1)
  (`UnionFind` description and `new_set`). Separate worst-case operation bounds
  are not stated on the page.
- Space claims, including what is excluded: not stated asymptotically. Source
  representation is two vectors of length `n`, with capacity possibly larger.
- Determinism, randomness or numerical assumptions: no randomness. The chosen
  representative depends deterministically on union order and tie handling for
  fixed inputs, but representative identity is not a stable semantic label.
- Variants explicitly distinguished by the source: immutable `find` versus
  flattening `find_mut`; panicking versus `try_*` APIs; fixed-size versus growable
  construction; capacity-management operations.
- Ambiguities or internally inconsistent statements: the quoted amortized
  operation claim does not specify the exact operation sequence or whether
  immutable, noncompressing `find` is included. `try_union(x, x)` has a documented
  validation exception that is easy to lose in a generic error contract.

### Algorithms, 4th Edition account

- Problem stated by the source: maintain a partition of `n` elements, initially
  singleton sets, with `union`, `find` and `count`; elements are named `0..n-1`
  (class documentation, lines 27-53).
- Inputs and their representation: fixed nonnegative `n` at construction and
  integer element IDs for later operations (lines 87-103).
- Preconditions and validity domain: `n >= 0`; every element argument lies in
  `0..n-1`; invalid values throw `IllegalArgumentException` (lines 87-103 and
  164-169).
- Output or observable interface: `find` returns a canonical representative;
  equal representatives exactly characterize common membership; `union` merges
  two sets; `count` returns the current number of sets. Deprecated `connected`
  is two finds (lines 27-57 and 104-163).
- Postconditions and guarantees: every element is in exactly one set. The class
  documentation claims the canonical element changes only when the set changes
  during `union`, not during `find` or `count` (lines 33-57). Membership is
  preserved by path compression and changed only by union.
- Strategy: weighted quick-union by rank with path compression by halving;
  union attaches smaller rank to larger, increments rank on a tie and decrements
  component count (lines 59-68 and 111-163).
- Named invariants: parent forest, root/canonical representative, rank of rooted
  subtree, and component count. The source comments state rank never exceeds 31
  for its integer-indexed representation (lines 83-85).
- Persistent and temporary state: fixed-size parent and rank arrays plus count.
- Mutations, allocation, I/O and failure behavior: constructor allocates arrays;
  both `union` and `find` mutate parent links because `find` performs path
  halving; `count` is read-only. The data type does no I/O, while its `main`
  client reads pairs and prints merges. Invalid arguments throw exceptions.
- Time claims, including operation and case being bounded: constructor Theta(n);
  union and find each Theta(log n) worst case; count Theta(1); any intermixed
  sequence of `m` union/find operations from the initialized structure takes
  O(m alpha(n)) (class documentation lines 59-70).
- Space claims, including what is excluded: not stated asymptotically on the
  class page; implementation contains two `n` arrays plus one scalar count.
- Determinism, randomness or numerical assumptions: no randomness. Membership
  results are deterministic; canonical representative depends on operation
  order and rank ties.
- Variants explicitly distinguished by the source: `QuickUnionUF`,
  `QuickFindUF`, and `WeightedQuickUnionUF` provide alternative implementations
  of the same API (lines 72-76). `connected` is deprecated in favor of two finds.
- Ambiguities or internally inconsistent statements: the documentation says a
  canonical element cannot change during `find`, but `find` path compression
  does not change the root returned for any member, so this is consistent if
  "canonical element" means root rather than internal parent. The distinction
  is semantically important.

## 4. Proposed Atlas normalization

- Proposed `Problem` identity and reason:
  `dynamic_sets.disjoint_set_operation_sequence`: consume initialization plus an
  ordered sequence of create/find/equivalent/union/count operations, maintain a
  partition, and emit each query/merge result. An operation-sequence contract is
  necessary because no individual call describes the stateful problem.
- Proposed `Algorithm` identity and reason:
  `dynamic_sets.union_by_rank_with_path_halving`. It captures the common parent
  forest, rank balancing and path-halving strategy. Petgraph's immutable find and
  growable set creation are explicit variants, not silently part of the common
  algs4 contract.
- Proposed implementation identity, if executable source is in scope: none in
  K-M0. Future candidates could be `rust.petgraph.union_find.0_8_3` and
  `java.algs4.uf`, subject to dependency and license decisions.
- Existing Atlas entity that may be synonymous: none.
- Proposed problem `input`, `requires`, `output`, `ensures`: input initial
  element count and operation stream; all referenced elements must already
  exist; output representative/equivalence/merge/count responses as requested;
  ensure the maintained relation is an equivalence relation, union merges
  exactly the two classes, find returns a member representing its class, and
  path rewrites never change membership.
- Proposed algorithm `requires`, determinism, complexity and memory claims:
  stable integer-like element identities and mutable persistent storage;
  deterministic partition responses but representative value is
  operation-order-dependent. Preserve algs4's Theta(n) construction,
  Theta(log n) per union/find worst case and O(m alpha(n)) sequence claim as
  declared; preserve petgraph's O(alpha(n)) amortized statement with its
  ambiguity and O(1) amortized new-set claim separately. Memory is O(n) inferred
  from source representation, not promoted as source-declared.
- Proposed implementation effects and tests: persistent allocation proportional
  to elements; union and compressing find mutate; petgraph immutable find does
  not. Tests should compare partitions rather than representative labels, cover
  redundant union, invalid indices, zero elements, interleaved queries,
  compression invariance, dynamic growth and failure/API distinctions.
- Proposed evidence level for each claim and why: API contracts and costs are
  `declared`; source representation and strategy are also source-declared facts;
  O(n) memory would be `inferred` if admitted; future operation-sequence tests
  become `tested`.
- Candidate relationships to other imported subjects: alternative union-find
  strategies implement the same operation-sequence problem; graph connectivity
  clients may compose with it, but no such composition is modeled in K-M0.
- Information intentionally left documentary: exact failure API, capacity
  controls, consuming labeling, component-count availability, mutable versus
  immutable lookup, representative tie policy and source aliases.

Alternative A, used above, treats the data type as one operation-sequence
problem. Alternative B creates separate problems for union, find, equivalence
and growth; it loses persistent cross-operation contracts and makes composition
depend on hidden shared state. Alternative C models only a batch partition from
a list of pairs; it is executable in schema 0.1 prose but discards online query
behavior and amortized sequencing. A is recommended despite being only
free-text-expressible today.

## 5. Fidelity assessment

### Bibliographic fidelity

- Preserved identifiers and locators: crate release and symbol; authors, edition,
  section, class, URLs and licenses.
- Missing or unstable source identity: no separate petgraph commit; algs4 has a
  page date rather than a tagged library release.
- Assessment: preserved.

### Algorithmic fidelity

- Preserved strategy, invariants and validity conditions: parent forest, unique
  root per class, rank balancing, path halving and valid element domain.
- Semantic details lost or altered: the common algorithm does not make
  petgraph's immutable lookup, growth, capacity API or algs4's count operation
  universal. Representative identity is deliberately not a stable output
  guarantee.
- Assessment: partial across complete APIs; preserved for the common union/find
  strategy.

### Representational fidelity

- Source vocabulary and decomposition retained: union-find/disjoint sets,
  representative/root, parent, rank and source-local operation names are
  recorded.
- Normalized or collapsed concepts: object methods become an abstract operation
  stream; Rust fallible APIs and Java exceptions become documentary variants.
- Assessment: intentionally transformed.

### Executable fidelity

- Upstream implementation or examples available: yes for both sources.
- Correction oracle proposed: execute the same bounded operation sequences and
  compare full equivalence relations and component counts after each step, not
  raw representative IDs; separately test source-specific failure/growth APIs.
- Behavior actually checked during this worksheet: none.
- Assessment: not assessed.

### Declared transformations

- translation: object APIs to a language-neutral state-transition contract.
- specialization or generalization: common fixed-element union/find core;
  petgraph growth and algs4 count retained as explicit extensions.
- type or representation adaptation: generic unsigned `K` and Java `int`
  normalized to valid stable element IDs.
- API decomposition or aggregation: constructor and method calls aggregated into
  one ordered operation-sequence problem.
- bug correction: none.
- pedagogical simplification: capacity/reserve mechanics excluded from the
  common semantic contract but retained as implementation effects.
- other: no source code copied.

## 6. Model-friction record

| Source fact | Current Atlas destination | Result | Decision affected | Provisional location |
|---|---|---|---|---|
| Persistent object state across operations | problem input/output prose | lossy | selection, composition | experimental annotation |
| Per-operation pre/postconditions | `requires` / `ensures` prose | lossy | selection, substitution | experimental annotation |
| Amortized bound over intermixed sequence | one time claim string | lossy | selection | worksheet |
| Worst-case and amortized costs coexist | `time_worst` plus optional expected | absent | selection | experimental annotation |
| `find` may or may not mutate representation | implementation effects summary | ambiguous | substitution, composition | worksheet |
| Growable versus fixed universe | problem input prose | ambiguous | selection | worksheet |
| Representative identity is non-semantic | output prose | lossy | substitution | worksheet |
| Alternative strategies implement same API | no variant/equivalence relation | absent | identity, substitution | worksheet |
| Parent forest and state transitions in AST | no direct types/operations | absent | documentary only | worksheet |

These losses can select a fixed-universe implementation when growth is required,
or a mutating query where read-only sharing is required. Treating representative
IDs as stable can falsely report inequivalence. Failure to model amortized
operation sequences can mis-rank candidates or reject a valid substitution.
Persistent state is family-salient, but streaming top-k supplies a structurally
different second case; mutable-query distinctions also occur in self-adjusting
trees or caches. Those cases should precede any schema proposal. AST absence is
not an import gate.

## 7. Operational probes

- identity lookup: **unsupported** before entities exist; exact ID lookup would
  work after import, union-find/disjoint-set alias lookup would remain textual.
- search by the proposed vocabulary: **supported** only when both aliases and
  operation names are explicitly repeated in claims.
- qualification by current properties: **unsupported** for fixed/growable
  universe, query mutation, component count and per-operation amortized costs.
- substitution between source variants: **unsupported** generically; comparing
  raw representative outputs would also produce false negatives.
- composition with a stated precondition/effect: **would require
  source-specific logic** because current composition has no persistent-state
  contract or dynamic candidate discovery.

All outcomes are hypothetical; no CLI test was executed.

## 8. Importer conclusion

- Recommended normalization: one explicit operation-sequence problem with a
  common union-by-rank/path-halving algorithm, plus documentary capability
  differences for growth, count and immutable lookup.
- Unresolved questions: how to express state transitions, operation-indexed
  costs, representation-only mutation and semantically irrelevant
  representatives without creating data-structure-specific schema fields.
- Decision-relevant loss: current properties cannot safely distinguish
  fixed/growable state, mutating/nonmutating find or amortized mixed-operation
  guarantees.
- Documentary divergence that should remain acceptable: `K` versus integer IDs,
  panic/result versus exception, component count availability, exact rank type
  and capacity management.
- Proposed next minimal experiment: run the same declarative operation sequence
  against both sources, compare partitions after every operation, then add a
  growth operation and a read-only-find constraint to demonstrate the exact
  eligibility boundary.
- Schema or AST change requested now: none.
