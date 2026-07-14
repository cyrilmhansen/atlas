# K-M2 dynamic-structure corpus batch

Status: complete  
Recorded: 2026-07-15  
Protocol: `k-m0.2`

## Scope

K-M2 imports persistent operations from three independent dynamic-structure
families. The registry grows from 14/19/22 to 25 problems, 30 algorithms and 34
implementations.

| Family | Construction | Mutation | Query |
|---|---|---|---|
| disjoint sets | singleton parent forest | rank union with path-halving lookup | immutable equivalence |
| max priority queue | bottom-up heapify over caller storage | push and pop-maximum | peek-maximum |
| associative map | capacity reservation with a supplied hasher | insert and remove | borrowed lookup |

Each operation is a separate `Problem` and `Algorithm`. Its input and output name
the persistent state explicitly. This avoids registering a library data type as
one opaque implementation, while keeping state continuity visible in schema 0.1
prose.

## Sources and executable boundary

- petgraph 0.8.3 `UnionFind`: release-pinned
  [API](https://docs.rs/petgraph/0.8.3/petgraph/unionfind/struct.UnionFind.html)
  and source already reviewed independently during K-M0;
- Rust 1.85 API baseline `BinaryHeap`: version-pinned
  [API](https://doc.rust-lang.org/1.85.0/std/collections/struct.BinaryHeap.html)
  and [binary-heap construction](https://doc.rust-lang.org/1.85.0/std/collections/binary_heap/index.html);
- hashbrown 0.17.1 `HashMap`: release-pinned
  [API](https://docs.rs/hashbrown/0.17.1/hashbrown/struct.HashMap.html) and
  [crate description](https://docs.rs/hashbrown/0.17.1/hashbrown/).

petgraph and hashbrown remain exact test-only dependencies of `atlas`; neither
enters the CLI's production dependency graph. `BinaryHeap` is tested through the
standard library. The adapter executes interleaved union-find operations,
capacity-aware heap operations and a hash map whose custom hasher deliberately
maps every key to the same hash.

## Cost scopes

K-M2 does not equate worst-case, expected and amortized costs.

| Operation | Registry worst case | Registry expected | Preserved only in this report |
|---|---|---|---|
| union-find union/equivalence | `O(log n)` inferred from rank-bounded forest depth | absent | upstream broad `O(alpha(n))` amortized operation claim |
| heap construction from `Vec` | `O(n)` | absent | none |
| heap push | `O(n)` for one resizing call | `O(1)` over input order and enough pushes | adverse ascending-order amortized `O(log n)` per push |
| heap pop | `O(log n)` | absent | none |
| heap peek | `O(1)` | absent | none |
| hash insert/get/remove | `O(n)` inferred under collisions | `O(1)` from Rust collection cost conventions | resize amortization and hash-distribution assumptions |

The schema's `time_expected` is not reused for amortized cost. That preserves
meaning but leaves sequence-scoped bounds unqueryable. Heap push and hash-table
operations are two independent families exhibiting the same missing dimension;
this now satisfies the Phase 2 threshold for considering a future schema
proposal, but K-M2 does not make that class-C change.

## State, lifetime and allocation

Construction, mutation and query effects are visible in implementation records:

- union changes parent/rank state, while `equiv` has no mutation;
- heap push may grow storage, pop mutates without allocation and peek borrows
  without mutation;
- map insertion may grow storage, lookup borrows without mutation, and removal
  retains allocated capacity for reuse;
- `BinaryHeap::from(Vec<T>)` consumes caller-provided storage and is recorded as
  requiring no additional allocation.

`atlas qualify priority_queue.construct --allocation none` therefore discovers
the caller-storage heap construction directly from registry claims. No query
branch knows about heaps.

Schema 0.1 still expresses lifetime only textually. It cannot prove that the
`state_after` supplied to a later operation is the exact state produced by the
earlier call, nor distinguish semantic mutation from representation-only path
compression as a typed effect. Those are modeling findings, not runtime gaps.

## Existing-query discovery

K-M2 adds `deduplicate.hashbrown_adapter.rust.vec.v1` as a second independently
tested implementation of the existing `deduplicate.hash.stable` algorithm. It
uses a test-local hashbrown `HashSet` adapter and does not call Atlas algorithm
code.

The pre-existing command:

```text
atlas qualify sequence.deduplicate --stable
```

returns the new implementation without any change to qualification logic. This
is the first direct Phase 2 evidence that adding an implementation record can
expand an existing candidate set generically.

## Fidelity and declared transformations

### Bibliographic fidelity

Crate versions and API symbols are pinned. Rust standard-library behavior is
anchored to the project's Rust 1.85 API baseline and tested with rustc 1.96.0.
Licenses are recorded per implementation.

### Algorithmic fidelity

Union-find preserves rank balancing, path halving and partition semantics;
representative identity is deliberately excluded. Heap operations preserve the
max-heap contract. Hash operations preserve quadratic-probing collision
resolution and key equality/hash requirements.

### Representational fidelity

Atlas normalizes library object methods into state-transition problems. This is
intentionally different from each source API and loses typed ownership/lifetime
continuity. Method boundaries, borrowed results and allocation behavior remain
visible.

### Executable fidelity

The external adapter checks real upstream implementations. Collision behavior
is exercised with distinct keys sharing one hash, rather than inferred from
non-colliding examples.

### Declared transformations

- petgraph panicking and checked variants are represented by the panicking
  `new`/`union`/`equiv` boundary only;
- `BinaryHeap` is specialized to a max-priority queue and construction from an
  owned `Vec`;
- hashbrown is specialized to a caller-supplied hasher; HashDoS resistance is
  not claimed for the test hasher;
- the stable-deduplication adapter is Atlas-authored orchestration over
  hashbrown, not presented as an unchanged upstream algorithm.

## Model-friction matrix

| Mismatch | Severity | Lossiness | Current consequence |
|---|---|---|---|
| amortized sequence bounds absent | high | decision-changing | generic selection cannot compare amortized guarantees |
| persistent-state identity is prose | high | decision-changing | composition cannot prove compatible state flow |
| semantic versus representational mutation | medium | lossy effect | path compression cannot be separately qualified |
| capacity/reservation parameter is prose | medium | lossy effect | numeric storage limits are not filterable |
| borrowed result lifetime is prose | medium | type loss | composition cannot validate borrow duration |
| hasher threat model absent | medium | decision-changing | HashDoS suitability cannot be selected generically |
| dynamic structures absent from AST | low for Knowledge | complete | no execution/presentation lowering, intentionally frozen |

## Acceptance

- construction, mutation, query and lifetime effects distinguishable: complete,
  with lifetime continuity explicitly limited to prose;
- worst-case, amortized and expected costs not collapsed: complete;
- allocation and caller-provided storage affect qualification: complete for
  heap construction and visible for all imported methods;
- new implementation enters an existing candidate set without query code:
  complete for stable deduplication;
- schema, AST, MIR, WASM, Explorer and composition scenarios unchanged:
  complete.

K-M2 closes on these results. K-M3 should now test whether bounded memory,
randomness, approximation and numerical error create the same persistent-state
and cost-scope pressure before any public schema extension is proposed.
