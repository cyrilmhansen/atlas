# DEC-062 - Generate private bytecode for one bounded visual machine

## Status

Accepted on 2026-07-14 (`gen-B`).

## Decision

Lower supported typed algorithm ASTs at build time to a compact private program
executed by one WASM visual machine. The initial machine contains:

- one bounded homogeneous sequence of signed 32-bit values;
- bounded scalar indices, counters, booleans and comparison results;
- structured assignment, loop, condition, return and halt control;
- semantic read, write, compare and swap operations carrying exact AST node
  identifiers.

The program is a disposable generated build artifact. It is not stored in the
registry, accepted as evidence, or exposed as a stable protocol.

## Rejected alternatives

- Direct browser interpretation of the complete AST would prematurely require
  a serialized executable AST contract.
- Generating one Rust/WASM stepper per algorithm would retain code and bundle
  growth proportional to the corpus.

## Consequences

- Program validation must reject unsupported types, effects and control shapes
  before Web output is emitted.
- The visual machine demonstrates algorithm semantics; native implementation
  equivalence must be tested separately.
- Calls, recursion, allocation, additional regions and a stable instruction set
  require later class C decisions.
