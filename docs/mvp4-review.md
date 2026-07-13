# MVP 4 single-region checkpoint review

Review date: 2026-07-13. Active scope: the single-region interpreter/JIT
checkpoint and standalone RV64 generator probe authorized by DEC-039 through
DEC-049. This is a checkpoint, not MVP 4 closure.

## Demonstrated boundary

`atlas-mir` links a private C shim to the pinned upstream MIR interpreter core.
It now exercises scalar arithmetic, bounded trace imports, guest reads,
comparisons, stores, swaps and shifted writes. Native Rust remains the oracle
for every algorithm-level experiment.

| Capability | MIR experiment | Cross-backend evidence |
|---|---|---|
| Scalar arithmetic and trace import | addition, three-value minimum | exact result, event order and first-on-tie behavior |
| AST-backed reads, predicates and swaps | even partition | output, boundary and exact typed AST trace links |
| AST-backed read-only scan | adjacent `is_sorted` | boolean, first inversion, early stop and typed AST trace links |
| Selection scan | minimum and maximum | value, index and first-occurrence tie policy |
| Symmetric swaps | reverse | exact output and double-reversal property |
| Shifted writes over private 16-byte pairs | stable insertion sort | exact native output, sortedness, permutation and duplicate stability |

These entry points are private adapter experiments. They are not registry
implementations, a generic AST compiler, a backend API or Atlas evidence.

DEC-046 additionally compiles MIR's host generator. Generated scalar addition,
guest `is_sorted`, reverse, partition and stable insertion reproduce interpreter
and native results. Exact spans and instruction shapes are local structural
observations, not performance claims.

The compact-reference comparison remains independent of MIR:
`GuestOffset(u32)`, `GuestHandle(u32)`, and `GuestRegionOffset` have separate
testable failure modes for overflow, bounds, object identity and region
identity. DEC-040 selects `GuestOffset(u32)` for one fixed-capacity region.
Offset zero is valid and is never a host pointer. MIR computes scalar byte
offsets; private host imports alone access the backing buffer.

Signed `i64` elements occupy 8 bytes. DEC-045's stability probe uses a private
16-byte `(i64 key, u64 original_index)` pair without promoting that layout to a
guest ABI.

`scripts/check-rv64-lp64-abi.sh` cross-compiles a static RV64 LP64 probe and
runs it with `qemu-riscv64`. It confirms the Linux toolchain/emulator path and
64-bit pointer width. DEC-049 separately cross-compiles and runs MIR's generator
itself, verifies a generated scalar addition and inspects its 16-byte RV64 code.
Neither probe validates RV64ILP32 or bare metal/Newlib.

## Deliberate limits

- No public plan, backend schema or persistent MIR artifact exists.
- Only private specialized programs exist. Partition and `is_sorted` link
  traces to exact AST nodes; the other experiments are not AST lowerings.
- Host JIT is exercised only for correction and structural inspection. The
  scalar MIR RV64 backend is exercised, but no RV64 guest-memory import, timed
  JIT measurement or QEMU system machine is exercised.
- RV64ILP32 is deferred; the standard LP64 ABI does not define guest-reference
  representation.
- Guest memory has one fixed-capacity little-endian region with private 8-byte
  accesses. It has no guest allocation, separate output/scratch region, region
  lifecycle or multi-region model.
- Textual pseudocode remains a two-file, test-only parser experiment and is not
  the source consumed by the MIR adapter.

## Acceptance checks

```sh
cargo test -p atlas-mir --locked --offline
cargo test --workspace --all-targets --locked --offline
cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings
scripts/check-mvp2.sh
scripts/check-rv64-lp64-abi.sh
scripts/check-mir-rv64-generator.sh
```
