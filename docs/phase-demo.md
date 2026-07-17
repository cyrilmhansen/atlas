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

## Run-length encoding result

`codec.rle.adjacent_runs` adds one Problem, Algorithm and Rust Implementation.
Its direct encoder is the oracle for a bounded WASM machine that consumes ASCII
input incrementally. The browser separates consumed input, the pending run and
emitted runs; edits require an explicit apply action and no presentation trace
is stored. Empty, singleton, alternating, invalid and oversized cases are tested.

This second case also required no public schema change. The experience shares a
small control cycle with union-find, but its state and rendering are different.
Two cases do not yet justify a generated experience format; the duplication is
retained until A* supplies a third independent case.

## A* result

`graph.astar.binary_heap` and its bounded grid implementation are registered
separately from the existing all-destinations Dijkstra contracts. The interactive
grid lets users paint walls, move start and goal, and inspect frontier, closed
cells and the final path one expansion at a time. The incremental implementation
is checked against `petgraph::algo::astar`, including an unreachable goal.

The map is explicitly a local unit-cost specialization. Generic A* requirements
and inferred costs remain registry claims; frontier size and expansions are local
observations. Desktop and mobile renders were checked at 1440x1000 and 390x1000.

## Verdict

The phase is conclusive. Three structurally different demonstrations execute real
WASM state and reuse registry identity, claims, provenance and evidence handoff
without a public schema change, stored presentation traces or a universal visual
runtime.

One scalability insufficiency is now demonstrated in the private Explorer: each
domain experience repeats registration, playback lifecycle and catalog routing,
while its state model and renderer remain legitimately specific. The next work
should factor only the common private experience shell and keep union-find, RLE
and A* steppers/renderers explicit. This does not yet justify a stable experience
format or an extension of the sequence visual bytecode.
