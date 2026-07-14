# MVP 6 private visual machine

This document describes the current executable presentation experiment. It is
an implementation note, not a public schema, ABI, evidence format or persistent
protocol. DEC-061 through DEC-063 define its scope.

## Authority and generation

`minimum.atlas-pseudo` parses to the same typed `AlgorithmAst` as the Rust AST
builder. `compile_minimum_visual_program` accepts only that exact reviewed AST
shape and emits `atlas-visual-bytecode-private-v0`. Bundle generation validates
every register and jump target and checks that each semantic instruction names
an existing AST node with the matching operation kind.

The generated program is embedded in the derived Web projection. The browser
passes its JSON representation to `VisualMachine`; it is never read from the
registry or retained as execution evidence. Native `minimum_by` remains the
correction oracle.

## Current machine

The machine owns one `i32` sequence, at most 16 index registers, a program
counter, the last less-than result, an optional result index and structural
counters. The first generated program uses two registers and nine
instructions:

| Instruction | Effect |
|---|---|
| `halt_if_empty` | returns no index for an empty sequence |
| `branch_index_less_than_length` | selects a validated in-program target |
| `read` | bounds-checks one indexed read and exposes its exact AST node |
| `compare_less` | compares two indexed values and exposes its exact AST node |
| `copy_if_less` | conditionally updates an index register |
| `increment` | checked index increment |
| `jump` | transfers to a validated in-program target |
| `return_optional_index` | bounds-checks and returns the selected index |

Control instructions execute internally until the next semantic `read` or
`compare_less`. One UI step therefore denotes one visible semantic operation,
not one bytecode instruction. The machine retains only current state and the
last operation; seeking backward resets and deterministically re-executes it.
No presentation trace is materialized.

For `sequence.minimum`, comparison is strict. Equal values do not replace the
selected index, so the first minimum is retained. Empty input returns `None`.

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
exact operation order, AST-node identity, invalid targets and the 4096-element
bound. The generated minimum path coexists with the three hand-written MVP 5
steppers. This differential period is intentional.

The compiler is still specialized to one reviewed AST shape. Mutation, writes
and swaps are not yet implemented by the common machine. Even partition is the
next gate because it requires those effects and exercises materially different
control flow without introducing a second data domain.
