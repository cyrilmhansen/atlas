# DEC-053 - Static public artifact with curated Rust/WASM execution

## Status

Accepted on 2026-07-13 (`web-A`).

## Decision

Use a static website as the distribution architecture for a future public
Atlas artifact. The site will combine a read-only catalog derived from the
validated registry with a deliberately curated set of Rust algorithms compiled
to WebAssembly and executed locally in the browser.

The artifact must remain usable without an application server, account,
database service or remote algorithm execution. A release bundle is the first
distribution boundary. GitHub Pages is an optional later publication target,
not part of this decision.

## Consequences

- The browser execution surface is bounded to explicitly selected algorithms;
  this is not a plugin system or a general code-execution service.
- Resource caps, cancellation and absence of algorithm network access are part
  of the runtime acceptance boundary.
- Static hosting keeps the operational surface small and makes a downloaded
  artifact locally inspectable.
- External publication remains outside the mandate until separately approved.
- This decision selects an architecture but does not activate MVP 5.
