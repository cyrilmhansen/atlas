import init, {
  InsertionSortStepper,
  IsSortedStepper,
  ReverseStepper,
  observe_insertion_sort_i32,
  observe_is_sorted_i32,
  observe_reverse_i32,
} from "./pkg/atlas_web.js";
import { EXPLORE_MAX_LENGTH, generateSequence, randomSeed } from "./generator.mjs";
import { PLAYBACK_SPEEDS, isLoopContext, playbackDelay } from "./playback.mjs";

const algorithmUi = {
  is_sorted: {
    id: "order.is_sorted.adjacent",
    dataset: "sort.degenerate.equal",
    boundary: "The read-only algorithm state stays in WASM; each displayed state copies current values only.",
    resultLabel: "Result",
    comparisonLabel: "Comparisons",
    secondaryLabel: "First inversion",
    sequenceHeading: "Sequence state",
    legend: "first decreasing pair",
    moved: false,
  },
  insertion: {
    id: "sort.insertion",
    dataset: "sort.regression.duplicates",
    boundary: "The incremental algorithm state stays in WASM; each displayed state copies current tagged values only.",
    resultLabel: "Correction + stability",
    comparisonLabel: "Comparisons",
    secondaryLabel: "Adjacent swaps",
    sequenceHeading: "Stable sorted output",
    legend: "moved from original index",
    moved: true,
  },
  reverse: {
    id: "reverse.symmetric.in_place",
    dataset: "sort.regression.duplicates",
    boundary: "The incremental algorithm state stays in WASM; each displayed state copies current values and origins only.",
    resultLabel: "Correction + involution",
    comparisonLabel: "Semantic reads / writes",
    secondaryLabel: "Symmetric swaps",
    sequenceHeading: "Reversed output",
    legend: "moved from original index",
    moved: true,
  },
};

const elements = Object.fromEntries(
  [
    "entity-count", "registry-digest", "source-commit", "build-environment", "algorithm-id", "algorithm-name",
    "execution-boundary", "result-label", "comparison-label", "secondary-label",
    "time-complexity", "time-provenance", "space-complexity", "space-provenance",
    "dataset-select", "dataset-context", "sequence-input", "input-count", "run-button", "runtime-status",
    "sorted-result", "comparison-count", "inversion-index", "local-time",
    "runtime-context", "sequence-heading", "sequence-visual", "legend-text",
    "sequence-note", "dynamics-panel", "trace-ast-id", "pseudocode-code",
    "trace-progress", "execution-context", "trace-sequence", "trace-event", "trace-slider",
    "trace-reset", "trace-previous", "trace-play", "trace-next", "trace-speed",
    "generator-profile", "generator-size", "generator-seed", "generator-random-seed", "generate-button",
    "scale-panel", "scale-operation", "scale-chart", "scale-note",
    "catalog-search", "catalog-body",
  ].map((id) => [id, document.getElementById(id)]),
);

let projection;
let wasmReady = false;
let activeAlgorithm = "is_sorted";
let generatedInput = null;
const tracePlayback = {
  algorithm: null,
  values: [],
  originalIndices: [],
  index: -1,
  input: [],
  stepper: null,
  operation: null,
  timer: null,
};

function stopTracePlayback() {
  if (tracePlayback.timer !== null) window.clearTimeout(tracePlayback.timer);
  tracePlayback.timer = null;
  elements["trace-play"].textContent = "Play";
}

function clearTrace(message) {
  stopTracePlayback();
  tracePlayback.stepper?.free();
  tracePlayback.algorithm = null;
  tracePlayback.values = [];
  tracePlayback.originalIndices = [];
  tracePlayback.index = -1;
  tracePlayback.input = [];
  tracePlayback.stepper = null;
  tracePlayback.operation = null;
  elements["trace-progress"].textContent = "No execution";
  elements["trace-event"].textContent = message;
  elements["trace-slider"].max = "0";
  elements["trace-slider"].value = "0";
  elements["trace-sequence"].replaceChildren();
  elements["execution-context"].hidden = true;
  document.querySelectorAll(".pseudo-line").forEach((line) => {
    line.classList.remove("is-active", "is-context");
  });
  updateTraceControls();
}

