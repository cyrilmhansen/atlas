# MVP 6 review

Review date: 2026-07-14. This records the generic executable-presentation audit
at commit `75c4cea54be8c4a7f7f86259b201ad475c9897db`. DEC-064 subsequently accepts
this evidence and closes MVP 6 without stabilizing private interfaces.

## Demonstrated boundary

Five sequence algorithms compile from their exact reviewed typed AST to bounded
`atlas-visual-bytecode-private-v0` programs and execute through one WASM
`VisualMachine`:

| Algorithm | Generated shape | Differential evidence |
|---|---|---|
| adjacent `is_sorted` | 9 instructions, one index | exact operations, early stop, first inversion and comparisons |
| stable insertion | 13 instructions, two indices | exact operations, sorted permutation, stability, comparisons and swaps |
| symmetric reverse | 11 instructions, two indices | exact operations, origins, reads/writes, swaps and involution |
| first minimum | 9 instructions, two indices | native result, first-on-tie behavior and exact AST nodes |
| even partition | 19 instructions, two indices | native mutation, boundary, permutation, counters and exact AST nodes |

The browser imports only `VisualMachine`. Programs, labels, datasets, counters
and result-view selection come from the derived projection. The three MVP 5
steppers and observations remain exported only for native/trace/differential
tests under the accepted consolidation A; `web/app.js` does not import them.

## Reproducibility evidence

`scripts/check-web.sh` passed both in the working repository and in a `git
archive` containing no `.git`, `target` or previous bundle. Both runs used
`ATLAS_SOURCE_COMMIT=75c4cea54be8c4a7f7f86259b201ad475c9897db`.
Recursive comparison found no differing file among the ten outputs:

```text
660bf9b88b1fa275655b56de7f9484b09158bd6f2872a793b9a99d94f2522952  app.js
99f4067c7f05dfa91cf5d488ca35a55268290ff164581a63da65a1eab56ffa45  data/atlas.json
aac3819ad6b13242cc47bce558b70a6d5011149433e2a094ff7697bb0e262590  generator.mjs
35938e4360ec1992713b2ee2d7a9bf49e3d23519b30235e29574ca62969c5881  index.html
4ee1f51b7c4c3b3ed1839370b912f4aee98c0461eef6cfd912dfc3d7d2530519  pkg/atlas_web.d.ts
a90489aafc5211fac112ac87c8c30a7d19537ba97422e1d9008d55bb511ab715  pkg/atlas_web.js
c02afdcd582999db9e12f9d0146e17a0d520299dad326c39bd36249ed97e3b0e  pkg/atlas_web_bg.wasm
edcefbed704a69a7159c71509db46c707dc8f563f1813293d4539150a99efe8c  pkg/atlas_web_bg.wasm.d.ts
723c6dbc335df5a06bc982b749d61a0caebd075e7710511ca867df2b02af1764  playback.mjs
1649eecb52176eb78dc1582a74081564ce078d4123b65bc11860b53b57c8b8da  styles.css
```

## Presentation evidence

Headless Chrome inspection used an even-partition input of 32 values at
390x900, 768x1024, 1440x1000 and 1920x1080. The state remains horizontally
scrollable on mobile, forms a 4x8 grid at tablet width and two rows on desktop.
Pseudocode, state, controls and claims remain separated without page overflow.

## Exit-criteria audit

| DEC-061 criterion | Status | Evidence |
|---|---|---|
| Five algorithms use one visual machine and match native correction | Complete | Rust and Node correction/differential gates |
| Current semantic operations retain exact AST IDs | Complete | compiler validation plus operation-order tests |
| A supported algorithm needs no new WASM class/export or browser algorithm dispatch | Complete for the five reviewed shapes | minimum/partition additions and consolidated browser boundary test |
| Per-algorithm presentation stays declarative and below 50 non-test lines | Complete | five bounded `WebPresentation` values |
| Responsive execution at accepted viewports | Complete | four-width Chrome audit |
| Clean static bundle remains reproducible | Complete | clean archive gate and byte comparison |

## Deliberate limits

- The compiler is five explicit reviewed lowerings, not a general AST compiler.
- `predicate_even`, bytecode, projection and WASM interfaces remain private.
- The machine has one `i32` sequence, bounded scalar indices and no calls,
  recursion, allocation instruction or multiple memory regions.
- Result validation and a few trace phrases still switch on bounded presentation
  kinds. Generalizing these would define a broader presentation semantics and is
  not required to demonstrate generated execution.
- Specialized WASM exports remain differential test oracles. They increase test
  surface and bundle bindings but are not browser execution paths.

## Closure decision

DEC-064 accepts test-only specialized exports as part of consolidation A. They
remain independent JavaScript/WASM boundary oracles and are not imported by the
production browser entry point. Stage 5 and MVP 6 are therefore complete at the
demonstrated boundary.
