# DEC-054 - Disposable Web projection and curated WASM facade

## Status

Accepted on 2026-07-13 (`boundary-A`).

## Decision

Generate the browser data from the validated Git-authoritative YAML registry
instead of parsing the authoritative registry directly in the browser. Expose
algorithm execution through a narrow, curated WASM facade rather than the full
internal Rust API.

The derived projection must identify its source commit and logical registry
digest. It is disposable build output and is not a source of knowledge or
execution evidence.

## Consequences

- The registry, validator and native Rust APIs can evolve without accidentally
  becoming Web contracts.
- Projection generation and WASM calls require native-equivalence and stale
  digest tests before they can be shown publicly.
- The initial projection format is private and reversible. Making it a stable
  public interface requires a new class C schema and compatibility decision.
- This decision does not define a general algorithm ABI or activate MVP 5.