function pseudocodeLine(sourceLine) {
  const text = sourceLine.trim();
  const indent = Math.floor((sourceLine.length - sourceLine.trimStart().length) / 2);
  if (text.startsWith("operation ")) {
    const [nodeId, operation, description] = text.slice("operation ".length).split("|").map((field) => field.trim());
    return { text: `${operation.toLowerCase()} ${description}`, nodeId, indent, kind: "operation" };
  }
  if (text.startsWith("let ")) {
    const [name, expression] = text.slice(4).split("|").map((field) => field.trim());
    return { text: `${name} <- ${expression}`, indent, kind: "control" };
  }
  if (text.startsWith("while ") || text.startsWith("if ") || text === "end") {
    const controlId = text === "while index < length(values)"
      ? "insertion.outer-loop"
      : text === "while current > 0"
        ? "insertion.inner-loop"
        : text === "while left < floor(length(values) / 2)" ? "reverse.loop" : undefined;
    return { text, indent, kind: "control", controlId };
  }
  if (text.startsWith("return ")) return { text, indent, kind: "return" };
  return { text, indent, kind: "plain" };
}

function renderPseudocode(dynamics) {
  let inBody = false;
  const lines = [];
  for (const sourceLine of dynamics.pseudocode_source.split("\n")) {
    const text = sourceLine.trim();
    if (text === "begin") {
      inBody = true;
      continue;
    }
    if (!inBody || text === "" || text.startsWith("#")) continue;
    lines.push(pseudocodeLine(sourceLine));
  }
  elements["pseudocode-code"].replaceChildren(...lines.map((line) => {
    const row = document.createElement("div");
    row.className = `pseudo-line is-${line.kind}`;
    row.style.setProperty("--indent", String(line.indent));
    row.textContent = line.text;
    if (line.nodeId) row.dataset.nodeId = line.nodeId;
    if (line.controlId) row.dataset.controlId = line.controlId;
    return row;
  }));
  elements["trace-ast-id"].textContent = dynamics.ast_id;
}

function traceEventLabel(event) {
  if (event.operation === "Read") {
    return `${event.nodeId}: read values[${event.leftIndex}] = ${tracePlayback.values[event.leftIndex]}`;
  }
  if (event.operation === "Swap") {
    const relation = tracePlayback.algorithm === "reverse" ? "symmetric" : "adjacent";
    return `${event.nodeId}: swap ${relation} positions #${event.leftIndex} and #${event.rightIndex}; the WASM state is now updated.`;
  }
  const symbols = ["<", "=", ">"];
  const comparison = `${event.nodeId}: compare values[${event.leftIndex}] ${symbols[event.ordering + 1]} values[${event.rightIndex}]`;
  if (tracePlayback.algorithm === "insertion") {
    return event.ordering < 0
      ? `${comparison}; the current element must move left.`
      : `${comparison}; this insertion position is stable.`;
  }
  if (event.ordering > 0) {
    return `${comparison}; inversion at #${tracePlayback.stepper.first_inversion}, return false and stop early.`;
  }
  return tracePlayback.stepper.done
    ? `${comparison}; scan complete, return true.`
    : `${comparison}; continue with the next adjacent pair.`;
}

