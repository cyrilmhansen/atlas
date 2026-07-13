# DEC-045 - Use private guest pairs to observe insertion stability

## Status

Accepted on 2026-07-13 (`insertion-B`).

## Decision

Exercise insertion-sort shifted writes with a private 16-byte guest element:

- signed little-endian `i64 key` at offset 0;
- little-endian `u64 original_index` bits at offset 8.

The MIR program compares only `key`. It keeps the current pair in registers,
shifts larger preceding pairs one element to the right, and writes the complete
pair at its insertion position. A strict comparison preserves input order for
equal keys. Native Rust insertion sort remains the correction oracle.

## Consequences

- Stability is directly observable rather than inferred from indistinguishable
  scalar duplicates.
- The pair layout is private test instrumentation, not a public element model,
  registry schema, guest ABI or persistent format.
- The experiment reuses the single bounded `GuestOffset(u32)` region and its
  little-endian aligned `i64` accessors. It does not change DEC-040.
- This demonstrates shifted writes for one specialized value shape; it does not
  introduce generic MIR sorting or comparison callbacks.
- The later host-JIT extension uses the exact same pair encoding and correction
  oracle at MIR optimization levels 0 through 3; it does not promote the layout
  to a stable format.
