# MIR integration boundary

This document describes the actual MVP 4 integration. MIR is an execution
adapter, never the source of Atlas semantics.

## Source and build

MIR is the original `vnmakarov/mir` repository, held as the `vendor/mir`
submodule at commit `a8ab7c31cd5f9b23b77d84c60b3d83e62d9d304c`.

`crates/atlas-mir/build.rs` invokes `CC` and `AR` directly. It compiles upstream
`mir.c`, which includes the interpreter and default code allocator, plus
`mir-gen.c`, which selects MIR's generator for the build host. The C-to-MIR
compiler remains absent. The build separately compiles `mir_shim.c` and links
the private archives with `dl` and `m` on Linux.

```sh
git submodule update --init --recursive
scripts/apply-mir-patches.sh
cargo test -p atlas-mir --locked --offline
```

## Current execution path

Every private entry point recreates its MIR module in `mir_shim.c` for one call:

1. `MIR_init` creates a context.
2. The shim constructs one specialized function and its private imports.
3. It appends instructions, then loads and links the module with
   `MIR_set_interp_interface`.
4. `MIR_interp_arr` executes the function.
5. The shim copies results or guest mutations out and calls `MIR_finish`.

The executed set now covers scalar addition and minimum tracing, partition,
adjacent `is_sorted`, minimum/maximum selection, reverse and stable insertion
over tagged pairs. MIR locals and guest scalar values are `i64`. Rust passes a
host backing-buffer pointer only to the private C adapter; MIR programs receive
element counts and compute scalar guest byte offsets. Private imports mediate
all buffer access. No host pointer is stored in a MIR register or exposed as a
guest reference.

## Guest references

The first slice compared three representations independently of MIR:

| Candidate | Checked now | Deferred |
|---|---|---|
| `GuestOffset(u32)` | overflow and single-region bounds | allocation and lifetime |
| `GuestHandle(u32)` | invalid handle and object-relative bounds | reuse and reclamation |
| `GuestRegionOffset` | region identity, overflow and bounds | encoding and lifetime |

They are neither host pointers nor MIR pointer operands. DEC-040 selected
`GuestOffset(u32)` for one fixed-capacity region. Current programs compute these
offsets as MIR scalar integers and use private checked imports for 8-byte loads
and stores. Handles, region IDs, allocation and lifecycle remain unselected for
the executable runtime.

## RV64 LP64 probe

`scripts/check-rv64-lp64-abi.sh` compiles a static C program with
`riscv64-linux-gnu-gcc -march=rv64gc -mabi=lp64d`, confirms an ELF64 RISC-V
binary, then runs it with `qemu-riscv64`. It establishes only the local Linux
cross-toolchain and user-mode emulator path.

It does not test bare metal/Newlib, devices, MIR-generated RISC-V code, or
RV64ILP32. The latter is deferred: the local compiler rejects it for RV64, and
it must not determine compact-reference representation.

## Instrumentation and JIT

The `minimum3_i64` interpreter probe explicitly calls the private import
`atlas_mir_record_compare(candidate, current)` before each of its two semantic
comparisons. The shim copies the bounded events and final result into
`MinimumTrace`, which Rust verifies against the native `sequence.minimum`
implementation. This trace is process-local, deterministic and deliberately
scalar: it contains neither guest references nor timing data.

`MinimumTrace` is not an Atlas evidence format, a registry entity, or a stable
FFI contract. The C shim keeps the active trace in static storage during one
call and is therefore not reentrant. The Rust entry point serializes access to
that storage, making this private API deterministic for concurrent Rust callers.
A future trace transport must define its own concurrency properties explicitly
before it can cross this private adapter boundary.

## Partition AST lowering

DEC-041 lowers the read, predicate, swap and boundary subset of the existing
`partition_ast()` into a private MIR interpreter program. Its only predicate is
signed `i64` evenness. The native Rust partition remains the correction oracle.
The adapter serializes a bounded little-endian guest byte region, passes only
its `u32` byte offsets to MIR, and imports private host functions for `i64`
loads, stores and operation tracing.

Each trace entry contains an exact partition AST node ID and its semantic kind.
The test validates both the node's existence and type against `partition_ast()`.
The trace is bounded to 128 entries and exposes truncation instead of silently
claiming a complete trace. This lowering is not a generic AST compiler, a
public backend API, or an RV64 code-generation test.

The host generator is compiled under DEC-046 and exercised only by the narrow
correction probes below. MIR RISC-V generation remains a later experiment and
cannot be inferred from either host JIT or the LP64 QEMU probe.

## Read-only is-sorted AST lowering

DEC-043 adds `is_sorted_ast()` for `order.is_sorted.adjacent` and lowers its
adjacent signed-`i64` reads and comparison to the host MIR interpreter. It
reuses the same bounded little-endian guest byte region as partition, but makes
no guest writes or allocations. Rust native `is_sorted_by` remains the
correction oracle.

The private result contains the sorted boolean and, on failure, the index of
the right-hand element of the first inverted pair. For example, `[3, 2]`
reports `false` and index `1`. Its bounded trace emits exact node IDs for the
left read, right read and adjacent comparison. Tests check result, first
inversion, early stop and every trace node's declared AST operation type.

