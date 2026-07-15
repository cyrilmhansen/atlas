# Project status

- Active phase: **Phase 2 - External corpus trial** under DEC-066
- Active work program: **Atlas Knowledge**
- Atlas Execution Lab: **frozen**
- Atlas Explorer: **maintained**
- Historical MVP status: MVP 1 through MVP 6 closed; no subsequent MVP activated
- MVP 1: closed locally at baseline `8a2a520`
- MVP 2: closed locally under DEC-036
- MVP 3: closed locally under DEC-038
- MVP 4: closed locally under DEC-052
- MVP 5: closed under DEC-060
- MVP 6: closed under DEC-064

Post-foundation planning is accepted under DEC-065 and DEC-066. The external
audit, three-program vocabulary and active external-corpus phase are recorded in
`docs/audits/2026-07-14-external-project-audit.md`,
`docs/project-vocabulary.md` and `docs/phase2-external-corpus.md`.

## Active slice

K-M4 independent dual import is complete. Two isolated importers produced six
worksheets each from one frozen source packet. Three of five source-readable
subjects converge operationally; three decision-changing divergences are
adjudicated and Welford remains deliberately unresolved because the packet did
not expose its article body. Corpus growth is paused for the bounded ontology
review required by the gate. K-M5 remains planned behind that review.

K-M4-W is also complete. Open Pébay and Chan-Golub-LeVeque reports now support a
neutral incremental second-central-moment import while Welford 1962 remains
historical, not textually verified. The two imports agree strongly and add
empty-state adaptation plus `proven` evidence semantics to the ontology review.

The bounded ontology review is accepted under DEC-067 and DEC-068. Schema 0.1
remains authoritative while K-M5 compares it with a disposable private decision
overlay. `proven` now requires an auditable proof mapping. Protocol `k-m0.3` and
the overlay experiment boundary are specified. The private parser/validator is
complete and tested; no corpus facts, evaluator or CLI surface are active yet.
Its 557 non-test lines are retained as explicit authoring-cost evidence.

## Latest closed slice

MVP 6 replaces algorithm-specific Web steppers and rendering branches with one
private generated program, one bounded WASM visual machine and one derived
presentation description under DEC-061 through DEC-063. Its viewport-width
layout and five generated executions are complete. `sequence.minimum`, even
`sequence.partition`, adjacent `is_sorted`, stable insertion and symmetric
reverse compile from their exact reviewed ASTs, execute through the common
machine and render from projected presentation metadata. The three MVP 5
steppers remain differential references; their generated replacements match the
retained steppers operation-for-operation. Differential migration is complete;
consolidation A has removed specialized browser imports and dispatch while
retaining the WASM exports as differential oracles. DEC-064 accepts this
test-only boundary and closes MVP 6 without stabilizing the private interfaces.

The machine remains limited to one typed sequence, bounded indices and
structured control; its current instruction subset is documented in
`docs/mvp6-visual-machine.md`. Its generated program and presentation
description are disposable build products, not schema, ABI, evidence or
registry authority.

## Completed foundation

MVP 1 through MVP 6 are closed. MVP 5 preserves a
locally openable static catalog with three curated Rust/WASM algorithms and
separated theoretical, counted and locally timed characteristics. Publication,
a stable Web format, advanced trace playback and MIR-in-browser execution remain
outside scope.

First gate status: complete under DEC-057. The private projection reproduces 10
problems, 15 algorithms and 20 implementations with the logical registry
digest. The browser and Node.js bindings execute adjacent `is_sorted`, report
exact comparisons and the first inversion, and reject inputs longer than 4096
elements. The second gate adds stable insertion over tagged values, exact
comparison/swap counts and visible original indices while distinguishing Web
transport copies from the algorithm's `O(1)` auxiliary-space claim. The third
gate adds symmetric reverse, exact source-level read/write/swap counts and an
involution check. All three curated execution workloads are complete;
the Web selector now materializes five deterministic cases from the accepted
sorting DatasetSpec with their original attribution and content digests. The
bundle records its effective build and browser environments. A Git archive
without repository metadata or prior build products reproduces all eight bundle
files bit for bit. DEC-058 keeps MVP 5 open for editable deterministic data,
operation-growth views and bounded semantic dynamics. Adjacent `is_sorted` has
exact AST-node links, pseudocode highlighting and incremental WASM playback.
Its bounded analytical trace is now test-only. Seeded
Explore/Scale generation and deterministic multi-size operation charts are also
complete. Insertion has equivalent typed AST/pseudocode and a stateful WASM
stepper under DEC-059. Reverse now adds typed AST/pseudocode and a symmetric-pair
stepper checked directly against native mutation, counters and involution. All
three browser paths retain current state rather than a presentation trace. The
extended ten-file bundle is byte-identical from a clean Git archive. DEC-060
closes this scope without stabilizing its private Web interfaces or activating a
subsequent MVP.

Current corpus progress:

- Problems: 31 total (10 at MVP 1 baseline)
- Algorithms: 36 total (15 at MVP 1 baseline)
- Implementations: 40 total (20 at MVP 1 baseline)

Current MIR adapter progress: scalar arithmetic and trace imports, mutable even
partition, adjacent `is_sorted`, minimum/maximum selection, reverse, and stable
insertion over private tagged pairs are independently checked against native
Rust. This completes the planned single-region interpreter capability ladder.
These private experiments do not add registry implementations or change corpus
counts.

Host-JIT progress: generated scalar addition, guest-memory `is_sorted`, reverse,
even partition and stable insertion match interpreter and native Rust results
under DEC-046. Partition and insertion keep semantic/correction observability in
the interpreter while the JIT checks results and mutation only. Exact generated
spans and x86-64 instruction shapes are observable. No timing,
executable-allocation measurement or automatic backend choice has been
introduced.

