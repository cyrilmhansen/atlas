# K-M4 importer B: collision-aware hash-map insert

Protocol: `k-m0.2`

## 1. Work and source record

- Importer: `k-m4-importer-b`.
- Importer-observed batch start: `2026-07-15T01:12:49+02:00`.
- Externally observed batch start/end and elapsed minutes: unavailable to the
  importer; orchestrator-owned.
- Importer-observed end: `2026-07-15T01:24:31+02:00`.
- Activity subdivisions: unavailable; no retrospective timing estimate.
- Human interventions: none.
- Tools: browser/web fetch, `curl` for exact frozen HTML, editor.
- Subject: `hashbrown::HashMap::insert`.
- Maintainers/work/release: hashbrown maintainers, hashbrown 0.17.1.
- Required locators actually read:
  - <https://docs.rs/hashbrown/0.17.1/hashbrown/struct.HashMap.html#method.insert>
  - <https://docs.rs/hashbrown/0.17.1/hashbrown/>
  - <https://doc.rust-lang.org/1.85.0/std/collections/index.html#performance>
- Supplemental pages: none.
- Retrieval date: 2026-07-15.
- Source class: Rust library API plus standard-library cost convention.
- Code license: hashbrown MIT or Apache-2.0. Documentation copyright/license is
  not separately stated on the pages read.

## 2. Source-faithful account

- Problem: insert a key-value pair into a mutable map; return `None` for a new
  key or replace and return the old value for an equal existing key. On
  replacement, the stored key object is not updated (`HashMap::insert`, required).
- Inputs: `&mut HashMap<K,V,S,A>`, owned key `K`, owned value `V`; keys implement
  `Eq` and `Hash`, with allocator and hash builder in the map type (required map
  context and signature).
- Requires: equal keys must hash equally. Changing a resident key's equality or
  hash is a logic error. A panicking key `Hash` implementation is a logic error
  and may corrupt contents/drop items (required map context).
- Output/ensures: `Option<V>` as above; after success the equal-key class maps to
  the new value, preserving the previously stored key object on replacement
  (required insert page).
- Strategy: hash map implemented with quadratic probing and SIMD lookup;
  default hasher is currently foldhash and may change. It is fast but typically
  does not protect against HashDoS; callers can replace it (required crate/map
  context).
- State/effects: mutates persistent map; consumes key and value; returns/drops
  replaced value according to caller handling; insertion may allocate because an
  empty zero-capacity map does not allocate until first insert. No I/O/blocking
  is stated (required context and method).
- Time: the hashbrown insert page states no bound. The required Rust 1.85 cost
  convention marks standard `HashMap` insert O(1) expected and amortized, explains
  resize may cost O(n), and says hashing can perform significantly worse through
  duplicate hashes. That page supplies notation/context, not an independently
  proven hashbrown 0.17.1 bound.
- Space: no auxiliary or capacity-growth bound is stated for this method.
- Determinism/security: iteration/order and default hash details are not part of
  insert output. Default foldhash lacks typical HashDoS protection. Collisions
  are semantically resolved through equality under the required contract, but
  can change cost.
- Variants: existing-key replacement versus absent-key insertion; configurable
  hash builder/allocator. `insert_unique_unchecked` is visible as another method
  but was not a required source and is not imported.
- Source ambiguity: no exact time bound or collision worst case is declared by
  hashbrown; the cost-convention page discusses standard `HashMap` generally.

## 3. Proposed Atlas normalization

- `Problem` identity: `map.insert_or_replace_by_key_equality`.
- Exact input: `mutable finite map, owned key, owned value`.
- Exact requires: `key equality is an equivalence relation; equal keys hash
  equally; resident key equality/hash remain stable; hashing does not panic;
  allocation succeeds if capacity growth is required`.
- Exact output: `optional old value`.
- Exact ensures: `if no equal key existed, add the supplied key/value and return
  none; otherwise retain the resident key object, replace its value, and return
  the old value`.
- `Algorithm` identity: `map.quadratic_probe_simd_insert`, bounded to hashbrown
  0.17.1 and its configured hasher. This distinguishes collision resolution from
  the abstract map update.
