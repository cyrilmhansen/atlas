import assert from "node:assert/strict";

import {
  PLAYBACK_SPEEDS,
  isInsertionLoopContext,
  playbackDelay,
} from "../playback.mjs";

assert.deepEqual(
  PLAYBACK_SPEEDS.map((speed) => speed.delayMilliseconds),
  [1600, 800, 400, 200, 100],
);
assert.equal(PLAYBACK_SPEEDS.filter((speed) => speed.selected).length, 1);
assert.equal(playbackDelay("200"), 200);
assert.throws(() => playbackDelay("250"), /unknown playback delay/);
assert.equal(isInsertionLoopContext("insertion.outer-loop", "stepper", false), true);
assert.equal(isInsertionLoopContext("insertion.inner-loop", "stepper", false), true);
assert.equal(isInsertionLoopContext(undefined, "stepper", false), false);
assert.equal(isInsertionLoopContext("insertion.outer-loop", "stepper", true), false);

console.log("Playback speeds follow a factor-two scale with one default.");
