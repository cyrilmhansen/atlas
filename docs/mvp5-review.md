# MVP 5 review

Review date: 2026-07-14. This records the reproducible local-artifact checkpoint
authorized by DEC-053 through DEC-057. DEC-058 subsequently keeps MVP 5 open for
editable generation, operation-growth views and bounded semantic dynamics; this
document is therefore a checkpoint audit rather than a closure candidate.

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
supplied explicitly. A sorted SHA-256 manifest matches for all eight files:

- `app.js`, `index.html` and `styles.css`;
- `data/atlas.json`;
- `pkg/atlas_web.js` and `pkg/atlas_web.d.ts`;
- `pkg/atlas_web_bg.wasm` and `pkg/atlas_web_bg.wasm.d.ts`.

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
  general algorithm runtime are excluded.

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

## Closure impact

Closing MVP 5 would preserve the current artifact as a reproducible local
demonstration, without promoting its private projection or WASM facade to stable
contracts. Publication, semantic dynamics, problem-specific datasets for all
workloads and any broader browser runtime should be selected in a separately
activated scope.
