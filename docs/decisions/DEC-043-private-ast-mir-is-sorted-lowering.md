# DEC-043 - Lower a private read-only is-sorted AST subset to MIR

## Status

Accepted on 2026-07-13 (`is-sorted-ast-mir-B`).

## Decision

Add `is_sorted_ast()` for `order.is_sorted.adjacent` and lower its explicit
adjacent reads and signed `i64` comparison to a private MIR interpreter
program. The native Rust `is_sorted_by` implementation remains the correction
oracle. No public backend trait, registry property, plan format or execution
artifact is added.

The lowering reuses the bounded little-endian `GuestOffset(u32)` byte region
from DEC-040 and DEC-041. It performs no guest write or allocation. It returns
the sorted boolean and, when false, the index of the right-hand element of the
first inverted pair. Thus `[3, 2]` returns `false` and `Some(1)`.

The private trace import records the left read, right read and adjacent
comparison. Rust maps each event to an exact AST node ID and semantic kind, and
tests validate node existence and kind against `is_sorted_ast()`.

## Consequences

- The trace is bounded to 128 in-memory events and reports truncation; it is
  not Atlas evidence or a persistent artifact.
- The lowering stops after the first inversion, matching the native algorithm's
  observable behavior for the selected signed `i64` specialization.
- Empty and singleton inputs perform no reads or comparisons.
- This is a second specialized lowering, not a generic AST compiler or a
  public MIR backend API.
