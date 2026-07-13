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
public contract. It also records the effective `rustc` and `wasm-bindgen`
versions, target and profile used to construct the bundle.

`atlas-web-wasm` depends on the `no_std` native algorithm crate. The current
facade exports adjacent `is_sorted`, stable insertion sort and symmetric reverse
over signed 32-bit integers. The read-only observation reports the native
result, exact adjacent comparisons and first decreasing right-hand index. The
insertion observation reports sorted values, original indices, comparisons and
adjacent swaps. The reverse observation reports reversed values and exact
semantic reads, writes and symmetric swaps. Inputs longer than 4096 elements
are rejected before execution.

Insertion tags each value with its original index before calling the native
generic in-place algorithm. A comparison returning `Less` corresponds exactly
to the adjacent swap immediately performed by that implementation. Equal values
retain increasing original indices, which makes stability directly testable.
The algorithm itself retains its sourced `O(1)` auxiliary-space claim; tagged
input and returned arrays are explicit Web observation/transport copies.

Reverse calls the native in-place implementation, then derives its semantic
counts from that implementation's fixed loop: `floor(n/2)` swaps, with two
element reads and two element writes per swap. These are source-level algorithm
operations, not measured WebAssembly memory instructions. Exact output and a
second reversal restoring the input are checked independently in the browser
and binding tests. The returned array is an explicit Web transport copy and
does not alter the algorithm's sourced `O(1)` auxiliary-space claim.

The derived projection also materializes the five deterministic cases from
`dataset.sequence.sort.m2.v0`. Each browser choice carries its original spec,
case, problem, class, seed, values and canonical content digest. These cases
are reused by `is_sorted` and `reverse` because their integer-sequence input
shape is compatible; they remain attributed to `sequence.sort` and are not
presented as problem-specific specifications for those operations. Edited input
is immediately marked as custom, ephemeral and without registry evidence.

The browser timing is calibrated over a bounded repeated batch and includes the
JavaScript/WebAssembly call boundary and observation allocation. The display
records repetitions, elapsed batch duration and browser identity. It is neither
algorithm-only timing nor portable benchmark evidence.

## Current limits

- The static bundle is built and tested locally but not published.
- Projection JSON and generated bindings are ignored build products.
- MIR, target code and semantic trace playback are not executed in the browser.

## Reproducible bundle gate

The complete bundle has been rebuilt from a Git archive containing no `.git`,
`target` or ignored build products. With the source commit supplied explicitly,
`scripts/check-web.sh` reproduced identical SHA-256 hashes for all eight output
files: HTML, CSS, JavaScript, projection JSON, WASM, JavaScript bindings and both
TypeScript declarations. See `docs/mvp5-review.md` for the exit audit.