- Algorithm requires: exact problem requirements and a hasher appropriate to the
  caller's collision/adversary regime.
- Determinism: abstract mapping update `true` under stable `Eq`/`Hash`; physical
  placement/cost not guaranteed deterministic across configurations.
- Time: `not stated for hashbrown 0.17.1 by the frozen method/context pages`.
  The temporary documentary expectation may cite the separate convention
  `expected amortized O(1), resize O(n), collision degradation possible`, but it
  must not be attached as a tested/proven hashbrown-specific theorem.
- Auxiliary memory: not stated; table growth may allocate.
- Implementation identity: `hashbrown-0.17.1.hash_map.insert`.
- Boundary/effects: Rust library method; map mutation; key/value ownership
  transfer; optional replaced-value return; possible allocation/reallocation;
  logic-error containment not specified; no stable ABI or I/O/blocking claim.
- Tests proposed: new key, equal-but-nonidentical replacement preserving stored
  key, deliberate collisions, all keys same hash, resize threshold, invalid
  mutable-key behavior only as a documented negative boundary, and custom
  adversary-resistant hasher.
- Evidence: API semantics/strategy/security warnings `declared`; standard cost
  notation `declared` only for its own page; no executable evidence here.
- Existing Atlas synonym: not assessed under experiment boundary.
- Documentary only: table control bytes/groups, exact load threshold, SIMD width,
  foldhash internals, allocator growth, and panic containment.

## 4. Fidelity and transformations

### Bibliographic fidelity

Crate release, exact type/method and all required locators are preserved; no
commit is recorded. Assessment: `partial`.

### Algorithmic fidelity

Collision resolution family, equality/hash contract, replacement semantics and
HashDoS warning are preserved. No hashbrown-specific complexity is invented.
Assessment: `preserved`, with cost `unresolved`.

### Representational fidelity

Generic Rust types, custom hasher/allocator, and resident-key preservation remain
visible; physical buckets are omitted. Assessment: `intentionally transformed`.

### Executable fidelity

Upstream executable code/examples exist. Oracle: compare mapping by equality,
old-value return and resident key identity across collision-heavy fixtures. No
behavior was run. Assessment: `not assessed`.

### Declared transformations

- Translation: Rust method to state transition.
- Specialization/generalization: source-bounded quadratic-probe implementation
  separated from abstract insert-or-replace problem.
- Type adaptation: `Eq`/`Hash` become explicit semantic preconditions.
- API aggregation/decomposition: absent and present cases remain one operation.
- Bug correction: none.
- Pedagogical simplification: physical SIMD/bucket mechanics documentary only.
- Other: standard collection costs retained as a convention, not reattributed.

## 5. Schema-loss record

Schema 0.1 cannot type persistent map state, hasher/adversary assumptions,
collision regime, equal-but-not-identical key retention, expected plus amortized
cost notation, per-call resize, logic errors, or panic containment. A mandatory
`time_worst` claim would force an unsupported value; least-lossy temporary text
is `not stated by frozen hashbrown sources; collision and resize sensitive`.

## 6. Selection requests

1. **Request:** insert/replace with old-value return under trusted keys and a
   nonadversarial collision distribution. **Accept semantically**; performance
   remains an unverified expectation, not a source-specific guarantee.
2. **Request:** preserve the newly supplied key object when it compares equal to
   an existing key. **Reject**: the existing key object is retained.
3. **Request:** provide bounded worst-case insertion under attacker-controlled
   keys with the default hasher. **Reject**: the source warns default foldhash is
   typically not HashDoS-resistant and gives no worst-case bound.

## 7. Ambiguities and conclusion

- Source ambiguity: exact worst/expected complexity, load factor and failure
  containment are unstated.
- Protocol ambiguity: the required cost-convention source could be misread as a
  hashbrown-specific complexity source; this worksheet does not do so.
- Model ambiguity: no structural place for collision/adversary regime or logic
  error, although both alter selection.
- Decision-relevant divergence: importing O(1) as unconditional or guaranteed
  would wrongly accept adversarial/worst-case requests.
- Public schema change requested: **none**. Record cost/security gaps outside the
  public schema until another independent family justifies a proposal.
