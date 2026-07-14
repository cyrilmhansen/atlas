const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));

for (const input of [[], [7], [1, 2, 3, 4, 5], [1, 2, 3, 4, 5, 6]]) {
  const typed = Int32Array.from(input);
  const stepper = new bindings.ReverseStepper(typed);
  const observation = bindings.observe_reverse_i32(typed);
  const nodeIds = [];

  while (stepper.step()) nodeIds.push(stepper.operation_node_id);

  assert.deepEqual(Array.from(stepper.values), Array.from(observation.values));
  assert.equal(stepper.reads, observation.reads);
  assert.equal(stepper.writes, observation.writes);
  assert.equal(stepper.swaps, observation.swaps);
  assert.equal(stepper.steps, observation.swaps * 3);
  assert.deepEqual(
    nodeIds,
    Array.from({ length: observation.swaps }, () => [
      "reverse.left.read",
      "reverse.right.read",
      "reverse.symmetric.swap",
    ]).flat(),
  );
  assert.deepEqual(
    Array.from(stepper.original_indices),
    Array.from({ length: input.length }, (_, index) => input.length - 1 - index),
  );
  stepper.free();
  observation.free();
}

assert.throws(
  () => new bindings.ReverseStepper(new Int32Array(65)),
  /exceeds the Atlas Explore limit of 64/,
);

console.log("Incremental WASM reverse execution matches native counts and mutation.");