## Dual-backend progression

DEC-042 keeps native Rust and MIR indefinitely. Rust remains the qualified
reference implementation and MIR is an additional experimental backend. A MIR
counterpart is added only after the native contract, deterministic cases and,
where applicable, AST operations are already available. No MIR result can
replace native correction evidence.

The rollout order is capability-driven:

| Capability | Candidate algorithms | Required comparison |
|---|---|---|
| Scalar register arithmetic | minimum, maximum | result and tie policy |
| Guest reads and comparisons | is-sorted, minimum, maximum | result and first failing index or tie policy where applicable |
| Guest swap | reverse, partition | mutation, permutation, AST trace for partition |
| Guest writes and shifts | insertion sort | sortedness, stability and permutation |
| Additional regions and explicit allocation | merge sort, filter, merge-sorted, deduplicate | output, allocation/copy effects, region safety |

The single-region rows through guest writes and shifts are complete. Additional
regions are not a routine continuation: they require a new memory-model
decision covering identity, bounds, lifetime and visible allocation/copy
effects.

Each slice runs the same deterministic correction cases through native Rust and
MIR. It checks the returned value, mutated or output data and declared
invariants. Trace-to-AST correspondence is required only when the existing
experimental AST expresses the lowered operations. The bounded interpreter trace
is diagnostic; truncation is reported and cannot certify complete trace coverage.

Interpreter cost is measured separately from native algorithm benchmarks. A
future JIT or RISC-V backend must first reproduce the same correction matrix;
it must not be selected automatically from local timing results.

## Stable insertion pair experiment

DEC-045 uses a private 16-byte guest pair `(i64 key, u64 original_index)` to
make insertion-sort stability observable. MIR compares only the key, retains
the current pair in registers and shifts complete preceding pairs through the
existing little-endian guest load/store imports. Tests compare the entire pair
sequence with native stable insertion sort, so ordering, permutation and stable
duplicate order are checked together. This pair is instrumentation, not a
general guest value representation.

## Host-JIT correction slice

DEC-046 adds two private host-JIT probes. `add_u64` verifies a generated scalar
function without imports. JIT `is_sorted` verifies control flow and calls the
same checked guest-load import used by the interpreter. Empty, singleton,
sorted, duplicate and inverted inputs compare the generated boolean and first
inversion with both interpreter and native Rust results.

Every call initializes and finishes the generator inside a fresh MIR context,
so its executable mappings are released with that context. This establishes
correction and lifecycle only. It does not yet record construction latency,
machine-code size or execution performance, and it does not select JIT over the
interpreter.

Atlas explicitly selects MIR optimization level 2 for the ordinary JIT probes;
it does not rely on the upstream default remaining unchanged. Private
correction variants also exercise levels 0, 1, 2 and 3 for both probes. In the
pinned MIR generator these mean fast generation, register allocation plus
combining, the default SSA/GVN/CCP pipeline, and the full pipeline respectively.
Correction at every level is not a performance comparison.

At the DEC-046 checkpoint, the public generator API returned a machine-code
address but not its exact byte length. Upstream retained the length only while
publishing code and could print it through generator debugging. Atlas declined
to parse that diagnostic as a persistent solution.

DEC-047 prepares a local upstream-compatible correction to that limitation. A
synchronous generator callback observes a complete function after publication
and relocation. Scalar addition and guest-memory `is_sorted` copy their bounded
byte slices before the MIR context is destroyed, then verify length and digest
without invoking debug IO or external tools. The guest case includes generated
control flow and relocated calls to the checked load import. The callback does
not cover separately generated lazy basic blocks.

Atlas vendors the reviewable diff in `patches/mir/code-observer.patch` while the
gitlink remains on the original MIR commit. `scripts/apply-mir-patches.sh`
checks that commit and applies the overlay idempotently; CI runs it immediately
after checkout. Once the patch has an upstream-retrievable commit, the
submodule can be updated and the overlay removed.

The local diagnostic can be inspected without timing or external tools:

```sh
cargo run -p atlas-mir --example observe_jit_code --locked --offline
```

It reports the host target, explicit optimization level, correction result,
exact byte length, local SHA-256 and copied bytes for both programs. Under
DEC-048, x86-64 hosts additionally report a Capstone instruction listing with
relative offsets and Intel syntax. Atlas stops at the first `ret` and reports
the remaining observed bytes separately, because MIR's span includes alignment
padding and relocated import addresses. This is an explicit diagnostic
heuristic, not general control-flow recovery. Nothing is archived.

The structural matrix is a separate untimed diagnostic:

```sh
cargo run -p atlas-mir --example compare_jit_shapes --locked --offline
```

For each optimization level 0 through 3, it first verifies the generated
result, then reports observed span, prefix through the first return, suffix,
instruction count, calls and branch classes. On the current pinned x86-64
stack, level 1 produces the smallest `is_sorted` prefix (112 bytes and 27
instructions), levels 2 and 3 both produce 119 bytes and 31 instructions, and
level 0 produces 144 bytes and 35 instructions. All retain two checked-load
calls, two conditional branches and two unconditional branches. These numbers
are local structural observations, not timing results or a level selection.
