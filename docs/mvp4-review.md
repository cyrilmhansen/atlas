# MVP 4 review

Review date: 2026-07-13. Closed scope: the single-region interpreter/JIT and
RV64 generator experiment authorized by DEC-039 through DEC-051 and closed by
DEC-052.

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
DEC-050 adds adjacent `is_sorted` with a checked little-endian offset load and
inspects its 128-byte generated code. None validates RV64ILP32 or bare
metal/Newlib. DEC-051 adds checked little-endian stores through a 176-byte
generated `reverse` function.

## Deliberate limits

- No public plan, backend schema or persistent MIR artifact exists.
- Only private specialized programs exist. Partition and `is_sorted` link
  traces to exact AST nodes; the other experiments are not AST lowerings.
- Host JIT is exercised only for correction and structural inspection. RV64
  covers scalar generation plus read/write guest imports over one region, but
  no timed JIT measurement, multiple regions or QEMU system machine is
  exercised.
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

The workspace tests retain native/interpreter/JIT correction equivalence. The
RV64 script separately cross-compiles MIR itself, executes generated scalar,
read-only and mutating code under QEMU, checks exact results and import counts,
and inspects the temporary target instructions.

## Exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| Keep knowledge semantics independent of MIR | Complete | Registry and composition crates have no MIR dependency |
| Preserve a native correction backend | Complete | Every MIR algorithm experiment compares with native Rust |
| Distinguish host pointers and guest references | Complete | `GuestOffset(u32)` plus independent handle/region candidate tests |
| Exercise visible guest reads, writes and effects | Complete | Checked imports, AST traces, reverse and stable shifted-pair tests |
| Validate MIR interpretation and generation | Complete | Interpreter, host JIT levels 0-3 and observed machine-code spans |
| Validate the selected RV64 target | Complete | LP64D MIR generator under QEMU with scalar, `is_sorted` and `reverse` |
| Preserve narrow, reversible scope | Complete | No public backend schema, persistent artifact, allocator or machine profile |

## Closure impact

DEC-052 closes MVP 4 without promoting the private shim, guest imports,
generated bytes or experimental AST lowering to public contracts. Native Rust
remains authoritative and MIR remains optional under DEC-042. Multi-region
memory, guest allocation, RV64ILP32, system emulation, performance ranking and
persistent target evidence require a separately activated future MVP.
