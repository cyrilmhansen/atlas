const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const projection = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const dynamics = projection.dynamics.find(
  (item) => item.algorithm_id === "sort.insertion",
);
assert.ok(dynamics?.program);

for (const input of [[], [7], [1, 2, 2, 5], [5, 4, 3], [5, -1, 5, 3, 0, -8, 3]]) {
  const typed = Int32Array.from(input);
  const retained = new bindings.InsertionSortStepper(typed);
  const generated = new bindings.VisualMachine(JSON.stringify(dynamics.program), typed);
  const retainedOperations = [];
  const generatedOperations = [];

  while (retained.step()) {
    retainedOperations.push([
      retained.operation_node_id,
      retained.operation_kind,
      retained.operation_left_index,
      retained.operation_right_index,
      retained.operation_ordering,
    ]);
  }
  while (generated.step()) {
    generatedOperations.push([
      generated.operation_node_id,
      generated.operation_kind,
      generated.operation_left_index,
      generated.operation_right_index,
      generated.operation_ordering,
    ]);
  }

  assert.deepEqual(generatedOperations, retainedOperations);
  assert.deepEqual(Array.from(generated.values), Array.from(retained.values));
  assert.deepEqual(
    Array.from(generated.original_indices),
    Array.from(retained.original_indices),
  );
  assert.equal(generated.comparisons, retained.comparisons);
  assert.equal(generated.swaps, retained.swaps);
  assert.equal(generated.steps, retained.steps);
  generated.free();
  retained.free();
}

console.log("Generated insertion matches retained stepper operation-for-operation.");
