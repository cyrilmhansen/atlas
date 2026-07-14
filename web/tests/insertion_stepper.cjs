const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const input = Int32Array.from([2, 1, 2, 1]);
const stepper = new bindings.InsertionSortStepper(input);
const trace = bindings.trace_insertion_sort_i32(input);

assert.deepEqual(Array.from(stepper.values), [2, 1, 2, 1]);
assert.equal(stepper.steps, 0);
assert.equal(stepper.done, false);

for (let index = 0; index < trace.event_count; index += 1) {
  assert.equal(stepper.step(), true);
  assert.equal(stepper.operation_node_id, trace.event_node_id(index));
  assert.equal(stepper.operation_kind, trace.event_operation(index));
  assert.equal(stepper.operation_left_index, trace.event_left_index(index));
  assert.equal(stepper.operation_right_index, trace.event_right_index(index));
  assert.equal(stepper.operation_ordering, trace.event_ordering(index));
}

assert.equal(stepper.done, true);
assert.equal(stepper.step(), false);
assert.deepEqual(Array.from(stepper.values), [1, 1, 2, 2]);
assert.deepEqual(Array.from(stepper.original_indices), [1, 3, 0, 2]);
assert.equal(stepper.comparisons, 5);
assert.equal(stepper.swaps, 3);

stepper.reset(Int32Array.from([3, 2, 1]));
assert.equal(stepper.steps, 0);
while (stepper.step()) {}
assert.deepEqual(Array.from(stepper.values), [1, 2, 3]);

assert.throws(
  () => new bindings.InsertionSortStepper(new Int32Array(33)),
  /exceeds the Atlas insertion Explore limit of 32/,
);

trace.free();
stepper.free();
console.log("Incremental WASM insertion execution matches the analytical trace and stable result.");
