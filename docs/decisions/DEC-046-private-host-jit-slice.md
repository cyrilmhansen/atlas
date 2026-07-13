# DEC-046 - Enable a private host-JIT correction slice

## Status

Accepted on 2026-07-13 (`jit-A`).

## Decision

Compile upstream `mir-gen.c` into `atlas-mir` and use MIR's default host code
allocator for two private correction probes:

- scalar `add_u64`, without imports;
- adjacent signed-`i64` `is_sorted`, using the existing checked guest-load
  import over one bounded little-endian offset region.

Each call creates a MIR context and module, initializes the generator, links
with `MIR_set_gen_interface`, invokes the generated function, calls
`MIR_gen_finish`, then destroys the MIR context. Rust compares JIT results with
the interpreter and the native reference implementation before any timing is
considered.

## Consequences

- The native build now includes MIR's host generator and executable-code
  allocation. The C-to-MIR compiler remains excluded.
- The interpreter remains the trace and observability backend. JIT execution
  adds no persistent evidence, backend selection or registry implementation.
- The guest JIT probe proves that generated code can call the same private
  bounds-checked host import as the interpreter.
- Construction latency, generated-code size and execution timing remain
  unmeasured. They require a separate local protocol and must not be mixed with
  native algorithm benchmarks.
- Allocation or protection failures are currently governed by upstream MIR;
  Atlas does not yet translate them into structured Rust errors.
