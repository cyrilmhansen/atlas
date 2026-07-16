const assert = require("node:assert/strict");
const fs = require("node:fs");

const projection = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
assert.equal(projection.format, "atlas-web-private-v0");
assert.match(projection.source_commit, /^[0-9a-f]{40}$/);
assert.match(projection.registry_digest, /^[0-9a-f]{64}$/);
assert.match(projection.build.rustc, /^rustc /);
assert.equal(projection.build.wasm_bindgen, "wasm-bindgen 0.2.100");
assert.equal(projection.build.target, "wasm32-unknown-unknown");
assert.equal(projection.build.profile, "release");
assert.deepEqual(projection.counts, { problems: 31, algorithms: 39, implementations: 43 });

assert.equal(projection.datasets.length, 10);
assert.deepEqual(
  projection.datasets.map((dataset) => [dataset.case_id, dataset.class]),
  [
    ["sort.typical.seeded_uniform", "typical"],
    ["sort.boundary.empty", "boundary"],
    ["sort.degenerate.equal", "degenerate"],
    ["sort.adversarial.descending", "adversarial"],
    ["sort.regression.duplicates", "regression"],
    ["partition.typical.mixed_sign", "typical"],
    ["partition.boundary.empty", "boundary"],
    ["partition.degenerate.all_matching", "degenerate"],
    ["partition.adversarial.alternating", "adversarial"],
    ["partition.regression.none_matching", "regression"],
  ],
);
for (const dataset of projection.datasets) {
  assert.match(dataset.content_digest_sha256, /^[0-9a-f]{64}$/);
  assert.ok(Array.isArray(dataset.values));
}
assert.equal(new Set(projection.datasets.map((dataset) => dataset.content_digest_sha256)).size, 10);
assert.deepEqual(projection.datasets[1].values, []);
assert.deepEqual(projection.datasets[4].values, [5, -1, 5, 3, 0, -8, 3]);

assert.equal(projection.dynamics.length, 5);
assert.equal(projection.dynamics[0].algorithm_id, "order.is_sorted.adjacent");
assert.equal(projection.dynamics[0].ast_id, "ast.order.is_sorted.adjacent.v0");
assert.equal(projection.dynamics[0].max_interactive_input_length, 64);
assert.equal(projection.dynamics[0].max_analytical_trace_input_length, 64);
assert.equal(projection.dynamics[0].program.instructions.length, 9);
assert.equal(projection.dynamics[0].presentation.key, "is_sorted");
assert.equal(projection.dynamics[0].presentation.result_view, "sortedness");
assert.match(
  projection.dynamics[0].pseudocode_source,
  /operation is-sorted\.adjacent\.compare \| Compare/,
);
assert.equal(projection.dynamics[1].algorithm_id, "sort.insertion");
assert.equal(projection.dynamics[1].ast_id, "ast.sort.insertion.v0");
assert.equal(projection.dynamics[1].max_interactive_input_length, 64);
assert.equal(projection.dynamics[1].max_analytical_trace_input_length, 32);
assert.equal(projection.dynamics[1].program.instructions.length, 13);
assert.equal(projection.dynamics[1].presentation.result_view, "stable_sorted");
assert.equal(projection.dynamics[1].presentation.tracks_origins, true);
assert.match(
  projection.dynamics[1].pseudocode_source,
  /operation insertion\.adjacent\.swap \| Swap/,
);
assert.equal(projection.dynamics[2].algorithm_id, "reverse.symmetric.in_place");
assert.equal(projection.dynamics[2].ast_id, "ast.reverse.symmetric.in_place.v0");
assert.equal(projection.dynamics[2].max_interactive_input_length, 64);
assert.equal(projection.dynamics[2].max_analytical_trace_input_length, 0);
assert.equal(projection.dynamics[2].program.instructions.length, 11);
assert.equal(projection.dynamics[2].presentation.result_view, "reversed");
assert.equal(projection.dynamics[2].presentation.tracks_origins, true);
assert.match(
  projection.dynamics[2].pseudocode_source,
  /operation reverse\.symmetric\.swap \| Swap/,
);
assert.equal(projection.dynamics[3].algorithm_id, "select.minimum.linear");
assert.equal(projection.dynamics[3].ast_id, "ast.select.minimum.linear.v0");
assert.equal(projection.dynamics[3].program.format, "atlas-visual-bytecode-private-v0");
assert.equal(projection.dynamics[3].program.instructions.length, 9);
assert.equal(projection.dynamics[3].presentation.key, "minimum");
assert.equal(projection.dynamics[3].presentation.primitive, "sequence");
assert.match(
  projection.dynamics[3].pseudocode_source,
  /operation minimum\.compare \| Compare/,
);
assert.equal(projection.dynamics[4].algorithm_id, "partition.two_pointer.in_place");
assert.equal(projection.dynamics[4].ast_id, "ast.partition.two_pointer.in_place.v0");
assert.equal(projection.dynamics[4].program.instructions.length, 19);
assert.equal(projection.dynamics[4].presentation.key, "partition");
assert.equal(projection.dynamics[4].presentation.dataset_predicate, "even");
assert.match(
  projection.dynamics[4].pseudocode_source,
  /operation partition\.swap \| Swap/,
);

const algorithm = projection.algorithms.find((item) => item.id === "order.is_sorted.adjacent");
assert.ok(algorithm);
assert.equal(algorithm.solves, "sequence.is_sorted");
assert.equal(algorithm.time_worst.value, "O(n)");
assert.ok(algorithm.time_worst.source);
assert.equal(algorithm.auxiliary_memory.value, "O(1)");
assert.ok(algorithm.auxiliary_memory.source);

const graphProblem = projection.problems.find((item) => item.id === "graph.nonnegative_shortest_distances");
assert.ok(graphProblem.requires);
assert.match(graphProblem.requires.value.join(" "), /nonnegative/);

const reservoir = projection.algorithms.find((item) => item.id === "stream.sample.reservoir_r");
assert.equal(reservoir.deterministic.value, false);
assert.equal(reservoir.requires, null);

const topK = projection.implementations.find((item) => item.id === "stream.top_k.rust.std_binary_heap.v1");
assert.equal(topK.version.value, "0.1.0");
assert.equal(topK.license.value, "MIT");
assert.match(topK.signature.value, /bounded_top_k/);
assert.match(topK.effects.value.allocation, /at most k/);
assert.equal(topK.effects.level, "tested");
assert.equal(topK.tests.value.length, 1);

console.log("Derived Web projection identity, datasets and sourced complexity passed.");