function renderTraceState() {
  const event = tracePlayback.operation;
  elements["trace-sequence"].replaceChildren(...tracePlayback.values.map((value, index) => {
    const cell = document.createElement("div");
    cell.className = "trace-cell";
    if (event?.operation === "Read" && index === event.leftIndex) cell.classList.add("is-read");
    if (event?.operation === "Compare"
      && (index === event.leftIndex || index === event.rightIndex)) {
      const inversion = tracePlayback.algorithm === "insertion"
        ? event.ordering < 0
        : event.ordering > 0;
      cell.classList.add(inversion
        ? "is-inversion"
        : "is-compare");
    }
    if (event?.operation === "Swap"
      && (index === event.leftIndex || index === event.rightIndex)) cell.classList.add("is-swap");
    const label = document.createElement("span");
    label.textContent = String(value);
    const position = document.createElement("small");
    position.textContent = tracePlayback.algorithm !== "is_sorted"
      ? `from #${tracePlayback.originalIndices[index]}`
      : `#${index}`;
    cell.append(label, position);
    return cell;
  }));
  document.querySelectorAll(".pseudo-line").forEach((line) => {
    line.classList.toggle("is-active", Boolean(event) && line.dataset.nodeId === event.nodeId);
    line.classList.toggle(
      "is-context",
      isLoopContext(
        line.dataset.controlId,
        tracePlayback.algorithm,
        Boolean(tracePlayback.stepper?.done),
      ),
    );
  });
  const showsLoopContext = ["insertion", "reverse"].includes(tracePlayback.algorithm)
    && Boolean(tracePlayback.stepper);
  elements["execution-context"].hidden = !showsLoopContext;
  if (showsLoopContext) {
    if (tracePlayback.algorithm === "reverse") {
      elements["execution-context"].textContent = tracePlayback.stepper.done
        ? `left index ${tracePlayback.stepper.left_index} · loop complete`
        : `left index ${tracePlayback.stepper.left_index} · right index ${tracePlayback.stepper.right_index}`;
    } else {
      elements["execution-context"].textContent = tracePlayback.stepper.done
        ? `outer index ${tracePlayback.stepper.outer_index} · loop complete`
        : `outer index ${tracePlayback.stepper.outer_index} · current index ${tracePlayback.stepper.current_index}`;
    }
  }
  const atEnd = Boolean(tracePlayback.stepper?.done);
  const stoppedEarly = tracePlayback.algorithm === "is_sorted"
    && atEnd
    && !tracePlayback.stepper.sorted;
  elements["trace-progress"].textContent = atEnd
    ? `${stoppedEarly ? "Early stop" : "Complete"} / ${tracePlayback.index + 1} WASM steps`
    : tracePlayback.index < 0 ? "Ready / WASM" : `WASM step ${tracePlayback.index + 1}`;
  elements["trace-event"].textContent = event
    ? traceEventLabel(event)
    : atEnd
      ? tracePlayback.algorithm === "is_sorted"
        ? "No adjacent pair exists; return true without a read or comparison."
        : tracePlayback.algorithm === "reverse"
          ? "No symmetric pair exists; the sequence is already complete."
          : "No insertion step is required; the sequence is already complete."
      : "Initial WASM state; advance to execute the first semantic operation.";
  elements["trace-slider"].value = String(tracePlayback.index + 1);
  updateTraceControls();
}

function updateTraceControls() {
  const available = Boolean(tracePlayback.stepper);
  elements["trace-reset"].disabled = !available || tracePlayback.index < 0;
  elements["trace-previous"].disabled = !available || tracePlayback.index < 0;
  elements["trace-play"].disabled = !available
    || (tracePlayback.stepper.done && tracePlayback.index < 0);
  elements["trace-next"].disabled = !available || tracePlayback.stepper.done;
}

function setTraceIndex(index) {
  seekStepper(index);
}

function readStepperState() {
  const stepper = tracePlayback.stepper;
  tracePlayback.values = Array.from(stepper.values);
  tracePlayback.originalIndices = tracePlayback.algorithm !== "is_sorted"
    ? Array.from(stepper.original_indices)
    : [];
  tracePlayback.index = stepper.steps - 1;
  tracePlayback.operation = stepper.operation_node_id === undefined ? null : {
    nodeId: stepper.operation_node_id,
    operation: stepper.operation_kind,
    leftIndex: stepper.operation_left_index,
    rightIndex: stepper.operation_right_index,
    ordering: stepper.operation_ordering,
  };
  elements["trace-slider"].max = String(stepper.steps);
  elements["trace-slider"].value = String(stepper.steps);
}

function prepareStepper(values, algorithm) {
  clearTrace("Preparing the incremental WASM execution.");
  const algorithmId = algorithmUi[algorithm].id;
  const dynamics = projection.dynamics.find((item) => item.algorithm_id === algorithmId);
  if (values.length > dynamics.max_interactive_input_length) {
    clearTrace(`Scale input (${values.length} values): interactive execution is bounded to ${dynamics.max_interactive_input_length}.`);
    return;
  }
  tracePlayback.algorithm = algorithm;
  tracePlayback.input = [...values];
  const input = new Int32Array(values);
  tracePlayback.stepper = algorithm === "insertion"
    ? new InsertionSortStepper(input)
    : algorithm === "reverse" ? new ReverseStepper(input) : new IsSortedStepper(input);
  readStepperState();
  renderTraceState();
}

function seekStepper(index) {
  const target = Math.max(-1, index);
  tracePlayback.stepper.reset(new Int32Array(tracePlayback.input));
  for (let step = 0; step <= target && tracePlayback.stepper.step(); step += 1) {}
  readStepperState();
  renderTraceState();
}

function advancePlayback() {
  if (!tracePlayback.stepper.step()) return false;
  readStepperState();
  renderTraceState();
  return true;
}

