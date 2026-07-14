const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const projection = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const dynamics = projection.dynamics.find(
  (item) => item.algorithm_id === "order.is_sorted.adjacent",
);
assert.ok(dynamics?.program);

for (const input of [[], [7], [1, 2, 2, 5], [5, 4, 3], [1, 2, 5, 4, 6]]) {
  const typed = Int32Array.from(input);
  const native = bindings.observe_is_sorted_i32(typed);
  const retained = new bindings.IsSortedStepper(typed);
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
  assert.equal(generated.comparisons, native.comparisons);
  assert.equal(generated.steps, retained.steps);
  assert.equal(generated.result_index, native.first_inversion);
  assert.equal(generated.has_result, !native.sorted);
  generated.free();
  retained.free();
  native.free();
}

console.log("Generated is_sorted matches native and retained stepper operation-for-operation.");
