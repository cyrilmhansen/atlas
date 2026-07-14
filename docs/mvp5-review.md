# MVP 5 review

Review date: 2026-07-14. This records the extended reproducible local-artifact
audit authorized by DEC-053 through DEC-059. Editable generation,
operation-growth views and bounded incremental semantic dynamics are complete;
DEC-060 accepts this audit and closes MVP 5 at the demonstrated boundary.

## Demonstrated boundary

The artifact is a static bundle derived from the Git-authoritative YAML registry,
the native `no_std` Rust algorithms and accepted DatasetSpec cases. It has no
application server, account, database service, remote execution or runtime
network dependency. `atlas-web-private-v0` remains a disposable build projection,
not a public or persistent schema.

| Workload | Native/WASM evidence | Characteristic boundary |
|---|---|---|
| Adjacent `is_sorted` | exact boolean, first inversion and early-stop comparisons | read-only input; JS/WASM call timing |
| Stable insertion | sorted permutation, duplicate stability, comparisons and adjacent swaps | algorithm `O(1)` space separated from tagged transport copies |
| Symmetric reverse | exact output, involution, semantic reads/writes and swaps | algorithm `O(1)` space separated from returned output copy |

All three workloads expose parser-equivalent pseudocode and incremental WASM
state through 64-element Explore inputs. Current operations link to exact typed
AST nodes. The browser stores no execution trace or snapshot sequence;
analytical traces remain private test oracles where they already exist.

The catalog projects all 10 problems, 15 algorithms and 20 implementations with
their claims and provenance. It carries the logical registry digest and source
commit. The five cases of `dataset.sequence.sort.m2.v0` retain spec, problem,
class, seed, values and canonical content digest. Their compatible reuse by
`is_sorted` and `reverse` is explicitly attributed to `sequence.sort`.

The bundle records the effective `rustc` and `wasm-bindgen` versions, target and
profile. The browser separately displays its runtime identity with locally timed
calls. Sourced asymptotic complexity, deterministic semantic counts and local
wall-clock observations occupy distinct fields and make no cross-environment
performance claim.

## Reproducibility evidence

`scripts/check-web.sh` validates the registry projection, compiles the release
WASM facade, generates Web and Node bindings, and runs native/WASM fixtures for
all three workloads. CI runs this gate after installing the pinned
`wasm-bindgen-cli` 0.2.100 and WASM target.

The same gate passes from a Git archive containing no repository metadata,
Cargo target directory or ignored Web output when `ATLAS_SOURCE_COMMIT` is
supplied explicitly. At source commit
`75ceb69f2eca70a957955059143bbbaa3048383e`, a recursive byte comparison and
sorted SHA-256 manifest match for all ten files:

- `app.js`, `generator.mjs`, `playback.mjs`, `index.html` and `styles.css`;
- `data/atlas.json`;
- `pkg/atlas_web.js` and `pkg/atlas_web.d.ts`;
- `pkg/atlas_web_bg.wasm` and `pkg/atlas_web_bg.wasm.d.ts`.

```text
5646683ce4a7549504a64ac79dbd1895e3c6c362a169771d918b6a68086d0f14  app.js
e76485b67733bfaf3f4a7216f67a2fc2680fd2d3cae043c1b5709607215961d6  data/atlas.json
aac3819ad6b13242cc47bce558b70a6d5011149433e2a094ff7697bb0e262590  generator.mjs
35938e4360ec1992713b2ee2d7a9bf49e3d23519b30235e29574ca62969c5881  index.html
a6c0b2bfce9ef0dc09660efbd562150b1e15e1f039b96777ac9ef5926292de38  pkg/atlas_web.d.ts
a93e4689e8ab6e52b70c72cad0c776ad3c729a6097c8b0448cc7d8f4bad5a5a0  pkg/atlas_web.js
544832f49c6ccea5ea1c3ba532a344cd590766d2e98c3bb9f57ea2dba736e0bc  pkg/atlas_web_bg.wasm
c4691ff81b22508fb3cac0285fe64018a29356a2c36c80bf66d64a46f802f05a  pkg/atlas_web_bg.wasm.d.ts
723c6dbc335df5a06bc982b749d61a0caebd075e7710511ca867df2b02af1764  playback.mjs
79bed1f53f39896c423e2dc9f0a201f49f1b6e18f408af160e372835bb227064  styles.css
```

## Deliberate limits

- The site is local and unpublished. A GitHub Pages or release deployment needs
  an explicit publication decision.
- DatasetSpec, projection and WASM facade shapes remain private experimental
  boundaries rather than stable browser APIs.
- Custom inputs are bounded to 4096 signed 32-bit integers, ephemeral and never
  registry evidence.
- The local timer includes observation construction and the JavaScript/WASM
  boundary. It is not algorithm-only or portable benchmark evidence.
- Semantic trace playback, MIR execution, RV64 emulation, cancellation and a
  general algorithm runtime are excluded. Bounded incremental semantic
  execution is included; traces remain analysis/test instruments.

## Exit criteria

| Criterion | Status | Evidence |
|---|---|---|
| Build static bundle from clean source | Complete | archive build with no `.git`, `target` or prior output |
| Preserve registry authority and provenance | Complete | source commit, logical digest and sourced catalog claims |
| Execute three curated native algorithms locally | Complete | binding fixtures plus browser correction/invariant checks |
| Separate theory, counts and timing | Complete | distinct sourced, deterministic and runtime-qualified fields |
| Expose build and runtime environments | Complete | tool versions/target/profile plus browser identity |
| Validate the bundle in CI | Complete | `Run MVP 5 Web acceptance slice` workflow step |
| Keep publication explicit | Complete | no deployment workflow or external publication |

## Closure record

DEC-060 preserves the current artifact as a reproducible local
demonstration, without promoting its private projection or WASM facade to stable
contracts. Publication, problem-specific datasets for all workloads, explicit
control-flow AST identity and any broader browser runtime should be selected in
a separately activated scope. No subsequent MVP is active.
