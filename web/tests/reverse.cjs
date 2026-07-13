const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));

const fixtures = [
  [[], [], 0],
  [[7], [7], 0],
  [[1, 2], [2, 1], 1],
  [[1, 2, 3, 4, 5], [5, 4, 3, 2, 1], 2],
  [[1, 2, 3, 4, 5, 6], [6, 5, 4, 3, 2, 1], 3],
];

for (const [input, values, swaps] of fixtures) {
  const observation = bindings.observe_reverse_i32(Int32Array.from(input));
  assert.deepEqual(Array.from(observation.values), values);
  assert.equal(observation.reads, swaps * 2);
  assert.equal(observation.writes, swaps * 2);
  assert.equal(observation.swaps, swaps);

  const restored = bindings.observe_reverse_i32(observation.values);
  assert.deepEqual(Array.from(restored.values), input);
  restored.free();
  observation.free();
}

assert.throws(
  () => bindings.observe_reverse_i32(new Int32Array(4097)),
  /exceeds the Atlas browser limit of 4096/,
);

console.log("WASM/native reverse fixtures, counters, involution and input limit passed.");
