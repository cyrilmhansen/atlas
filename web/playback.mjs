export const PLAYBACK_SPEEDS = Object.freeze([
  { label: "0.5x", delayMilliseconds: 1600 },
  { label: "1x", delayMilliseconds: 800, selected: true },
  { label: "2x", delayMilliseconds: 400 },
  { label: "4x", delayMilliseconds: 200 },
  { label: "8x", delayMilliseconds: 100 },
]);

export function playbackDelay(value) {
  const delay = Number(value);
  if (!PLAYBACK_SPEEDS.some((speed) => speed.delayMilliseconds === delay)) {
    throw new Error(`unknown playback delay ${value}`);
  }
  return delay;
}

export function isLoopContext(controlId, algorithm, done) {
  if (done) return false;
  if (algorithm === "insertion") {
    return controlId === "insertion.outer-loop" || controlId === "insertion.inner-loop";
  }
  return algorithm === "reverse" && controlId === "reverse.loop";
}
