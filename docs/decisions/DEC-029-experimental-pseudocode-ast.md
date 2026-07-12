# DEC-029 - Experimental pseudocode AST shape

## Status

Accepted provisionally on 2026-07-12 as the reversible implementation of
DEC-015 (`AST-B`).

## Decision

Model structured pseudocode in memory with nested statements for bindings,
conditions, loops, iteration, operations, and returns. Typed semantic operations
carry locally unique IDs and cover reads, writes, comparisons, swaps, recursion,
allocations, copies, partitions, predicates, and assertions.

Parameters record read/write modes. Effects separately list mutations,
allocations, and copies. A renderer produces backend-independent human-readable
pseudocode. Trace event kinds must be present in the corresponding AST.
Each trace step also stores the exact AST operation-node ID that produced it.
Validation rejects an unknown node or a mismatch between the node's declared
semantic operation and the event kind.

## Consequences

The same shape describes top-down merge sort and in-place two-pointer partition,
which are materially different algorithms. DEC-030 replaces structural strings
with minimal typed expressions. The AST still has no persistent serialization,
public schema, MIR mapping, or normative execution semantics.