function renderPlaybackSpeeds() {
  elements["trace-speed"].replaceChildren(...PLAYBACK_SPEEDS.map((speed) => {
    const option = document.createElement("option");
    option.value = String(speed.delayMilliseconds);
    option.textContent = speed.label;
    option.selected = Boolean(speed.selected);
    return option;
  }));
}

function schedulePlaybackStep() {
  tracePlayback.timer = window.setTimeout(() => {
    tracePlayback.timer = null;
    if (!advancePlayback()) {
      stopTracePlayback();
      return;
    }
    schedulePlaybackStep();
  }, playbackDelay(elements["trace-speed"].value));
}

function datasetOptionLabel(dataset) {
  const name = dataset.case_id.split(".").at(-1).replaceAll("_", " ");
  return `${dataset.class[0].toUpperCase()}${dataset.class.slice(1)} - ${name}`;
}

function selectDataset(caseId) {
  const dataset = projection.datasets.find((item) => item.case_id === caseId);
  if (!dataset) throw new Error(`derived projection is missing dataset case ${caseId}`);
  elements["dataset-select"].value = dataset.case_id;
  elements["sequence-input"].value = dataset.values.join(", ");
  elements["dataset-context"].textContent = `${dataset.spec_id} for ${dataset.problem_id}; ${dataset.class}; seed ${dataset.seed}; sha256 ${dataset.content_digest_sha256}`;
  generatedInput = null;
  setEditedInputPending(false);
}

function populateDatasets() {
  const custom = document.createElement("option");
  custom.value = "";
  custom.textContent = "Custom or generated input";
  custom.disabled = true;
  elements["dataset-select"].replaceChildren(custom, ...projection.datasets.map((dataset) => {
    const option = document.createElement("option");
    option.value = dataset.case_id;
    option.textContent = datasetOptionLabel(dataset);
    return option;
  }));
  selectDataset(algorithmUi[activeAlgorithm].dataset);
}

function parseSequence() {
  const source = elements["sequence-input"].value.trim();
  if (source === "") return [];
  const tokens = source.split(/[\s,;]+/).filter(Boolean);
  return tokens.map((token) => {
    const value = Number(token);
    if (!Number.isInteger(value) || value < -2147483648 || value > 2147483647) {
      throw new Error(`${token} is not a signed 32-bit integer`);
    }
    return value;
  });
}

function setRuntimeStatus(label, state) {
  elements["runtime-status"].textContent = label;
  elements["runtime-status"].className = `runtime-status${state ? ` is-${state}` : ""}`;
}

function setEditedInputPending(pending) {
  elements["run-button"].disabled = !pending;
}

function refreshRandomSeed() {
  elements["generator-seed"].value = String(randomSeed());
}

function syncSeedMode(refresh) {
  const random = elements["generator-random-seed"].checked;
  elements["generator-seed"].disabled = random;
  if (random && refresh) refreshRandomSeed();
}

function renderSequence(values, highlight = {}) {
  const visual = elements["sequence-visual"];
  visual.replaceChildren();
  const displayLimit = 128;
  const displayedValues = values.slice(0, displayLimit);
  elements["sequence-note"].textContent = values.length > displayLimit
    ? `Scale overview: first ${displayLimit} of ${values.length} values shown; counters use the complete sequence.`
    : "";
  if (values.length === 0) {
    const empty = document.createElement("span");
    empty.className = "empty-state";
    empty.textContent = "Empty sequence";
    visual.append(empty);
    return;
  }
  const scale = Math.max(...displayedValues.map((value) => Math.abs(value)), 1);
  displayedValues.forEach((value, index) => {
    const column = document.createElement("div");
    column.className = "value-column";
    if (highlight.firstInversion !== undefined
      && (index === highlight.firstInversion || index === highlight.firstInversion - 1)) {
      column.classList.add("is-inversion");
    }
    if (highlight.originalIndices && highlight.originalIndices[index] !== index) column.classList.add("is-moved");
    const bar = document.createElement("div");
    bar.className = "value-bar";
    bar.style.height = `${Math.max(8, Math.round((Math.abs(value) / scale) * 130))}px`;
    bar.title = `Index ${index}: ${value}`;
    const label = document.createElement("span");
    label.textContent = String(value);
    column.append(bar, label);
    if (highlight.originalIndices) {
      const origin = document.createElement("small");
      origin.textContent = `from #${highlight.originalIndices[index]}`;
      column.append(origin);
    }
    visual.append(column);
  });
}

