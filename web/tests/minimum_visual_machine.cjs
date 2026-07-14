const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const projection = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const dynamics = projection.dynamics.find(
  (item) => item.algorithm_id === "select.minimum.linear",
);
assert.ok(dynamics?.program);

for (const fixture of [
  { input: [7, -2, 4, -1, 9], value: -2, index: 1, comparisons: 4 },
  { input: [2, 1, 1, 3], value: 1, index: 1, comparisons: 3 },
  { input: [42], value: 42, index: 0, comparisons: 0 },
  { input: [], value: undefined, index: undefined, comparisons: 0 },
]) {
  const machine = new bindings.VisualMachine(
    JSON.stringify(dynamics.program),
    Int32Array.from(fixture.input),
  );
  const nodes = [];
  while (machine.step()) nodes.push(machine.operation_node_id);

  assert.equal(machine.done, true);
  assert.equal(machine.result_value, fixture.value);
  assert.equal(machine.result_index, fixture.index);
  assert.equal(machine.comparisons, fixture.comparisons);
  assert.equal(machine.steps, fixture.comparisons * 3);
  assert.equal(nodes.filter((node) => node === "minimum.compare").length, fixture.comparisons);
  machine.free();
}

const invalid = structuredClone(dynamics.program);
invalid.instructions[1].when_true = 999;
assert.throws(
  () => new bindings.VisualMachine(JSON.stringify(invalid), Int32Array.from([1])),
  /unknown target/,
);

const scale = new bindings.VisualMachine(
  JSON.stringify(dynamics.program),
  new Int32Array(4096),
);
while (scale.step()) {}
assert.equal(scale.comparisons, 4095);
scale.free();
assert.throws(
  () => new bindings.VisualMachine(JSON.stringify(dynamics.program), new Int32Array(4097)),
  /browser limit of 4096/,
);

console.log("Generated minimum visual program matches native semantics and exact AST nodes.");
