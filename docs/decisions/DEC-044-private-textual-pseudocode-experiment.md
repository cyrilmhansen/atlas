# DEC-044 - Run a private textual pseudocode experiment

## Status

Accepted on 2026-07-13 (`pseudo-A`).

## Decision

Add two test-only, editable structured-pseudocode fixtures for
`order.is_sorted.adjacent` and `partition.two_pointer.in_place`. A deliberately
small parser converts each fixture into the existing in-memory AST. Tests then
require structural equality with the corresponding Rust AST builder.

The syntax uses explicit headers, parameter modes, effect lists, block markers,
bindings, operations and returns. It recognizes only the expressions and
operation kinds needed by these two examples. Unsupported syntax is rejected
with a source line number.

## Consequences

- The fixtures are source-controlled research inputs, not a registry format,
  public schema, interchange protocol or normative algorithm description.
- The parser is compiled only for tests. It creates no runtime API and does not
  participate in CLI validation, execution, traces or MIR lowering.
- Structural AST equality is the target-level measure: text and current Rust
  construction must preserve all IDs, parameter modes, effects, control flow,
  expressions and operation operands.
- The meta-level measure is intentionally qualitative for this first pass:
  assess readability, supported vocabulary, parser complexity and the number of
  algorithm-specific exceptions before deciding whether a stable editing format
  is warranted.
- Initial result: the two sources are readable and exact, but the parser still
  duplicates a small expression catalog and has one dedicated
  `adjacent inversion` condition. That cost is evidence against promoting this
  syntax before broader cases reduce or justify such exceptions.
