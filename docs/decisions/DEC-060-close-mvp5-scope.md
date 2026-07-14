# DEC-060 - Close MVP 5 at the reproducible local Web artifact

## Status

Accepted on 2026-07-14 (`close-mvp5-A`).

## Decision

Close MVP 5 with the extended local interactive artifact demonstrated under
DEC-053 through DEC-059:

- the static catalog is derived from the Git-authoritative registry and carries
  its source commit, logical digest and provenance;
- adjacent `is_sorted`, stable insertion and symmetric reverse execute locally
  through private Rust/WASM facades and match native correction checks;
- all three algorithms expose parser-equivalent pseudocode and bounded,
  incremental, AST-linked WASM state without retaining presentation traces;
- editable and seeded generated inputs separate an Explore regime from a Scale
  regime;
- sourced complexity, exact semantic counts and qualified local timing remain
  visibly distinct;
- the ten-file bundle is byte-reproducible from a clean Git archive at
  `75ceb69f2eca70a957955059143bbbaa3048383e`.

MVP 5 does not stabilize the private Web projection, WASM facade, pseudocode or
stepper interfaces. It does not publish or deploy the site, execute MIR in the
browser, define a general algorithm runtime, or archive custom executions as
registry evidence.

## Consequences

- The artifact is accepted as a reproducible local demonstration, not as a
  stable browser platform or public protocol.
- Analytical traces remain validation and analysis instruments; browser
  presentation retains only current incremental execution state.
- Publication, broader DatasetSpec coverage, explicit control-flow AST identity
  and any general browser execution boundary require separately accepted scope.
- No MVP 6 or other implementation phase is activated by this closure. The next
  MVP must define its purpose and exit criteria explicitly.
