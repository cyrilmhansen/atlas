import assert from "node:assert/strict";
import { createRequire } from "node:module";
import path from "node:path";

import { generateSequence } from "../generator.mjs";

const require = createRequire(import.meta.url);
const bindings = require(path.resolve(process.argv[2]));

for (const length of [8, 32, 128]) {
  const ascending = generateSequence("ascending", length, 0);
  const sorted = bindings.observe_is_sorted_i32(Int32Array.from(ascending));
  assert.equal(sorted.comparisons, length - 1);
  sorted.free();

  const descending = generateSequence("descending", length, 0);
  const insertion = bindings.observe_insertion_sort_i32(Int32Array.from(descending));
  assert.equal(insertion.comparisons, (length * (length - 1)) / 2);
  assert.equal(insertion.swaps, (length * (length - 1)) / 2);
  insertion.free();

  const reverse = bindings.observe_reverse_i32(Int32Array.from(ascending));
  assert.equal(reverse.swaps, Math.floor(length / 2));
  reverse.free();
}

console.log("Scale profiles expose exact linear and quadratic operation-growth fixtures.");
