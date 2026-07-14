const assert = require("node:assert/strict");
const fs = require("node:fs");

const source = fs.readFileSync(process.argv[2], "utf8");
const projection = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));

assert.doesNotMatch(source, /\b(?:InsertionSortStepper|IsSortedStepper|ReverseStepper)\b/);
assert.doesNotMatch(source, /\bobserve_(?:insertion_sort|is_sorted|reverse)_i32\b/);
assert.doesNotMatch(source, /activeAlgorithm\s*===/);
assert.match(source, /import init, \{ VisualMachine \}/);
assert.match(source, /function runGeneratedAlgorithm\(/);

assert.equal(projection.dynamics.length, 5);
for (const dynamics of projection.dynamics) {
  assert.ok(dynamics.program, `${dynamics.algorithm_id} has no generated program`);
  assert.ok(dynamics.presentation, `${dynamics.algorithm_id} has no presentation`);
}

console.log("Browser execution depends only on generated programs and VisualMachine.");