function measureLocalCall(runOnce, sampleBit, expectedBit) {
  let iterations = 1;
  let executedIterations = 0;
  let elapsedMilliseconds = 0;
  let checksum = 0;

  while (true) {
    executedIterations = iterations;
    checksum = 0;
    const start = performance.now();
    for (let index = 0; index < iterations; index += 1) {
      const observation = runOnce();
      checksum ^= sampleBit(observation);
      observation.free();
    }
    elapsedMilliseconds = performance.now() - start;
    if (elapsedMilliseconds >= 12 || iterations >= 131072) break;
    iterations *= 2;
  }
  const expectedChecksum = executedIterations % 2 === 1 ? expectedBit : 0;
  if (checksum !== expectedChecksum) {
    throw new Error("repeated WASM observation changed its result");
  }
  return {
    elapsedMilliseconds,
    iterations: executedIterations,
    microsecondsPerCall: elapsedMilliseconds > 0
      ? (elapsedMilliseconds * 1000) / executedIterations
      : null,
  };
}

function displayTiming(timing, boundary) {
  elements["local-time"].textContent = timing.microsecondsPerCall === null
    ? "Below timer resolution"
    : `${timing.microsecondsPerCall.toFixed(2)} us/call`;
  elements["runtime-context"].textContent = `${timing.iterations} repeated ${boundary} calls in ${timing.elapsedMilliseconds.toFixed(1)} ms; ${navigator.userAgent}. Not algorithm-only or portable benchmark evidence.`;
}

function runIsSorted(values) {
  const input = new Int32Array(values);
  const observation = observe_is_sorted_i32(input);
  const firstInversion = observation.first_inversion;
  const expectedBit = observation.sorted ? 1 : 0;
  const timing = measureLocalCall(
    () => observe_is_sorted_i32(input),
    (sample) => sample.sorted ? 1 : 0,
    expectedBit,
  );
  elements["sorted-result"].textContent = observation.sorted ? "Sorted" : "Not sorted";
  elements["sorted-result"].className = observation.sorted ? "is-true" : "is-false";
  elements["comparison-count"].textContent = String(observation.comparisons);
  elements["inversion-index"].textContent = firstInversion ?? "None";
  renderSequence(values, { firstInversion });
  prepareStepper(values, "is_sorted");
  displayTiming(timing, "JS/WASM observation");
  observation.free();
}

function runInsertionSort(values) {
  const input = new Int32Array(values);
  const observation = observe_insertion_sort_i32(input);
  const output = Array.from(observation.values);
  const originalIndices = Array.from(observation.original_indices);
  const sorted = output.every((value, index) => index === 0 || output[index - 1] <= value);
  const sortedIndices = [...originalIndices].sort((left, right) => left - right);
  const permutation = originalIndices.length === values.length
    && sortedIndices.every((originalIndex, index) => originalIndex === index);
  const valuesMatchOrigins = output.length === values.length
    && output.every((value, index) => values[originalIndices[index]] === value);
  const stable = output.every((value, index) => index === 0
    || output[index - 1] !== value
    || originalIndices[index - 1] < originalIndices[index]);
  const correct = sorted && permutation && valuesMatchOrigins && stable;
  const expectedBit = observation.comparisons & 1;
  const timing = measureLocalCall(
    () => observe_insertion_sort_i32(input),
    (sample) => sample.comparisons & 1,
    expectedBit,
  );
  elements["sorted-result"].textContent = correct ? "Stable sorted" : "Correction failed";
  elements["sorted-result"].className = correct ? "is-true" : "is-false";
  elements["comparison-count"].textContent = String(observation.comparisons);
  elements["inversion-index"].textContent = String(observation.swaps);
  renderSequence(output, { originalIndices });
  prepareStepper(values, "insertion");
  displayTiming(timing, "JS/WASM sort observation");
  observation.free();
}

function runReverse(values) {
  const input = new Int32Array(values);
  const observation = observe_reverse_i32(input);
  const output = Array.from(observation.values);
  const expected = [...values].reverse();
  const restored = observe_reverse_i32(observation.values);
  const correct = output.every((value, index) => value === expected[index])
    && output.length === expected.length
    && Array.from(restored.values).every((value, index) => value === values[index]);
  const expectedBit = observation.swaps & 1;
  const timing = measureLocalCall(
    () => observe_reverse_i32(input),
    (sample) => sample.swaps & 1,
    expectedBit,
  );
  elements["sorted-result"].textContent = correct ? "Reversed + restored" : "Correction failed";
  elements["sorted-result"].className = correct ? "is-true" : "is-false";
  elements["comparison-count"].textContent = `${observation.reads} / ${observation.writes}`;
  elements["inversion-index"].textContent = String(observation.swaps);
  renderSequence(output, { originalIndices: values.map((_, index) => values.length - 1 - index) });
  prepareStepper(values, "reverse");
  displayTiming(timing, "JS/WASM reverse observation");
  restored.free();
  observation.free();
}

