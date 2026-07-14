# K-M4 import A: collision-aware hash-map insert

Protocol: `k-m0.2`

## Work record and source identity

- Importer: `importer-a`.
- Start timestamp: `2026-07-15T01:12:41+02:00`.
- End timestamp: `2026-07-15T01:22:51+02:00`.
- Active/source/modeling subdivisions: unavailable.
- Human intervention: none during this import.
- Tools beyond browser/editor: `curl` and `pandoc`.
- Subject: `hashbrown::HashMap::insert`.
- Maintainer/work/release: hashbrown maintainers, crate `hashbrown 0.17.1`.
- Source class: Rust library API plus standard-library cost convention.
- Mandatory locators actually read:
  <https://docs.rs/hashbrown/0.17.1/hashbrown/struct.HashMap.html#method.insert>,
  <https://docs.rs/hashbrown/0.17.1/hashbrown/>, and
  <https://doc.rust-lang.org/1.85.0/std/collections/index.html#performance>.
- Supplemental pages consulted: none.
- Retrieval date: 2026-07-15.
- Crate license: `MIT OR Apache-2.0` as identified on docs.rs; documentation
  copyright details are not stated on the selected API text.

## Source-faithful account

- Problem: insert a key-value pair into an associative map, either creating a new
  mapping or replacing the value of an equal existing key (mandatory `insert`).
- Inputs: mutable `HashMap<K,V,S,A>`, owned key `K`, owned value `V`; the relevant
  map requires `K: Eq + Hash`, a `BuildHasher`, and allocator (mandatory type and
  method context).
- Preconditions: equal keys must hash equally. Mutating a resident key so its
  hash or equality changes is a logic error. A key's `Hash` implementation
  panicking is also a logic error and may corrupt contents/drop entries
  (mandatory `HashMap` type page).
- Output: `None` when the key was absent; otherwise replace the old value and
  return it as `Some(old_value)`. On equality with a nonidentical stored key, the
  original stored key is retained rather than replaced (mandatory `insert`).
- Strategy: hashbrown is a SwissTable port; its map uses quadratic probing and
  SIMD lookup (mandatory crate context and type description).
- State/effects: map table and value mapping are mutated; key and value are
  consumed. Insertion may cause array resize, an O(n) extra operation whose cost
  is amortized over a series (mandatory collection cost convention). Allocation
  is therefore possible.
- Failure/I/O: no normal failure is specified by `insert`; allocation and custom
  `Hash`/`Eq` behavior can fail or violate logic outside the method's return
  channel. No method I/O, although user-defined key logic could involve global
  state or I/O in the logic-error cases described by the type page.
- Time: the supplied convention gives HashMap insert O(1) expected and amortized
  (`O(1)~*`). It states that collisions can cause significantly worse than
  expected performance, though considered very unlikely under its probabilistic
  hashing premise. The hashbrown `insert` page itself states no bound; the
  convention is therefore recorded as the packet's cost convention, not as an
  implementation-specific worst-case theorem.
- Space: no asymptotic bound is stated for one insertion. Resize allocates a
  larger backing array and moves entries (mandatory convention, capacity
  discussion).
- Determinism/security: abstract mapping and returned old value are deterministic
  for valid fixed `Eq`/`Hash` behavior. Internal placement is not an interface
  guarantee. The default hash algorithm is foldhash, may change, is fast, and
  typically does not protect against HashDoS (mandatory hashbrown type page).
- Variants: custom hash builders are supported; `insert_unique_unchecked` skips
  duplicate lookup under a safety precondition but is outside this subject.
- Source ambiguity: the standard-library convention names `HashMap` generally;
  it does not prove a precise collision bound for hashbrown 0.17.1 or define the
  probability space behind “very unlikely.”

## Proposed Atlas normalization

- `Problem` identity: `associative_map.insert_or_replace` because replacement and
  old-value return are part of the observable contract.
- Exact `input`: mutable finite key-to-value map, owned `(key, value)` pair.
- Exact `requires`: `key` has equivalence and hash operations with
  `equal(a,b) => hash(a)=hash(b)`; resident key equivalence/hash remains stable;
  allocation is permitted unless capacity is separately guaranteed.
- Exact `output`: optional previous value associated with an equal key.
- Exact `ensures`: afterward exactly one mapping for the key equivalence class
  has the new value; all other mappings are unchanged; if an equal key existed,
  its stored key object is preserved and its old value is returned.
- `Algorithm` identity: `associative_map.swisstable_quadratic_probe_insert`.
- Algorithm requirements: compatible hash/equality and table capacity management.
- Determinism: deterministic abstract map transition for fixed operations;
  collision pattern/internal placement is outside the guarantee.
