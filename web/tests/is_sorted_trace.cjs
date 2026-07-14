const assert = require("node:assert/strict");
const path = require("node:path");

const bindings = require(path.resolve(process.argv[2]));
const trace = bindings.trace_is_sorted_i32(Int32Array.from([1, 2, 5, 4, 6]));

assert.equal(trace.sorted, false);
assert.equal(trace.first_inversion, 3);
assert.equal(trace.event_count, 9);
assert.deepEqual(
  Array.from({ length: trace.event_count }, (_, index) => [
    trace.event_node_id(index),
    trace.event_operation(index),
  ]),
  [
    ["is-sorted.left.read", "Read"],
    ["is-sorted.right.read", "Read"],
    ["is-sorted.adjacent.compare", "Compare"],
    ["is-sorted.left.read", "Read"],
    ["is-sorted.right.read", "Read"],
    ["is-sorted.adjacent.compare", "Compare"],
    ["is-sorted.left.read", "Read"],
    ["is-sorted.right.read", "Read"],
    ["is-sorted.adjacent.compare", "Compare"],
  ],
);
assert.equal(trace.event_left_index(8), 2);
assert.equal(trace.event_right_index(8), 3);
assert.equal(trace.event_ordering(8), 1);
assert.equal(trace.event_node_id(9), undefined);
trace.free();

assert.throws(
  () => bindings.trace_is_sorted_i32(new Int32Array(65)),
  /exceeds the Atlas Explore limit of 64/,
);

console.log("Bounded is_sorted trace events and exact AST node links passed.");
