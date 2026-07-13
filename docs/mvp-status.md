# MVP status

- Active MVP: **MVP 4 - LP64 MIR adapter probe**
- Status: single-region interpreter/JIT checkpoint plus read-only RV64 generation
- MVP 1: closed locally at baseline `8a2a520`
- MVP 2: closed locally under DEC-036
- MVP 3: closed locally under DEC-038

## Current slice

MVP 1, MVP 2, and MVP 3 are closed locally. MVP 4 tests the MIR adapter
boundary with a standard RV64 LP64 ABI probe and independent compact-reference
comparison; it does not activate RV64ILP32 or a fantasy computer.

Current corpus progress:

- Problems: 10 / 10
- Algorithms: 15 / 15
- Implementations: 20 / 20

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
`is_sorted` function that calls a checked single-region offset import. Guest
writes, multiple regions and allocation remain unselected.

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
RV64ILP32. The narrow host-JIT correction path covers read-only and mutating
guest memory; latency, executable-allocation measurement, RV64 guest writes,
multi-region memory, guest allocation and persistent MIR traces remain open.

See `docs/mvp4-review.md`, `docs/mir-integration.md`, and DEC-039 for the
accepted scope, checks and limits.
