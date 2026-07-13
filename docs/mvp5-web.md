# MVP 5 Web artifact

The active MVP 5 slice is a static bundle built from the Git-authoritative Atlas
registry and native Rust algorithms. DEC-053 through DEC-057 define its scope
and private browser boundary.

## Build and run

Prerequisites are the `wasm32-unknown-unknown` Rust target, Node.js and
`wasm-bindgen-cli` 0.2.100.

```sh
scripts/build-web.sh
python3 -m http.server 4173 --directory build/web
```

Open `http://127.0.0.1:4173/`. The HTTP process is a static-file server, not an
Atlas application server. The bundle performs no network access after its own
files have loaded.

Run the complete first-slice acceptance check with:

```sh
scripts/check-web.sh
```

## Authority boundary

`build/web/data/atlas.json` is generated from the validated aggregate YAML. It
records the source commit and the same logical digest used by the SQLite
projection. Its `atlas-web-private-v0` shape is disposable and unversioned as a
public contract.

`atlas-web-wasm` depends on the `no_std` native algorithm crate. The current
facade exports only adjacent `is_sorted` over signed 32-bit integers. Its typed
observation reports the native result, the exact number of adjacent comparisons
and the first decreasing right-hand index. Inputs longer than 4096 elements are
rejected before execution.

The browser timing is calibrated over a bounded repeated batch and includes the
JavaScript/WebAssembly call boundary and observation allocation. The display
records repetitions, elapsed batch duration and browser identity. It is neither
algorithm-only timing nor portable benchmark evidence.

## Current limits

- Stable insertion and `reverse` are not exported yet.
- Dataset choices are local UI fixtures, not yet generated from `DatasetSpec`.
- The static bundle is built and tested locally but not published.
- Projection JSON and generated bindings are ignored build products.
- MIR, target code and semantic trace playback are not executed in the browser.
