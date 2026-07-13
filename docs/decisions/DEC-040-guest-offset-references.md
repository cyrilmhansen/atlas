# DEC-040 - Select bounded guest offsets for MVP 4

## Status

Accepted on 2026-07-13 (`refs-offset-A`).

## Decision

The first MVP 4 guest-memory experiment represents every guest reference as a
`GuestOffset(u32)`: an unsigned byte offset from the base of one bounded guest
region. The region is created at a fixed size and has no guest-visible growth
operation. `0` is a valid offset; MVP 4 defines no null guest reference.

Every typed access introduced later must state its required alignment and prove
both arithmetic-overflow and region-bound checks before dereferencing the host
buffer. The reference is valid only while its owning host-side guest region is
live. It is neither a host pointer, a serialized address, nor a cross-process
capability.

## Alternatives rejected for this first experiment

- `u32` handles require an object table, reuse and lifetime policy before the
  first sequence access.
- Region-plus-offset references require region identity and lifecycle semantics
  that one contiguous fixed-capacity experiment does not exercise.

## Consequences

- MIR receives offset values as scalar integers only; host pointers stay in the
  private runtime shim.
- The selected model is not public schema, execution evidence or a stable ABI.
- A later multi-object runtime may replace it only through a new decision and
  migration of any private adapter code that depends on it.
- The next implementation gate is one bounds-checked guest-memory operation,
  followed by a separately accepted AST-to-MIR lowering experiment.
