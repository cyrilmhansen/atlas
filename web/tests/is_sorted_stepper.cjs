const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));

for (const input of [[1, 1, 3, 5], [5, -1, 7], [], [7]]) {
  const typed = Int32Array.from(input);
  const stepper = new bindings.IsSortedStepper(typed);
  const trace = bindings.trace_is_sorted_i32(typed);

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
  assert.equal(stepper.sorted, trace.sorted);
  assert.equal(stepper.first_inversion, trace.first_inversion);
  stepper.free();
  trace.free();
}

assert.throws(
  () => new bindings.IsSortedStepper(new Int32Array(65)),
  /exceeds the Atlas Explore limit of 64/,
);

console.log("Incremental WASM is_sorted execution matches its analytical trace.");
