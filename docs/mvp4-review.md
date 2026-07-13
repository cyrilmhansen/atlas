# MVP 4 review

Review date: 2026-07-13. Active scope: the LP64 MIR probe authorized by
DEC-039.

## Demonstrated boundary

`atlas-mir` links a private C shim to the pinned upstream MIR interpreter core.
The shim creates and executes a scalar `i64` addition, then a three-value
minimum whose two semantic comparisons call a private trace import. Rust checks
the final minimum and ordered comparison values against the native algorithm.
This is a runtime smoke test, not a public backend API or an Atlas evidence
format.

The compact-reference comparison is independent of MIR: `GuestOffset(u32)`,
`GuestHandle(u32)`, and `GuestRegionOffset` have separate testable failure
modes for arithmetic overflow, bounds, object identity and region identity.
DEC-040 selects `GuestOffset(u32)` for one fixed-capacity guest region. It is a
byte offset, zero is valid, and it is never a host pointer. No guest offset is
passed into MIR yet.

`scripts/check-rv64-lp64-abi.sh` cross-compiles a static RV64 LP64 probe and
runs it with `qemu-riscv64`. It confirms the local toolchain/emulator path and
64-bit pointer width. It does not validate RV64ILP32, bare metal/Newlib, or a
MIR RISC-V generator.

`docs/mir-integration.md` details the pinned source, build, interpreter
lifecycle, FFI boundary, reference candidates, QEMU probe, instrumentation and
JIT limits.

## Deliberate limits

- No public plan or backend schema exists.
- No Atlas plan is translated to MIR yet.
- No MIR JIT, MIR RISC-V backend, or QEMU system machine is exercised.
- RV64ILP32 is deferred; the standard LP64 ABI does not define guest-reference
  representation.
- No guest-memory load or store has been imported into MIR yet.

## Acceptance checks

```sh
cargo test -p atlas-mir --locked --offline
sh scripts/check-rv64-lp64-abi.sh
```
