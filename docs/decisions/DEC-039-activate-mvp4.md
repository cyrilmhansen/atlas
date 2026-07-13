# DEC-039 - Activate MVP 4 LP64 MIR probe

## Status

Accepted on 2026-07-13 (`mir-submodule-A`, `refs-compare-A`,
`emulator-user-A`, with RV64ILP32 deferred).

## Decision

Activate MVP 4 with an adapter experiment in `atlas-mir`. MIR is the original
`vnmakarov/mir` upstream, included as a submodule at commit
`a8ab7c31cd5f9b23b77d84c60b3d83e62d9d304c`. The adapter compiles only MIR's
interpreter core and invokes it through a private C shim; no Atlas semantic
model depends on MIR headers or types.

The guest ABI baseline is standard RV64 LP64. A cross-compiled LP64 C probe is
run under `qemu-riscv64` user mode. This validates the local compiler, ELF and
emulator path, not a fantasy computer or a MIR RISC-V backend.

RV64ILP32 is excluded from MVP 4. The local GCC probe rejects that ABI for an
RV64 target, and the ABI remains experimental. Compact guest references are
therefore compared independently as `u32` offsets, `u32` handles, and
region-plus-offset values. None is yet the runtime's selected persistent model.

## Consequences

- Host pointers, guest references, and MIR scalar values remain distinct.
- The initial adapter runs only an interpreter scalar smoke test; JIT and MIR
  RISC-V code generation require later, separately documented experiments.
- QEMU user mode is a Linux ABI probe only. Firmware, devices, console and
  clock design for a fantasy computer remain out of scope.
- A later choice of compact-reference model must use this comparison evidence
  and define allocation, lifetime, region and invalid-reference semantics.
- Schema 0.1, registry authority and the native reference backend remain
  unchanged.