function runObservation() {
  try {
    const values = parseSequence();
    elements["input-count"].textContent = `${values.length} value${values.length === 1 ? "" : "s"}`;
    if (!wasmReady) throw new Error("WebAssembly runtime is not ready");
    if (activeAlgorithm === "is_sorted") runIsSorted(values);
    else if (activeAlgorithm === "insertion") runInsertionSort(values);
    else runReverse(values);
    if (generatedInput && generatedInput.length > EXPLORE_MAX_LENGTH) renderScaleStudy(generatedInput);
    else elements["scale-panel"].hidden = true;
    setRuntimeStatus("Executed in WASM", "ready");
    return true;
  } catch (error) {
    elements["sorted-result"].textContent = "Invalid input";
    elements["sorted-result"].className = "is-false";
    elements["comparison-count"].textContent = "-";
    elements["inversion-index"].textContent = "-";
    elements["local-time"].textContent = "-";
    elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
    setRuntimeStatus("Input rejected", "error");
    return false;
  }
}

function countSelectedOperation(values) {
  const input = new Int32Array(values);
  if (activeAlgorithm === "is_sorted") {
    const observation = observe_is_sorted_i32(input);
    const count = observation.comparisons;
    observation.free();
    return [count, "Adjacent comparisons"];
  }
  if (activeAlgorithm === "insertion") {
    const observation = observe_insertion_sort_i32(input);
    const count = observation.comparisons;
    observation.free();
    return [count, "Comparisons"];
  }
  const observation = observe_reverse_i32(input);
  const count = observation.swaps;
  observation.free();
  return [count, "Symmetric swaps"];
}

function renderScaleStudy(configuration) {
  const candidates = [8, 16, 32, 64, 128, 512, 2048, 4096];
  const sizes = candidates.filter((size) => size <= configuration.length);
  if (!sizes.includes(configuration.length)) sizes.push(configuration.length);
  const observations = sizes.map((size) => {
    const values = generateSequence(configuration.profile, size, configuration.seed);
    const [count, operation] = countSelectedOperation(values);
    return { size, count, operation };
  });
  const maximum = Math.max(...observations.map((observation) => observation.count), 1);
  elements["scale-chart"].replaceChildren(...observations.map((observation) => {
    const column = document.createElement("div");
    column.className = "scale-column";
    const count = document.createElement("strong");
    count.textContent = observation.count.toLocaleString("en-US");
    const bar = document.createElement("div");
    bar.className = "scale-bar";
    bar.style.height = `${Math.max(4, Math.round((observation.count / maximum) * 150))}px`;
    const size = document.createElement("span");
    size.textContent = `n=${observation.size}`;
    column.append(count, bar, size);
    return column;
  }));
  elements["scale-operation"].textContent = observations[0].operation;
  elements["scale-note"].textContent = `${configuration.profile}; seed ${configuration.seed}. Exact semantic counts over complete generated sequences. This illustrates input-dependent growth; the sourced complexity claim above remains authoritative.`;
  elements["scale-panel"].hidden = false;
}

function generateFromControls() {
  try {
    const profile = elements["generator-profile"].value;
    const length = Number(elements["generator-size"].value);
    if (elements["generator-random-seed"].checked) refreshRandomSeed();
    const seed = Number(elements["generator-seed"].value);
    const values = generateSequence(profile, length, seed);
    generatedInput = { profile, length, seed };
    setEditedInputPending(false);
    elements["dataset-select"].value = "";
    elements["sequence-input"].value = values.join(", ");
    const regime = length <= EXPLORE_MAX_LENGTH ? "Explore" : "Scale";
    elements["dataset-context"].textContent = `Generated locally; ${regime}; ${profile}; length ${length}; seed ${seed}; deterministic and ephemeral.`;
    runObservation();
  } catch (error) {
    elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
    setRuntimeStatus("Generator rejected", "error");
  }
}

