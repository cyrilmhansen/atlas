import assert from "node:assert/strict";

import { PLAYBACK_SPEEDS, playbackDelay } from "../playback.mjs";

assert.deepEqual(
  PLAYBACK_SPEEDS.map((speed) => speed.delayMilliseconds),
  [1600, 800, 400, 200, 100],
);
assert.equal(PLAYBACK_SPEEDS.filter((speed) => speed.selected).length, 1);
assert.equal(playbackDelay("200"), 200);
assert.throws(() => playbackDelay("250"), /unknown playback delay/);

console.log("Playback speeds follow a factor-two scale with one default.");
