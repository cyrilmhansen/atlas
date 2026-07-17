const assert = require("node:assert/strict");
const path = require("node:path");

const { RleMachine } = require(path.resolve(process.argv[2]));

function snapshot(machine) {
  return JSON.parse(machine.snapshot_json());
}

const machine = new RleMachine("AAABCCDDDD");
let previousCursor = 0;
while (machine.step()) {
  const current = snapshot(machine);
  assert.ok(current.cursor >= previousCursor);
  assert.ok(current.output.every((run) => run.count > 0));
  previousCursor = current.cursor;
}
assert.deepEqual(snapshot(machine).output, [
  { symbol: 65, count: 3 },
  { symbol: 66, count: 1 },
  { symbol: 67, count: 2 },
  { symbol: 68, count: 4 },
]);
assert.equal(snapshot(machine).reads, 10);
assert.equal(snapshot(machine).comparisons, 9);

machine.reset("");
assert.equal(machine.done, true);
assert.deepEqual(snapshot(machine).output, []);
assert.throws(() => machine.reset("é"), /ASCII/);
assert.throws(() => machine.reset("A".repeat(65)), /limit/);
machine.free();

console.log("Incremental RLE WASM input consumption and output emission passed.");
