export const EXPLORE_MAX_LENGTH = 64;
export const MAX_GENERATED_LENGTH = 4096;

const MASK_64 = (1n << 64n) - 1n;
const PROFILES = new Set(["uniform", "ascending", "descending", "few_unique"]);

export function randomSeed(randomSource = globalThis.crypto) {
  if (!randomSource?.getRandomValues) throw new Error("secure local random generation is unavailable");
  const seed = new Uint32Array(1);
  randomSource.getRandomValues(seed);
  return seed[0];
}

function nextSplitMix64(state) {
  const nextState = (state + 0x9e3779b97f4a7c15n) & MASK_64;
  let value = nextState;
  value = ((value ^ (value >> 30n)) * 0xbf58476d1ce4e5b9n) & MASK_64;
  value = ((value ^ (value >> 27n)) * 0x94d049bb133111ebn) & MASK_64;
  return [nextState, value ^ (value >> 31n)];
}

export function generateSequence(profile, length, seed) {
  if (!PROFILES.has(profile)) throw new Error(`unknown generation profile ${profile}`);
  if (!Number.isInteger(length) || length < 0 || length > MAX_GENERATED_LENGTH) {
    throw new Error(`generated length must be between 0 and ${MAX_GENERATED_LENGTH}`);
  }
  if (!Number.isSafeInteger(seed) || seed < 0 || seed > 0xffffffff) {
    throw new Error("seed must be an unsigned 32-bit integer");
  }

  const midpoint = Math.floor(length / 2);
  if (profile === "ascending") return Array.from({ length }, (_, index) => index - midpoint);
  if (profile === "descending") return Array.from({ length }, (_, index) => midpoint - index);

  let state = BigInt(seed);
  return Array.from({ length }, () => {
    let random;
    [state, random] = nextSplitMix64(state);
    return profile === "few_unique" ? Number(random % 8n) : Number(random % 201n) - 100;
  });
}
