const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const projection = JSON.parse(fs.readFileSync(process.argv[3], "utf8"));
const dynamics = projection.dynamics.find(
  (item) => item.algorithm_id === "partition.two_pointer.in_place",
);
assert.ok(dynamics?.program);

for (const fixture of [
  { input: [], output: [], boundary: 0, predicates: 0, swaps: 0 },
  { input: [1, 3, 5], output: [1, 3, 5], boundary: 0, predicates: 4, swaps: 0 },
  { input: [1, 2, 3, 4, 5, 6], output: [6, 2, 4, 3, 5, 1], boundary: 3, predicates: 6, swaps: 2 },
  { input: [0, 1, 0, 1], output: [0, 0, 1, 1], boundary: 2, predicates: 4, swaps: 1 },
]) {
  const machine = new bindings.VisualMachine(
    JSON.stringify(dynamics.program),
    Int32Array.from(fixture.input),
  );
  const nodes = [];
  while (machine.step()) nodes.push(machine.operation_node_id);

  assert.deepEqual(Array.from(machine.values), fixture.output);
  assert.equal(machine.result_index, fixture.boundary);
  assert.equal(machine.predicate_evaluations, fixture.predicates);
  assert.equal(machine.swaps, fixture.swaps);
  assert.equal(nodes.at(-1), "partition.boundary");
  assert.equal(nodes.filter((node) => node === "partition.swap").length, fixture.swaps);
  assert.deepEqual(
    Array.from(machine.original_indices).sort((left, right) => left - right),
    fixture.input.map((_, index) => index),
  );
  machine.free();
}

console.log("Generated even partition matches native mutation, counters and AST nodes.");
