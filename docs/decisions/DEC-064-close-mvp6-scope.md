# DEC-064 - Close MVP 6 at the generated visual-machine boundary

## Status

Accepted on 2026-07-14 (`close-mvp6-A`).

## Decision

Close MVP 6 at the boundary audited in `docs/mvp6-review.md`:

- five reviewed sequence algorithms compile from exact typed ASTs to bounded
  private programs and execute through one browser `VisualMachine`;
- `web/app.js` imports no algorithm-specific observation or stepper and contains
  no active-algorithm execution dispatch;
- projected presentation data selects programs, datasets, labels, counters and
  bounded result views;
- native observations and the three MVP 5 steppers remain private WASM exports
  solely as differential test oracles.

Stage 5's no-specialized-export criterion is narrowed to the production browser
entry point. Test-only exports are accepted because they preserve independent
JavaScript/WASM boundary checks and are not alternate browser execution paths.

## Evidence

- all five generated programs match native or retained reference behavior;
- exact semantic operations retain their typed AST node identifiers;
- the consolidated-browser gate rejects specialized imports and execution
  dispatch in `web/app.js`;
- the ten-file bundle is byte-identical when rebuilt from a clean Git archive at
  `75c4cea54be8c4a7f7f86259b201ad475c9897db`;
- Chrome inspection passes at 390, 768, 1440 and 1920 pixel widths.

## Consequences

- MVP 6 is closed without stabilizing the bytecode, projection, presentation or
  WASM interfaces.
- Removing the test-only exports remains a reversible cleanup option, not an
  MVP 6 exit requirement.
- The five specialized AST lowerings are accepted evidence for the bounded
  approach, not a claim of a general AST compiler.
- General calls, allocation instructions, multiple memory regions, arbitrary
  predicates, MIR-in-browser execution and publication remain outside scope.
- No subsequent MVP is activated by this decision. Performance fingerprints
  remain a candidate future scope based on `docs/performance-model-research.md`.
