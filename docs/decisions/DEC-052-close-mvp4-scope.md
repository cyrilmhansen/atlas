# DEC-052 - Close MVP 4 at the single-region MIR/RV64 checkpoint

## Status

Accepted on 2026-07-13 (`close-mvp4-A`).

## Decision

Close MVP 4 with the private execution boundary demonstrated under DEC-039
through DEC-051:

- the original upstream MIR revision remains pinned and locally patched only
  for synchronous generated-code observation;
- native Rust remains the correction and qualification authority;
- the MIR interpreter covers scalar traces and one bounded little-endian
  `GuestOffset(u32)` region with reads, writes, scans, swaps and shifted pairs;
- exact trace-to-AST links are validated for partition and `is_sorted`;
- the host JIT reproduces scalar, read-only and mutating correction cases at
  explicit optimization levels, with local x86-64 code inspection;
- standard RV64GC LP64D generation runs under QEMU user mode for scalar
  addition, checked `is_sorted` reads and checked `reverse` writes.

MVP 4 does not add a public backend interface, persistent MIR or machine-code
format, backend selection, general AST compiler, C-to-MIR pipeline, multi-region
memory, guest allocation, RV64ILP32 ABI, system-emulated fantasy computer,
performance ranking or archived target execution.

## Consequences

- MIR remains an additive experimental adapter; registry, composition and
  native algorithm operation do not depend on it.
- The accepted runtime semantics stop at one fixed-capacity host-owned region.
  Any region table, handles, lifetime or allocation policy requires a new class
  C decision.
- QEMU user mode proves generated RV64 code and target imports, not devices,
  boot, console, clock or a complete machine profile.
- Exact generated spans and instruction shapes are diagnostic observations,
  not performance evidence or stable artifacts.
- Future work may retain both native and MIR backends indefinitely under
  DEC-042, but a subsequent MVP must be activated explicitly before widening
  this scope.
