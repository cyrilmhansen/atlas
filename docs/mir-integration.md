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

MIR generator interfaces are not compiled by this crate. Enabling them requires
a separate decision, host-JIT smoke test, and size/latency measurement protocol.
MIR RISC-V generation is a later experiment and cannot be inferred from the
LP64 QEMU probe.
