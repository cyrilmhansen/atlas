# Phase demonstration: structurally different algorithms

## Question

Can Atlas give three structurally different registered algorithms useful local
WASM experiences without changing the public schema or growing a universal
visual runtime?

The accepted cases are:

1. union-find, showing components merge;
2. run-length encoding, showing visible input and encoded output;
3. A*, showing frontier and path evolution on a small editable map.

Only limitations that block or materially distort a demonstration are recorded.
Each case must use registry identity and claims, execute real algorithm state in
WASM, and remain clearly separate from implementation evidence.

## Union-find result

The existing `disjoint_set.rank_path_halving.union` registry entry now opens a
full-width interactive experience. A bounded WASM machine uses Petgraph's real
union-find implementation and exposes two semantic steps per operation:
representative inspection, then union. Users can choose or click elements,
advance manually or play, resize and reset the structure, and observe canonical
components and operation counts.

Native and Node tests cover exact component membership, redundant unions,
reset, bounds and step ordering. Desktop and mobile browser renders were checked
at 1440x1000 and 390x1000.

No public Atlas insufficiency blocked this case. Registry projection already
provided identity, provenance and complexity. The sequence-oriented visual
bytecode was intentionally not extended: a small domain machine and dedicated
renderer were clearer and exposed the actual operation. This is the
first measurement of experience-specific code, not yet evidence for a generic
experience format.

## Active experiment: run-length encoding

Test whether a codec with separate visible input and output can reuse the same
registry/evidence boundary while executing incrementally in WASM. The test is
falsified if the demonstration requires a public schema change, stores a trace
as presentation state, or must pretend the sequence visual bytecode represents
stream emission.
