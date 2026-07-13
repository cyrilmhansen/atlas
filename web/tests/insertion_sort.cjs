const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));

const fixtures = [
  [[], [], [], 0, 0],
  [[7], [7], [0], 0, 0],
  [[1, 2, 3], [1, 2, 3], [0, 1, 2], 2, 0],
  [[3, 2, 1], [1, 2, 3], [2, 1, 0], 3, 3],
  [[2, 1, 2, 1], [1, 1, 2, 2], [1, 3, 0, 2], 5, 3],
];

for (const [input, values, indices, comparisons, swaps] of fixtures) {
  const observation = bindings.observe_insertion_sort_i32(Int32Array.from(input));
  assert.deepEqual(Array.from(observation.values), values);
  assert.deepEqual(Array.from(observation.original_indices), indices);
  assert.equal(observation.comparisons, comparisons);
  assert.equal(observation.swaps, swaps);
  observation.free();
}

assert.throws(
  () => bindings.observe_insertion_sort_i32(new Int32Array(4097)),
  /exceeds the Atlas browser limit of 4096/,
);

console.log("WASM/native stable insertion fixtures and input limit passed.");
