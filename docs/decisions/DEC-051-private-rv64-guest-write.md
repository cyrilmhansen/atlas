# DEC-051 - Add checked guest writes to the MIR RV64 probe

## Status

Accepted on 2026-07-13 (`rv64-write-A`).

## Decision

Extend the temporary RV64 generator probe with the existing specialized
`reverse` control flow and a private `guest_store_i64(offset, value)` import.
The store applies the same negative, alignment and single-region bounds checks
as DEC-050's load, then writes explicit little-endian `i64` bytes.

Generated `reverse` performs two checked loads and two checked stores per swap.
Empty, singleton, even and odd fixtures are compared with exact expected bytes;
reversing the even fixture twice must restore it. An inconsistent span must be
rejected by the wrapper before generated execution, leaving bytes and both
import counters unchanged.

Optimization remains level 2 under standard RV64GC LP64D and QEMU user mode.
The temporary generated code must expose four indirect calls, loop control and
a return to the cross-toolchain disassembler.

## Consequences

- This validates read/write target imports, mutation and the selected
  single-region little-endian offset semantics in generated RV64 code.
- The region is still fixed-capacity and host-owned. There is no guest-visible
  pointer, allocation, growth, second region or lifecycle protocol.
- Import state remains private, process-local and single-threaded. This is not a
  public ABI or concurrency contract.
- Exact output, involution and import counts establish correction; no timing or
  persistent execution evidence is introduced.
- Multi-region algorithms and system-emulation devices remain separate class C
  decisions.