function claimSummary(entity, kind) {
  if (kind === "problem") return [entity.output.value, entity.output.level];
  if (kind === "algorithm") return [`${entity.time_worst.value} time; ${entity.auxiliary_memory.value} space`, entity.time_worst.level];
  return [`${entity.language.value}; ${entity.target.value}`, entity.entrypoint.level];
}

function renderCatalog() {
  const term = elements["catalog-search"].value.trim().toLowerCase();
  const rows = [
    ...projection.problems.map((entity) => ["problem", entity, "defines"]),
    ...projection.algorithms.map((entity) => ["algorithm", entity, `solves ${entity.solves}`]),
    ...projection.implementations.map((entity) => ["implementation", entity, `implements ${entity.implements}`]),
  ].filter(([, entity, relation]) => `${entity.id} ${relation} ${JSON.stringify(entity)}`.toLowerCase().includes(term));

  elements["catalog-body"].replaceChildren(...rows.map(([kind, entity, relation]) => {
    const row = document.createElement("tr");
    const summary = claimSummary(entity, kind);
    const kindCell = document.createElement("td");
    kindCell.innerHTML = `<span class="kind-label">${kind}</span>`;
    const idCell = document.createElement("td");
    const code = document.createElement("code");
    code.textContent = entity.id;
    idCell.append(code);
    const relationCell = document.createElement("td");
    relationCell.textContent = `${relation}; ${summary[0]}`;
    const evidenceCell = document.createElement("td");
    evidenceCell.textContent = summary[1];
    row.append(kindCell, idCell, relationCell, evidenceCell);
    return row;
  }));
}

function applyProjection() {
  const total = projection.counts.problems + projection.counts.algorithms + projection.counts.implementations;
  elements["entity-count"].textContent = `${total} entities from validated YAML`;
  elements["registry-digest"].textContent = projection.registry_digest;
  elements["source-commit"].textContent = `source ${projection.source_commit}`;
  elements["build-environment"].textContent = `${projection.build.rustc}; ${projection.build.wasm_bindgen}; ${projection.build.target} ${projection.build.profile}`;

  const ui = algorithmUi[activeAlgorithm];
  const algorithm = projection.algorithms.find((item) => item.id === ui.id);
  if (!algorithm) throw new Error(`derived projection is missing ${ui.id}`);
  elements["algorithm-id"].textContent = algorithm.id;
  elements["algorithm-name"].textContent = algorithm.name.value;
  elements["time-complexity"].textContent = algorithm.time_worst.value;
  elements["time-provenance"].textContent = `${algorithm.time_worst.level}: ${algorithm.time_worst.source}`;
  elements["space-complexity"].textContent = algorithm.auxiliary_memory.value;
  elements["space-provenance"].textContent = `${algorithm.auxiliary_memory.level}: ${algorithm.auxiliary_memory.source}`;
  elements["execution-boundary"].textContent = ui.boundary;
  elements["result-label"].textContent = ui.resultLabel;
  elements["comparison-label"].textContent = ui.comparisonLabel;
  elements["secondary-label"].textContent = ui.secondaryLabel;
  elements["sequence-heading"].textContent = ui.sequenceHeading;
  elements["legend-text"].textContent = ui.legend;
  elements["legend-text"].parentElement.classList.toggle("is-moved", ui.moved);
  const dynamics = projection.dynamics.find((item) => item.algorithm_id === algorithm.id);
  elements["dynamics-panel"].hidden = !dynamics;
  if (dynamics) {
    renderPseudocode(dynamics);
  } else {
    clearTrace("No validated semantic execution is exposed for this algorithm.");
  }
  renderCatalog();
}

document.querySelectorAll("[data-algorithm]").forEach((option) => {
  option.addEventListener("click", () => {
    activeAlgorithm = option.dataset.algorithm;
    document.querySelectorAll("[data-algorithm]").forEach((item) => {
      item.classList.toggle("is-active", item === option);
      item.setAttribute("aria-pressed", String(item === option));
    });
    if (!projection) return;
    selectDataset(algorithmUi[activeAlgorithm].dataset);
    applyProjection();
    runObservation();
  });
});

document.querySelectorAll("[data-view]").forEach((tab) => {
  tab.addEventListener("click", () => {
    document.querySelectorAll("[data-view]").forEach((item) => {
      const selected = item === tab;
      item.classList.toggle("is-active", selected);
      item.setAttribute("aria-selected", String(selected));
      document.getElementById(`${item.dataset.view}-view`).hidden = !selected;
    });
  });
});

