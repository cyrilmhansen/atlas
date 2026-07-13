const assert = require("node:assert/strict");
const fs = require("node:fs");

const projection = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));
assert.equal(projection.format, "atlas-web-private-v0");
assert.match(projection.source_commit, /^[0-9a-f]{40}$/);
assert.match(projection.registry_digest, /^[0-9a-f]{64}$/);
assert.deepEqual(projection.counts, { problems: 10, algorithms: 15, implementations: 20 });

const algorithm = projection.algorithms.find((item) => item.id === "order.is_sorted.adjacent");
assert.ok(algorithm);
assert.equal(algorithm.solves, "sequence.is_sorted");
assert.equal(algorithm.time_worst.value, "O(n)");
assert.ok(algorithm.time_worst.source);
assert.equal(algorithm.auxiliary_memory.value, "O(1)");
assert.ok(algorithm.auxiliary_memory.source);

console.log("Derived Web projection identity, counts and sourced complexity passed.");
