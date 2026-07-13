# DEC-042 - Retain native and MIR backends indefinitely

## Status

Accepted on 2026-07-13.

## Decision

Keep the native Rust implementation of every qualified algorithm as the
long-lived reference backend. A MIR implementation is an additional,
independently exercised backend for a deliberately supported subset. Adding a
MIR backend never removes, replaces or weakens the native implementation,
native correction tests, registry evidence or composition support.

## Consequences

- Cross-backend correction compares declared outputs, mutations and applicable
  invariants on the same deterministic cases.
- A MIR trace is useful for diagnosis and AST-link validation, but remains
  private until a later decision defines durable trace evidence.
- Interpreter measurements are not compared with native performance results:
  they quantify observability and execution cost of the interpreter only.
- JIT, RISC-V generation and backend selection remain separate experiments.
- A missing MIR counterpart means "not supported by this experimental backend",
  never "unsupported by Atlas".
