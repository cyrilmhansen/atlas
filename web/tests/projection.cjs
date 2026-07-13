const assert = require("node:assert/strict");
const fs = require("node:fs");

const projection = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
assert.equal(projection.format, "atlas-web-private-v0");
assert.match(projection.source_commit, /^[0-9a-f]{40}$/);
assert.match(projection.registry_digest, /^[0-9a-f]{64}$/);
assert.deepEqual(projection.counts, { problems: 10, algorithms: 15, implementations: 20 });

assert.equal(projection.datasets.length, 5);
assert.deepEqual(
  projection.datasets.map((dataset) => [dataset.case_id, dataset.class]),
  [
    ["sort.typical.seeded_uniform", "typical"],
    ["sort.boundary.empty", "boundary"],
    ["sort.degenerate.equal", "degenerate"],
    ["sort.adversarial.descending", "adversarial"],
    ["sort.regression.duplicates", "regression"],
  ],
);
for (const dataset of projection.datasets) {
  assert.equal(dataset.spec_id, "dataset.sequence.sort.m2.v0");
  assert.equal(dataset.problem_id, "sequence.sort");
  assert.match(dataset.content_digest_sha256, /^[0-9a-f]{64}$/);
  assert.ok(Array.isArray(dataset.values));
}
assert.equal(new Set(projection.datasets.map((dataset) => dataset.content_digest_sha256)).size, 5);
assert.deepEqual(projection.datasets[1].values, []);
assert.deepEqual(projection.datasets[4].values, [5, -1, 5, 3, 0, -8, 3]);

const algorithm = projection.algorithms.find((item) => item.id === "order.is_sorted.adjacent");
assert.ok(algorithm);
assert.equal(algorithm.solves, "sequence.is_sorted");
assert.equal(algorithm.time_worst.value, "O(n)");
assert.ok(algorithm.time_worst.source);
assert.equal(algorithm.auxiliary_memory.value, "O(1)");
assert.ok(algorithm.auxiliary_memory.source);

console.log("Derived Web projection identity, datasets and sourced complexity passed.");