RV64 generator progress: DEC-049 cross-compiles MIR's generator itself for
RV64GC LP64D, runs it under QEMU user mode, and verifies a generated 16-byte
scalar addition by result and disassembly. DEC-050 adds a 128-byte read-only
`is_sorted` function that calls a checked single-region offset import. DEC-051
adds a 176-byte `reverse` function with checked reads and writes. Multiple
regions and allocation remain unselected.

## MVP 1 closure

- 10 problems
- 15 algorithms
- 20 tested Rust implementations
- Git-authoritative manifests with a rebuildable SQLite projection
- `list`, `show`, `validate`, `search`, and `explain` commands

Implemented commands: `validate`, `list`, `show`, `search`, `explain`, and
`qualify`.

Runtime boundary: algorithm implementations live in the `std`-independent
`atlas-algorithms` crate. Its core subset has no dependencies; its default
features enable `alloc` and optional hash-based deduplication. Registry and CLI
remain in the `std`-based `atlas` crate.

## Exit criteria status

| Criterion | Status |
|---|---|
| 10 problems, 15 algorithms, 20 tested implementations | Complete |
| `list`, `show`, `validate`, `search`, `explain` | Complete |
| Add registry components without validator code changes | Demonstrated |
| Qualified claims with provenance | Complete for MVP 1 local schemes |
| Mandatory-property coverage at 90% or more | Complete: required fields enforced |
| Validate semantic types and evidence references | Satisfied for MVP 1 local evidence schemes |
| Git-authoritative committed source | Complete: local MVP 1 baseline established |
| Deterministic rebuild and SQLite projection | Complete |
| Schema versioning and compatibility rules | Complete before freeze |
| Single local acceptance command | Complete: `scripts/check-mvp1.sh` |

See `docs/mvp1-review.md` for evidence and recommended closure order.

## MVP 2 closure

| Criterion | Status |
|---|---|
| Deterministic dataset matrices for sorting and partitioning | Complete |
| Regenerable correction observations | Complete |
| Qualified generated sorting benchmark observation | Complete, non-normative |
| Stable in-place sort with no allocation qualification | Complete |
| Replay from a locally retained execution identifier | Complete |
| Process memory measurement | Complete for the sorting harness |
| Allocation and traversal measurements | Explicitly unavailable |
| Clean, exact comparison campaign | Procedure complete; retained report intentionally not required |
| Allocation and traversal measurements | Explicitly unavailable; deferred by closure scope |
| Numeric memory-limit and cross-evidence queries | Deferred by closure scope |
| Broader observed dominance domains | Deferred to future MVP work |

See `docs/mvp2-review.md` and DEC-036 for evidence, limits, and impact on the
next MVP.

## MVP 3 closure

`atlas compose cleanup` builds one internally represented
`filter -> sort -> deduplicate` pipeline from existing sequence components. It
exposes compatibility, preconditions, mutations, allocations, copies, and a
rejected alternative. It is experimental and does not change schema 0.1 or
define a persistent plan format.

`atlas compose cleanup --rust` renders the selected orchestration, which is
compiled and run as `cleanup_generated`. Atlas itself does not execute generated
source.

The same scenario also supports `--goal expected-time`, using declared
complexity claims only. Both selected alternatives have independently compiled
and runnable Rust orchestration examples; Atlas itself does not execute them.

`atlas compose find` is the second composition scenario. It records the sorted
input required by binary search, the step that establishes it, and a rejected
merge-sort alternative before rendering a compiled Rust orchestration.

`atlas compose partition-sort` makes a structured partition intermediate
explicit, including projection, retention of the other branch, and reassembly.
`atlas compose unique-sort` isolates sorting and deduplication, distinguishing
the required output allocation from rejected intermediate merge/hash storage.
`atlas compose merge-sorted` adds a two-input fan-in, with two explicit sorted
preconditions and their establishing mutations. All render independently
compiled Rust examples.

All composition scenarios accept explicit force/forbid implementation constraints without
modifying the registry. They remain bounded to reviewed candidates; generation
with an override is deferred until its exact source is verified.

See `docs/mvp3-review.md` for the acceptance checks and deliberate limits.

## MVP 4 single-region checkpoint

`atlas-mir` pins the original MIR upstream and executes scalar MIR interpreter
programs through a private C shim. The first semantic trace records the two
comparisons of a stable three-value minimum and is checked against the native
reference implementation. It is private, bounded, in-memory instrumentation,
not an execution record or evidence format. The registry, composition model and
native backend do not depend on MIR.

The adapter lowers specialized partition and adjacent `is_sorted` subsets and
checks exact trace links against their experimental AST nodes. Additional
specialized programs validate first-on-tie minimum/maximum selection, reverse,
and stable insertion using shifted 16-byte tagged pairs. All operate through
private imports over the selected bounded little-endian offset region.

The three compact guest-reference candidates were tested independently: offset,
handle, and region-plus-offset. DEC-040 selects bounded `u32` byte offsets in
one fixed-capacity region for the first guest-memory experiment.
`scripts/check-rv64-lp64-abi.sh` proves the local RV64 LP64 compiler/QEMU-user
path. DEC-049 separately proves scalar MIR-generated RV64 code under that
emulator, and DEC-050 proves one read-only checked guest import; none tests
RV64ILP32. DEC-051 adds checked RV64 mutation. The narrow host-JIT correction
path covers additional read-only and mutating shapes; latency,
executable-allocation measurement, multi-region memory, guest allocation and
persistent MIR traces remain open.

See `docs/mvp4-review.md`, `docs/mir-integration.md`, and DEC-039 for the
accepted scope, checks and limits.
