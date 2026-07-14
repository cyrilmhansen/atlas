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

DEC-058 adds a second deterministic local-data path. The user can edit the
sequence directly or generate `uniform`, `few_unique`, `ascending` and
`descending` profiles. By default each generation draws a new unsigned 32-bit
seed from the browser's local cryptographic random source and displays the seed.
Unchecking `Random seed` makes the field editable and reuses that fixed seed;
URL scenarios also select fixed mode. Explore sizes 8 through 64 admit bounded
animation. Scale sizes 128 through 4096 retain the existing execution limit and
show exact operation counts across multiple `n` values. Generated data remains
ephemeral and is never promoted to DatasetSpec or registry evidence.

Adjacent `is_sorted` is the first semantic-dynamics adapter. It calls the native
algorithm and records each left read, right read and comparison at its comparator
boundary. Every event exposes the exact node ID from
`ast.order.is_sorted.adjacent.v0`; Rust tests verify node existence and operation
concordance against the AST. The browser renders the already tested textual
pseudocode, active node, immutable sequence state, event detail, timeline and
reset/previous/play/next controls. Inputs above 64 elements cannot produce this
trace. The initial demonstration uses the 12-element equal DatasetSpec case so
the complete 33-event scan is visible. Inputs with an inversion still stop
immediately as the native algorithm requires; the final event explicitly labels
that early return instead of making the short trace look truncated.

Insertion sort uses the incremental model accepted in DEC-059. Its typed AST
and private textual pseudocode describe the same adjacent stable insertion used
by the native implementation. A stateful WASM stepper executes one read,
comparison or swap per `step()` call and retains only the current tagged
sequence, loop state and counters. The browser never receives a precomputed
insertion trace or a collection of snapshots. It displays the active exact AST
node and each element's original index, making both mutation and equal-value
stability visible.

The pauseable stepper is necessarily a separate implementation of the insertion
control flow: the generic native Rust function cannot yield and retain its call
frame at each semantic operation. Rust and Node tests compare every step with
the bounded analytical trace, then compare the final values, original indices,
comparisons and swaps with the native implementation. Previous-step and slider
navigation reset the WASM state and deterministically re-execute to the chosen
position. The 64-element insertion Explore bound keeps this replay bounded;
its independent analytical trace remains capped at 32. Scale execution through
4096 elements remains aggregate and trace-free.

Playback delays form a factor-two series from `0.5x` through `8x`. Changing the
selection while playing cancels the pending timeout and schedules the next WASM
step immediately with the new delay.

The Scale chart runs complete generated sequences at increasing sizes and plots
deterministic comparisons or swaps. It illustrates profile-dependent operation
growth only. It neither derives asymptotic complexity nor replaces the sourced
claim displayed above it. A scenario can be reproduced with private URL
parameters such as `?algorithm=insertion&profile=descending&size=128&seed=7`.

The browser timing is calibrated over a bounded repeated batch and includes the
JavaScript/WebAssembly call boundary and observation allocation. The display
records repetitions, elapsed batch duration and browser identity. It is neither
algorithm-only timing nor portable benchmark evidence.

## Current limits

- The static bundle is built and tested locally but not published.
- Projection JSON and generated bindings are ignored build products.
- MIR and target code are not executed in the browser; native Rust/WASM remains
  the dynamics and correction path.
- Reverse exposes Scale counts but does not yet have a validated pseudocode or
  interactive execution adapter.
- `is_sorted` still materializes a bounded analytical trace for presentation;
  migrating it to the incremental WASM model is a later internal improvement.

## Reproducible bundle gate

The DEC-057 checkpoint bundle was rebuilt from a Git archive containing no `.git`,
`target` or ignored build products. With the source commit supplied explicitly,
`scripts/check-web.sh` reproduced identical SHA-256 hashes for all eight output
files: HTML, CSS, JavaScript, projection JSON, WASM, JavaScript bindings and both
TypeScript declarations. DEC-058 adds another source module and changes the
bundle; its final clean-archive manifest must be recorded again before closure.
See `docs/mvp5-review.md` for the earlier checkpoint audit.
