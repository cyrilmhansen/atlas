# MIR integration boundary

This document describes the actual MVP 4 integration. MIR is an execution
adapter, never the source of Atlas semantics.

## Source and build

MIR is the original `vnmakarov/mir` repository, held as the `vendor/mir`
submodule at commit `a8ab7c31cd5f9b23b77d84c60b3d83e62d9d304c`.

`crates/atlas-mir/build.rs` invokes `CC` and `AR` directly. It compiles only
upstream `mir.c`, which includes the interpreter, into a private static archive.
The MIR generator and C-to-MIR compiler are deliberately absent. It separately
compiles `mir_shim.c` and links both archives with `dl` and `m` on Linux.

```sh
git submodule update --init --recursive
cargo test -p atlas-mir --locked --offline
```

## Current execution path

The only executed MIR function is `add_u64`, recreated by `mir_shim.c` on each
call:

1. `MIR_init` creates a context.
2. The shim creates one module and one `i64` function with two `i64` arguments.
3. It appends `MIR_ADD` and `ret`, then loads and links it with
   `MIR_set_interp_interface`.
4. `MIR_interp_arr` executes the function.
5. The shim copies the result out and calls `MIR_finish` before returning.

Rust exposes one private `extern "C"` function returning `u64`. MIR local
registers are `i64`, as required by the upstream API used in this probe. No host
pointer is accepted, stored, or returned by this boundary.

## Guest references

The first slice compares values independently of MIR:

| Candidate | Checked now | Deferred |
|---|---|---|
| `GuestOffset(u32)` | overflow and single-region bounds | allocation and lifetime |
| `GuestHandle(u32)` | invalid handle and object-relative bounds | reuse and reclamation |
| `GuestRegionOffset` | region identity, overflow and bounds | encoding and lifetime |

They are neither host pointers nor MIR pointer operands. One representation and
its lifetime rules must be selected before a guest reference crosses a MIR call
boundary.

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

MIR generator interfaces are not compiled by this crate. Enabling them requires
a separate decision, host-JIT smoke test, and size/latency measurement protocol.
MIR RISC-V generation is a later experiment and cannot be inferred from the
LP64 QEMU probe.

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
