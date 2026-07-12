# DEC-030 - Minimal typed AST expressions

## Status

Accepted on 2026-07-12 (`expr-B`) as an experimental, reversible decision.

## Decision

Replace structural expression strings in the two experimental algorithm ASTs
with a minimal typed expression tree. Version 0 supports variables, integer and
boolean constants, sequence length, indexing, ranges, index arithmetic,
comparisons, boolean conjunction and negation, and abstract calls.

AST validation resolves variables, checks operand and condition types, tracks
bindings and predicate results, and rejects writes to parameters without a
`ReadWrite` or `Output` mode. Human descriptions remain on semantic operations
but do not determine their operands or control flow.

## Consequences

The renderer remains backend-independent while structural accesses can now be
inspected without parsing prose. Abstract calls declare their result type in
experiment version 0; a signature environment is deferred until a third real
call shape demonstrates the necessary model. No evaluator, persistent format,
public schema, or general-purpose language is introduced.