- Time: convention-declared O(1) expected and amortized; implementation worst
  case and adversarial-collision bound are not stated.
- Memory: not stated asymptotically; possible O(n)-sized table growth is an
  implementation effect, not a fabricated auxiliary-memory claim.
- Evidence: contracts and representation are `declared`; convention-level costs
  are `declared` with their different source locator; no claim is tested/proven.
- Implementation boundary: `hashbrown::HashMap<K,V,S,A>::insert` 0.17.1. It
  mutates/possibly allocates, owns inputs, returns an old value, performs no
  declared I/O/blocking, and retains an existing equal key object.
- Candidate tests: absent key, equal identical/nonidentical keys, collision sets,
  pre-reserved and resize boundaries, malformed `Eq`/`Hash` only in isolated
  negative tests, and multiset preservation around replacement.
- Information left documentary: control-byte/SIMD details, exact probe sequence,
  hash seeding, resize factor, allocator failure, adversarial worst case and
  iteration order.

## Fidelity

### Bibliographic fidelity

Crate release, symbols and all three packet locators are preserved. Exact source
commit and documentation license text are not captured. Assessment: **partial**.

### Algorithmic fidelity

Insert/replace semantics, retained key, Eq/Hash contract, quadratic probing,
SIMD lookup, collision caveat and resize amortization are retained. No unstated
worst-case theorem is added. Assessment: **preserved**.

### Representational fidelity

Rust generics/ownership become a language-neutral map transition; the
source-specific SwissTable representation stays algorithm/implementation data.
Assessment: **intentionally transformed**.

### Executable fidelity

Upstream implementation/examples exist. A future oracle can compare mapping and
old-value semantics using keys equal but not identical, plus controlled hashes.
No behavior was executed. Assessment: **not assessed**.

### Declared transformations

- Translation: Rust API to state transition.
- Generalization: generic map insertion problem separated from SwissTable.
- Type adaptation: `Eq`/`Hash` become explicit equivalence/hash laws.
- API decomposition: only safe `insert`, excluding entry and unchecked APIs.
- Bug correction: none.
- Pedagogical simplification: control bytes and allocator policy remain
  documentary.
- Other: cost convention attribution remains separate from method attribution.

## Model friction

| Source fact | Schema 0.1 destination | Result | Decision affected |
|---|---|---|---|
| Expected plus amortized cost | time claim strings | lossy | selection |
| Collision/adversary-dependent degradation | no structured cost condition | absent | selection, substitution |
| Eq/Hash algebraic law | free-text requirement | lossy | selection |
| Existing equal key is retained | output/ensures prose | lossy | substitution |
| Mutable persistent map state | input/output prose | lossy | composition |
| Resize and allocator effects | implementation effects prose | lossy | selection, composition |
| Default hasher lacks HashDoS protection | implementation requirement prose | lossy | selection |

Candidates with distinct collision resistance or key-replacement semantics could
become indistinguishable. An invalid default-hasher candidate could be selected
for adversarial keys. A no-allocation composition could be rejected or accepted
incorrectly without capacity. Mixed amortized/expected regimes independently
occur in heap push; adversarial distributions also recur in Bloom membership.

## Selection requests

1. “Insert trusted application keys; expected amortized constant time is
   sufficient.” **Accept conditionally**: the supplied convention is O(1)~*,
   assuming valid keys and its nonadversarial hashing premise.
2. “Accept attacker-controlled keys with a documented HashDoS-resistance
   guarantee using the default builder.” **Reject**: foldhash is documented as
   typically not protecting against such attacks.
3. “Replacing an equal but nonidentical key must preserve the original stored key
   object and return the prior value.” **Accept**: this is the exact `insert`
   contract.

Generic schema qualification cannot currently distinguish any of these without
source-specific text interpretation.

## Ambiguities and conclusion

- Source ambiguity: applicability of the std cost convention to exact
  hashbrown behavior and the undefined collision probability model.
- Protocol ambiguity: whether the supplied convention is normative evidence or
  contextual vocabulary; this import treats it as contextual declared evidence.
- Model ambiguity: no field separates expected collision cost, resize
  amortization and adversarial worst case.
- Recommended normalization: insert-or-replace problem, SwissTable quadratic
  probing algorithm, versioned hashbrown implementation.
- Minimal next probe: controlled equal-hash keys across reserve/resize boundaries,
  recording probes and allocations without generalizing timings.
- Public schema change requested: **none**.
