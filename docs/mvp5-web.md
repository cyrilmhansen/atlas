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
facade exports adjacent `is_sorted` and stable insertion sort over signed 32-bit
integers. The read-only observation reports the native result, exact adjacent
comparisons and first decreasing right-hand index. The insertion observation
reports sorted values, original indices, comparisons and adjacent swaps.
Inputs longer than 4096 elements are rejected before execution.

Insertion tags each value with its original index before calling the native
generic in-place algorithm. A comparison returning `Less` corresponds exactly
to the adjacent swap immediately performed by that implementation. Equal values
retain increasing original indices, which makes stability directly testable.
The algorithm itself retains its sourced `O(1)` auxiliary-space claim; tagged
input and returned arrays are explicit Web observation/transport copies.

The browser timing is calibrated over a bounded repeated batch and includes the
JavaScript/WebAssembly call boundary and observation allocation. The display
records repetitions, elapsed batch duration and browser identity. It is neither
algorithm-only timing nor portable benchmark evidence.

## Current limits

- `reverse` is not exported yet.
- Dataset choices are local UI fixtures, not yet generated from `DatasetSpec`.
- The static bundle is built and tested locally but not published.
- Projection JSON and generated bindings are ignored build products.
- MIR, target code and semantic trace playback are not executed in the browser.
