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
They are not host pointers and are not yet passed into MIR.

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
- The reference comparison has not selected a winner or defined guest-object
  lifetime semantics.

## Acceptance checks

```sh
cargo test -p atlas-mir --locked --offline
sh scripts/check-rv64-lp64-abi.sh
```
