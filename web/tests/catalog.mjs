import assert from "node:assert/strict";
import fs from "node:fs";

import {
  catalogRecords,
  claimEntries,
  comparableRows,
  executablePresentation,
  filterCatalog,
  findRecord,
  relatedRecords,
} from "../catalog.mjs";

const projection = JSON.parse(fs.readFileSync(process.argv[2], "utf8"));

assert.equal(catalogRecords(projection).length, 109);
assert.equal(filterCatalog(projection, "", "problem").length, 31);
assert.equal(filterCatalog(projection, "graph", "problem").length, 4);

const problem = findRecord(projection, "graph.reachable_traversal");
assert.equal(problem.kind, "problem");
assert.deepEqual(
  relatedRecords(projection, problem).map((record) => record.entity.id),
  ["graph.bfs.traversal", "graph.dfs.traversal"],
);

const algorithm = findRecord(projection, "graph.bfs.traversal");
assert.deepEqual(
  relatedRecords(projection, algorithm).map((record) => record.entity.id),
  ["graph.reachable_traversal", "graph.bfs.petgraph.0_8_3"],
);

const implementation = findRecord(projection, "stream.top_k.rust.std_binary_heap.v1");
const implementationClaims = new Map(claimEntries(implementation).map((entry) => [entry.key, entry.claim]));
assert.equal(implementationClaims.get("version").value, "0.1.0");
assert.match(implementationClaims.get("effects").value.allocation, /at most k/);
assert.equal(implementationClaims.get("effects").level, "tested");

const hashDeduplicate = findRecord(projection, "deduplicate.hash.stable");
const quadraticDeduplicate = findRecord(projection, "deduplicate.quadratic.stable");
const comparison = new Map(comparableRows(hashDeduplicate, quadraticDeduplicate).map((row) => [row.key, row]));
assert.equal(comparison.get("time_expected").left.value, "O(n)");
assert.equal(comparison.get("time_expected").right, null);
assert.equal(comparison.get("stable").left.value, true);
assert.equal(comparison.get("stable").right.value, true);

const insertion = findRecord(projection, "sort.insertion");
assert.equal(executablePresentation(projection, insertion).key, "insertion");
assert.equal(executablePresentation(projection, algorithm), undefined);

console.log("Relational catalog, factual comparison and execution availability passed.");
