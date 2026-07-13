# DEC-048 - Embed narrow x86-64 Capstone disassembly

## Status

Accepted on 2026-07-13 (`disasm-A`).

## Decision

Use the Rust `capstone 0.14.0` binding as a diagnostic consumer of bounded MIR
code observations. Disable its default architecture set and enable only
`arch_x86`, `full` and `std` for the first experiment. `capstone-sys` compiles
its bundled C sources, so Atlas does not depend on a host Capstone package or
`pkg-config` configuration.

The initial API supports x86-64 hosts only and uses Intel syntax with addresses
relative to the start of the observed span. It stops at the first `ret` and
retains every following byte as a visible suffix. This boundary matches the two
current MIR functions but is a documented heuristic, not general control-flow
reconstruction.

The output remains process-local and is neither serialized nor added to Atlas
evidence. Enabling another architecture or defining a persistent instruction
format requires a separate decision.

## Consequences

- Scalar addition visibly lowers to `lea` plus `ret` at MIR optimization level
  2 on the current x86-64 host.
- Guest-memory `is_sorted` exposes its loop, branches, checked-load calls and
  epilogue without external tools or debug-output parsing.
- Padding and relocated import addresses after the return are not presented as
  instructions, even when x86 could decode their bytes syntactically.
- The dependency adds a C build through the `cc` crate, but no runtime shared
  library requirement.
- Capstone's decoding does not establish reachability, semantic equivalence or
  code quality. Native Rust remains the correction oracle.
- The Rust binding is MIT licensed and the embedded Capstone engine is BSD
  licensed; distribution review must retain the dependency notices.
