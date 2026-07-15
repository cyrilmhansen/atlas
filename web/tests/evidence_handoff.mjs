import assert from "node:assert/strict";
import { readFile } from "node:fs/promises";
import { executablePresentation, findRecord } from "../catalog.mjs";

const projection = JSON.parse(await readFile(process.argv[2], "utf8"));
const html = await readFile("web/index.html", "utf8");
const app = await readFile("web/app.js", "utf8");

const executable = findRecord(projection, "order.is_sorted.adjacent");
const unavailable = findRecord(projection, "graph.bfs.traversal");
assert.ok(executablePresentation(projection, executable));
assert.equal(executablePresentation(projection, unavailable), undefined);
assert.equal(unavailable.entity.time_worst.value, "O(V + E)");
assert.equal(unavailable.entity.time_worst.level, "inferred");
assert.equal(unavailable.entity.time_worst.source, "analysis:phase2/k-m1-graph-corpus");

const executableAlgorithms = projection.algorithms.filter((algorithm) =>
  projection.dynamics.some((dynamics) => dynamics.algorithm_id === algorithm.id && dynamics.presentation));
assert.equal(executableAlgorithms.length, 5);

assert.match(html, /id="entity-execution-status"/);
assert.match(html, /id="execution-evidence-link"/);
assert.match(html, />Local observation</);
assert.match(app, /Interactive WASM model available; implementation evidence is separate/);
assert.match(app, /No interactive WASM model; implementation evidence remains available/);
assert.match(app, /renderClaimProvenance\(elements\["time-provenance"\], algorithm\.time_worst\)/);
assert.match(app, /selectedEntityId = algorithmUi\[activeAlgorithm\]\.id/);

console.log("Evidence availability and exact registry handoff passed.");
