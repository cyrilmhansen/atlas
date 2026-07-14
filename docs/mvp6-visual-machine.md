# MVP 6 private visual machine

This document describes the current executable presentation experiment. It is
an implementation note, not a public schema, ABI, evidence format or persistent
protocol. DEC-061 through DEC-063 define its scope.

## Authority and generation

The minimum, partition, `is_sorted`, insertion and reverse pseudocode sources
parse to the same typed `AlgorithmAst` values as their Rust AST builders. Each
specialized compiler accepts only its exact reviewed AST shape and emits
`atlas-visual-bytecode-private-v0`. Bundle generation validates every register
and jump target and checks that each semantic instruction names an existing AST
node with the matching operation kind.

The generated program is embedded in the derived Web projection. The browser
passes its JSON representation to `VisualMachine`; it is never read from the
registry or retained as execution evidence. Native `minimum_by`,
`partition_in_place` and `is_sorted_by`, plus the retained insertion and reverse
steppers, remain correction oracles.

## Current machine

The machine owns one `i32` sequence, at most 16 index registers, a program
counter, the last comparison/predicate results, an optional result index and
structural counters. Original indices are allocated lazily on the first swap;
an unchanged identity is only produced when requested for display. Minimum uses
two registers and nine instructions; even partition uses two registers and 19
instructions; insertion uses two registers and 13 instructions.
Symmetric reverse uses two registers and 11 instructions.

| Instruction | Effect |
|---|---|
| `halt_if_empty` | returns no index for an empty sequence |
| `branch_index_less_than_length` | selects a validated in-program target |
| `branch_index_less_than_index` | compares two index registers for two-pointer control |
| `branch_predicate` | branches on the last intrinsic predicate result |
| `branch_if_greater` | branches on an adjacent inversion comparison |
| `branch_if_less` | branches on the last strict less-than comparison |
| `branch_register_non_zero` | guards a checked previous-index operation |
| `set_register_to_length` | initializes an index from sequence length |
| `read` | bounds-checks one indexed read and exposes its exact AST node |
| `read_previous` | reads the checked index immediately before a register |
| `predicate_even` | evaluates `i32 is_even` and exposes its exact AST node |
| `compare_less` | compares two indexed values and exposes its exact AST node |
| `compare_greater` | compares direct/previous indices for an inversion |
| `compare_less_previous` | compares a direct index with its predecessor |
| `copy_if_less` | conditionally updates an index register |
| `copy_register` | copies one validated index register to another |
| `increment` / `decrement` | checked index movement |
| `swap_previous` | mutates values and origins and exposes its exact AST node |
| `swap_registers` | swaps two checked register-selected values and origins |
| `jump` | transfers to a validated in-program target |
| `return_optional_index` | bounds-checks and returns the selected index |
| `return_none` | completes without a selected or inversion index |
| `return_index` | returns a boundary and exposes its exact AST node |

Control instructions execute internally until the next semantic read,
predicate, comparison, swap or boundary operation. One UI step therefore
denotes one visible semantic operation, not one bytecode instruction. The
machine retains only current state and the last operation; seeking backward
resets and deterministically re-executes it. No presentation trace is
materialized.

For `sequence.minimum`, comparison is strict. Equal values do not replace the
selected index, so the first minimum is retained. Empty input returns `None`.
For the partition experiment, `predicate_even` is a provisional class-B
intrinsic. It avoids introducing callbacks or a call ABI while exercising
mutation. Dataset selection exposes and restricts the concrete predicate.
Insertion copies the outer index into a current index, then uses strict adjacent
comparisons and swaps. Equal values never move past one another; lazy origin
tracking makes stability and permutation directly checkable.
Reverse initializes its right index from sequence length, exposes two semantic
reads per pair and counts the two writes performed by each symmetric swap.

## Bounds

- program JSON: 32 KiB maximum;
- instructions: 1 to 256;
- registers: 1 to 16;
- sequence: 4096 signed 32-bit values for aggregate local execution;
- interactive playback: 64 values, enforced by the projected dynamics bound;
- control transitions between visible operations: 256 maximum;
- visible semantic operations per execution: `256 * max(sequence length, 1)`;
- network, allocation instructions, calls, recursion and additional memory
  regions: absent.

The local timing shown by the browser includes JSON program parsing, machine
construction and execution. It is explicitly not algorithm-only or portable
benchmark evidence. Scale views use exact semantic counts over the complete
input and keep sourced asymptotic claims separate.

## Validation and current limit

Rust and Node tests cover empty, singleton, ordinary and tied-minimum cases,
plus empty, all-matching, none-matching, mixed and alternating partitions,
sorted, descending and duplicate-heavy insertion inputs, and even/odd reverse
inputs with involution checks.
Native mutation, boundary, permutation, counters, exact operation order,
AST-node identity, invalid targets and the 4096-element bound are checked. The
five generated paths coexist with the three hand-written MVP 5 steppers. This
differential period is intentional.

The compilers remain specialized to five reviewed AST shapes. Adjacent
`is_sorted`, stable insertion and symmetric reverse now use generated paths
while retaining their hand-written steppers as operation-for-operation oracles.
The browser now imports only `VisualMachine` and derives all five configurations
from the projection; it has no per-algorithm execution dispatch. The specialized
WASM exports remain available only to differential tests. There is no general
predicate, call, write-value or multi-region model. Removing the retained
exports requires a later explicit consolidation decision.
