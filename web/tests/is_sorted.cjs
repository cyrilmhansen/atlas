const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));

const fixtures = [
  [[], true, 0, undefined],
  [[7], true, 0, undefined],
  [[-2, 0, 0, 4], true, 3, undefined],
  [[1, 2, 5, 4, 6], false, 3, 3],
  [[3, 2, 1], false, 1, 1],
];

for (const [values, sorted, comparisons, inversion] of fixtures) {
  const observation = bindings.observe_is_sorted_i32(Int32Array.from(values));
  assert.equal(observation.sorted, sorted);
  assert.equal(observation.comparisons, comparisons);
  assert.equal(observation.first_inversion, inversion);
  observation.free();
}

assert.throws(
  () => bindings.observe_is_sorted_i32(new Int32Array(4097)),
  /exceeds the Atlas browser limit of 4096/,
);

console.log("WASM/native is_sorted fixtures and input limit passed.");
