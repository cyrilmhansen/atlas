const assert = require("node:assert/strict");
const path = require("node:path");

const { AstarMachine } = require(path.resolve(process.argv[2]));

function snapshot(machine) { return JSON.parse(machine.snapshot_json()); }

const blocked = Uint8Array.from([0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0]);
const machine = new AstarMachine(4, 3, blocked, 0, 11);
let previousExpansions = 0;
while (machine.step()) {
  const current = snapshot(machine);
  assert.ok(current.expansions >= previousExpansions);
  previousExpansions = current.expansions;
}
const result = snapshot(machine);
assert.equal(result.found, true);
assert.equal(result.path[0], 0);
assert.equal(result.path.at(-1), 11);
assert.equal(result.path.length - 1, 5);
assert.ok(result.closed.some(Boolean));
machine.free();

const sealed = new AstarMachine(3, 3, Uint8Array.from([0, 1, 0, 1, 0, 1, 0, 1, 0]), 0, 2);
while (sealed.step()) {}
assert.equal(snapshot(sealed).found, false);
assert.deepEqual(snapshot(sealed).path, []);
sealed.free();

console.log("Incremental A* WASM frontier and shortest path passed.");
