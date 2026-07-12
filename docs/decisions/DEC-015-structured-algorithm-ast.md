# DEC-015 - Structured algorithm AST

- Status: accepted, implementation deferred
- Class: C
- Date: 2026-07-11

## Decision

Represent algorithms, in addition to prose and implementations, with a small
backend-independent structured pseudocode AST. Its intended vocabulary covers
control flow, abstract variables and types, reads, writes, comparisons, swaps,
calls, allocations, copies, conversions, effects, invariants, and assertions.

## Consequences

Rust remains an implementation language rather than the normative algorithm
description. Cap'n Proto may later encode the AST but will not define its
semantics. MIR remains an execution adapter. No public AST schema is introduced
until at least two current algorithms have been modeled experimentally.
