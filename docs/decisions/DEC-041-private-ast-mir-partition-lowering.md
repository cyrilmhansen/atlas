# DEC-041 - Lower a private partition AST subset to MIR

## Status

Accepted on 2026-07-13 (`ast-mir-B`, `endian-LE-A`).

## Decision

Lower the explicit read, predicate, swap and boundary subset of the existing
`partition_ast()` to a private MIR interpreter program. The native Rust
`partition_in_place` implementation remains the correction oracle. No public
backend trait, plan format, registry property or execution artifact is added.

The guest region selected by DEC-040 stores `i64` values in little-endian byte
order. Each value starts at an offset divisible by eight. MIR receives only the
element count and computes `GuestOffset(u32)` byte offsets; it never receives
the host pointer to the backing byte buffer.

The shim exposes three private imports:

- `atlas_mir_guest_load_i64(offset) -> i64`;
- `atlas_mir_guest_store_i64(offset, value)`;
- `atlas_mir_record_partition_operation(event_code)`.

The first predicate specialization is signed `i64` evenness. Each trace code is
converted by Rust to one exact AST node ID and semantic operation kind. Tests
validate both against `partition_ast()`.

## Consequences

- The adapter uses a bounded 128-event in-memory trace and reports truncation;
  it never turns the trace into evidence or a persistent artifact.
- The ABI is private to `atlas-mir`, process-local and serialized by its Rust
  adapter lock. It is not a guest ABI, RISC-V ABI or MIR public interface.
- Big-endian guest data is deferred. QEMU system-mode supports data big-endian
  CPUs, but the current user-mode LP64 probe and Linux sysroot do not provide a
  reproducible RV64 big-endian execution path.
- A later endian change requires a new decision and an emulator/toolchain probe.