elements["dataset-select"].addEventListener("change", () => {
  selectDataset(elements["dataset-select"].value);
  runObservation();
});
elements["sequence-input"].addEventListener("input", () => {
  elements["dataset-select"].value = "";
  elements["dataset-context"].textContent = "Custom ephemeral input; no registry evidence.";
  generatedInput = null;
  clearTrace("Input edited; run the algorithm to initialize its semantic execution.");
  setRuntimeStatus("Ready to execute", "");
  setEditedInputPending(true);
  try {
    const count = parseSequence().length;
    elements["input-count"].textContent = `${count} value${count === 1 ? "" : "s"}`;
  } catch {
    elements["input-count"].textContent = "Invalid value";
  }
});
elements["generate-button"].addEventListener("click", generateFromControls);
elements["generator-random-seed"].addEventListener("change", () => syncSeedMode(true));
elements["trace-reset"].addEventListener("click", () => {
  stopTracePlayback();
  setTraceIndex(-1);
});
elements["trace-previous"].addEventListener("click", () => {
  stopTracePlayback();
  setTraceIndex(tracePlayback.index - 1);
});
elements["trace-next"].addEventListener("click", () => {
  stopTracePlayback();
  advancePlayback();
});
elements["trace-play"].addEventListener("click", () => {
  if (tracePlayback.timer !== null) {
    stopTracePlayback();
    return;
  }
  if (tracePlayback.stepper.done) setTraceIndex(-1);
  elements["trace-play"].textContent = "Pause";
  schedulePlaybackStep();
});
elements["trace-speed"].addEventListener("change", () => {
  if (tracePlayback.timer === null) return;
  window.clearTimeout(tracePlayback.timer);
  tracePlayback.timer = null;
  schedulePlaybackStep();
});
elements["trace-slider"].addEventListener("input", () => {
  stopTracePlayback();
  setTraceIndex(Number(elements["trace-slider"].value) - 1);
});
elements["dynamics-panel"].addEventListener("keydown", (event) => {
  if (event.target instanceof HTMLInputElement || event.target instanceof HTMLSelectElement) return;
  if (event.key === "ArrowLeft") {
    event.preventDefault();
    stopTracePlayback();
    setTraceIndex(tracePlayback.index - 1);
  } else if (event.key === "ArrowRight") {
    event.preventDefault();
    stopTracePlayback();
    setTraceIndex(tracePlayback.index + 1);
  } else if (event.key === " ") {
    event.preventDefault();
    elements["trace-play"].click();
  }
});
elements["run-button"].addEventListener("click", () => {
  if (runObservation()) setEditedInputPending(false);
});
elements["catalog-search"].addEventListener("input", renderCatalog);

const query = new URLSearchParams(window.location.search);
renderPlaybackSpeeds();
syncSeedMode(true);
const requestedAlgorithm = query.get("algorithm");
if (requestedAlgorithm && algorithmUi[requestedAlgorithm]) {
  activeAlgorithm = requestedAlgorithm;
  document.querySelectorAll("[data-algorithm]").forEach((item) => {
    const selected = item.dataset.algorithm === activeAlgorithm;
    item.classList.toggle("is-active", selected);
    item.setAttribute("aria-pressed", String(selected));
  });
}

try {
  const [projectionResponse] = await Promise.all([
    fetch("./data/atlas.json"),
    init("./pkg/atlas_web_bg.wasm"),
  ]);
  if (!projectionResponse.ok) throw new Error(`cannot load registry projection (${projectionResponse.status})`);
  projection = await projectionResponse.json();
  populateDatasets();
  applyProjection();
  wasmReady = true;
  setRuntimeStatus("WASM ready", "ready");
  const requestedProfile = query.get("profile");
  const requestedSize = query.get("size");
  const requestedSeed = query.get("seed");
  const profileExists = [...elements["generator-profile"].options]
    .some((option) => option.value === requestedProfile);
  const sizeExists = [...elements["generator-size"].options]
    .some((option) => option.value === requestedSize);
  if (profileExists && sizeExists && /^\d+$/.test(requestedSeed ?? "")) {
    elements["generator-profile"].value = requestedProfile;
    elements["generator-size"].value = requestedSize;
    elements["generator-seed"].value = requestedSeed;
    elements["generator-random-seed"].checked = false;
    syncSeedMode(false);
    generateFromControls();
  } else {
    runObservation();
  }
} catch (error) {
  setRuntimeStatus("Runtime unavailable", "error");
  elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
  renderSequence([]);
}
