# DEC-056 - Activate MVP 5 as a local static interactive artifact

## Status

Accepted on 2026-07-14 (`mvp5-A`).

## Decision

Activate MVP 5 as one bounded vertical slice of the public-artifact roadmap.
The exit artifact is a locally openable static bundle containing:

- a read-only catalog generated from the validated registry;
- curated local WebAssembly execution for adjacent `is_sorted`, stable
  insertion sort and in-place `reverse`;
- sourced time and auxiliary-space complexity;
- deterministic operation counters whose boundaries are explicit and tested;
- optional local browser timing, visibly separated from theory and counters.

The first implementation experiment is native/WebAssembly result equivalence
for `is_sorted` over the same deterministic Atlas datasets. Mutation, counters
and timing follow only after that gate passes.

## Exit criteria

- A clean checkout builds a static bundle without an application server.
- The derived catalog carries its source commit and logical registry digest and
  reproduces exact registry entity counts and references.
- The three selected algorithms match native Rust correction fixtures and
  reject inputs beyond an explicit browser resource limit.
- Complexity claims retain provenance; operation counts and local timings
  cannot be mistaken for theoretical or portable performance claims.
- CI validates projection determinism, native/WebAssembly equivalence and the
  built bundle.

## Excluded scope

MVP 5 does not stabilize the derived projection, expose a general algorithm
ABI, add a plugin system, execute MIR in the browser, add advanced semantic
trace playback, archive browser executions or publish/deploy the artifact.

## Consequences

- DEC-053 through DEC-055 are the distribution and measurement constraints for
  this MVP.
- The projection and WebAssembly facade remain private and replaceable.
- Any stable public format, broad calling convention or external publication
  requires a separate decision.
