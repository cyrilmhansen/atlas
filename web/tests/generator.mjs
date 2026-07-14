import assert from "node:assert/strict";

import {
  EXPLORE_MAX_LENGTH,
  MAX_GENERATED_LENGTH,
  generateSequence,
  randomSeed,
} from "../generator.mjs";

assert.deepEqual(generateSequence("uniform", 8, 17), generateSequence("uniform", 8, 17));
assert.notDeepEqual(generateSequence("uniform", 8, 17), generateSequence("uniform", 8, 18));
assert.deepEqual(generateSequence("ascending", 5, 0), [-2, -1, 0, 1, 2]);
assert.deepEqual(generateSequence("descending", 5, 0), [2, 1, 0, -1, -2]);
assert.ok(generateSequence("few_unique", 64, 1).every((value) => value >= 0 && value <= 7));
assert.equal(EXPLORE_MAX_LENGTH, 64);
assert.equal(generateSequence("uniform", MAX_GENERATED_LENGTH, 0).length, 4096);
assert.throws(() => generateSequence("uniform", 4097, 0), /between 0 and 4096/);
assert.throws(() => generateSequence("unknown", 8, 0), /unknown generation profile/);
assert.throws(() => generateSequence("uniform", 8, -1), /unsigned 32-bit integer/);
assert.equal(randomSeed({ getRandomValues: (values) => { values[0] = 0xfedcba98; } }), 0xfedcba98);
assert.throws(() => randomSeed({}), /random generation is unavailable/);

console.log("Deterministic Web data profiles, sizes, seeds and limits passed.");
