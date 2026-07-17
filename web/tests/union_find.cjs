const assert = require("node:assert/strict");
const path = require("node:path");

const wasm = require(path.resolve(process.argv[2]));

const machine = new wasm.UnionFindMachine(6);
assert.equal(machine.done, true);
assert.deepEqual(JSON.parse(machine.snapshot_json()).components, [0, 1, 2, 3, 4, 5]);

machine.begin_union(0, 1);
assert.equal(machine.done, false);
assert.equal(machine.step(), true);
let snapshot = JSON.parse(machine.snapshot_json());
assert.equal(snapshot.phase, "merge");
assert.equal(snapshot.left_representative, 0);
assert.equal(snapshot.right_representative, 1);
assert.deepEqual(snapshot.components, [0, 1, 2, 3, 4, 5]);

assert.equal(machine.step(), true);
snapshot = JSON.parse(machine.snapshot_json());
assert.equal(snapshot.phase, "complete");
assert.equal(snapshot.merged, true);
assert.deepEqual(snapshot.components, [0, 0, 2, 3, 4, 5]);

machine.begin_union(1, 2);
machine.step();
machine.step();
machine.begin_union(0, 2);
machine.step();
machine.step();
snapshot = JSON.parse(machine.snapshot_json());
assert.equal(snapshot.merged, false);
assert.deepEqual(snapshot.components, [0, 0, 0, 3, 4, 5]);
assert.equal(snapshot.union_attempts, 3);
assert.equal(snapshot.successful_unions, 2);

assert.throws(() => machine.begin_union(6, 0), /outside union-find size 6/);
assert.throws(() => new wasm.UnionFindMachine(33), /browser limit of 32/);
machine.free();

console.log("Incremental union-find WASM components and redundant unions passed.");
