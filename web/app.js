import init, { VisualMachine } from "./pkg/atlas_web.js";
import {
  catalogRecords,
  claimEntries,
  comparableRows,
  displayName,
  executablePresentation,
  filterCatalog,
  findRecord,
  relatedRecords,
} from "./catalog.mjs";
import { EXPLORE_MAX_LENGTH, generateSequence, randomSeed } from "./generator.mjs";
import { PLAYBACK_SPEEDS, isLoopContext, playbackDelay } from "./playback.mjs";

const algorithmUi = {};

function hydrateGeneratedAlgorithms() {
  for (const dynamics of projection.dynamics) {
    const presentation = dynamics.presentation;
    if (!presentation || !dynamics.program) continue;
    algorithmUi[presentation.key] = {
      id: dynamics.algorithm_id,
      dataset: presentation.default_dataset,
      boundary: presentation.boundary,
      resultLabel: presentation.result_label,
      comparisonLabel: presentation.primary_counter_label,
      secondaryLabel: presentation.secondary_label,
      sequenceHeading: presentation.sequence_heading,
      legend: presentation.legend,
      moved: presentation.tracks_origins,
      datasetProblem: presentation.dataset_problem_id,
      datasetPredicate: presentation.dataset_predicate,
      comparisonInterest: presentation.comparison_interest,
      resultView: presentation.result_view,
      primaryCounter: presentation.primary_counter,
      secondaryCounter: presentation.secondary_counter,
      highlight: presentation.highlight,
      predicateLabel: presentation.predicate_label,
      dynamics,
    };
    if (document.querySelector(`[data-algorithm="${presentation.key}"]`)) continue;
    const option = document.createElement("button");
    option.className = "algorithm-option";
    option.type = "button";
    option.dataset.algorithm = presentation.key;
    option.setAttribute("aria-pressed", "false");
    option.textContent = presentation.selector_label;
    document.querySelector(".algorithm-selector").append(option);
  }
}

function selectAlgorithm(key) {
  activeAlgorithm = key;
  document.querySelectorAll("[data-algorithm]").forEach((item) => {
    const selected = item.dataset.algorithm === activeAlgorithm;
    item.classList.toggle("is-active", selected);
    item.setAttribute("aria-pressed", String(selected));
  });
}

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
    "catalog-search", "catalog-kind", "catalog-result-count", "catalog-results",
    "entity-detail-kind", "entity-detail-name", "entity-detail-id", "entity-compare",
    "entity-execute", "entity-relation-list", "entity-claim-list", "comparison-panel",
    "compare-left", "compare-right", "comparison-close", "comparison-grid",
    "entity-execution-status", "execution-evidence-link",
  ].map((id) => [id, document.getElementById(id)]),
);

let projection;
let wasmReady = false;
let activeAlgorithm = "is_sorted";
let generatedInput = null;
let selectedEntityId;
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
    const relation = tracePlayback.algorithm === "reverse"
      ? "symmetric"
      : algorithmUi[tracePlayback.algorithm].resultView === "partition_boundary"
        ? "partition"
        : "adjacent";
    return `${event.nodeId}: swap ${relation} positions #${event.leftIndex} and #${event.rightIndex}; the WASM state is now updated.`;
  }
  if (event.operation === "Predicate") {
    const value = tracePlayback.values[event.leftIndex];
    return `${event.nodeId}: ${algorithmUi[tracePlayback.algorithm].predicateLabel}(${value}) is ${event.predicateResult}; continue the corresponding scan.`;
  }
  if (event.operation === "Partition") {
    return `${event.nodeId}: return boundary #${event.leftIndex}; matching values precede it.`;
  }
  const symbols = ["<", "=", ">"];
  const comparison = `${event.nodeId}: compare values[${event.leftIndex}] ${symbols[event.ordering + 1]} values[${event.rightIndex}]`;
  if (algorithmUi[tracePlayback.algorithm].comparisonInterest === "less") {
    return event.ordering < 0
      ? `${comparison}; candidate #${event.leftIndex} becomes the first minimum.`
      : `${comparison}; retain the earlier minimum at #${event.rightIndex}.`;
  }
  if (tracePlayback.algorithm === "insertion") {
    return event.ordering < 0
      ? `${comparison}; the current element must move left.`
      : `${comparison}; this insertion position is stable.`;
  }
  if (event.ordering > 0) {
    const firstInversion = tracePlayback.stepper.result_index
      ?? tracePlayback.stepper.first_inversion;
    return `${comparison}; inversion at #${firstInversion}, return false and stop early.`;
  }
  return tracePlayback.stepper.done
    ? `${comparison}; scan complete, return true.`
    : `${comparison}; continue with the next adjacent pair.`;
}

function renderTraceState() {
  const event = tracePlayback.operation;
  elements["trace-sequence"].classList.toggle("is-wrapped", tracePlayback.values.length > 16);
  elements["trace-sequence"].replaceChildren(...tracePlayback.values.map((value, index) => {
    const cell = document.createElement("div");
    cell.className = "trace-cell";
    if (event?.operation === "Read" && index === event.leftIndex) cell.classList.add("is-read");
    if (event?.operation === "Predicate" && index === event.leftIndex) {
      cell.classList.add(event.predicateResult ? "is-compare" : "is-inversion");
    }
    if (event?.operation === "Compare"
      && (index === event.leftIndex || index === event.rightIndex)) {
      const inversion = algorithmUi[tracePlayback.algorithm].comparisonInterest === "less"
        ? event.ordering < 0
        : tracePlayback.algorithm === "insertion" ? event.ordering < 0 : event.ordering > 0;
      cell.classList.add(inversion
        ? "is-inversion"
        : "is-compare");
    }
    if (event?.operation === "Swap"
      && (index === event.leftIndex || index === event.rightIndex)) cell.classList.add("is-swap");
    const label = document.createElement("span");
    label.textContent = String(value);
    const position = document.createElement("small");
    position.textContent = algorithmUi[tracePlayback.algorithm].moved
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
      const leftIndex = tracePlayback.stepper.register_value("left");
      const rightIndex = tracePlayback.stepper.register_value("right");
      elements["execution-context"].textContent = tracePlayback.stepper.done
        ? `left index ${leftIndex} · loop complete`
        : `left index ${leftIndex} · right index ${rightIndex}`;
    } else {
      const outerIndex = tracePlayback.stepper.register_value("index");
      const currentIndex = tracePlayback.stepper.register_value("current");
      elements["execution-context"].textContent = tracePlayback.stepper.done
        ? `outer index ${outerIndex} · loop complete`
        : `outer index ${outerIndex} · current index ${currentIndex}`;
    }
  }
  const atEnd = Boolean(tracePlayback.stepper?.done);
  const stoppedEarly = tracePlayback.algorithm === "is_sorted"
    && atEnd
    && tracePlayback.stepper.has_result;
  elements["trace-progress"].textContent = atEnd
    ? `${stoppedEarly ? "Early stop" : "Complete"} / ${tracePlayback.index + 1} WASM steps`
    : tracePlayback.index < 0 ? "Ready / WASM" : `WASM step ${tracePlayback.index + 1}`;
  elements["trace-event"].textContent = event
    ? traceEventLabel(event)
    : atEnd
      ? algorithmUi[tracePlayback.algorithm].resultView === "partition_boundary"
        ? `Partition complete at boundary #${tracePlayback.stepper.result_index}.`
        : tracePlayback.algorithm === "minimum"
        ? tracePlayback.stepper.has_result
          ? `Return the first minimum, ${tracePlayback.stepper.result_value}, at #${tracePlayback.stepper.result_index}.`
          : "Empty input has no minimum; return none."
        : tracePlayback.algorithm === "is_sorted"
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
  tracePlayback.originalIndices = algorithmUi[tracePlayback.algorithm].moved
    ? Array.from(stepper.original_indices)
    : [];
  tracePlayback.index = stepper.steps - 1;
  tracePlayback.operation = stepper.operation_node_id === undefined ? null : {
    nodeId: stepper.operation_node_id,
    operation: stepper.operation_kind,
    leftIndex: stepper.operation_left_index,
    rightIndex: stepper.operation_right_index,
    ordering: stepper.operation_ordering,
    predicateResult: stepper.operation_kind === "Predicate" ? stepper.predicate_result : undefined,
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
  const ui = algorithmUi[algorithm];
  tracePlayback.stepper = new VisualMachine(
    JSON.stringify(ui.dynamics.program),
    new Int32Array(values),
  );
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
  const predicate = dataset.predicate ? `; predicate ${dataset.predicate}` : "";
  elements["dataset-context"].textContent = `${dataset.spec_id} for ${dataset.problem_id}; ${dataset.class}${predicate}; seed ${dataset.seed}; sha256 ${dataset.content_digest_sha256}`;
  generatedInput = null;
  setEditedInputPending(false);
}

function populateDatasets() {
  const custom = document.createElement("option");
  custom.value = "";
  custom.textContent = "Custom or generated input";
  custom.disabled = true;
  const ui = algorithmUi[activeAlgorithm];
  const datasets = projection.datasets.filter((dataset) => dataset.problem_id === ui.datasetProblem
    && (ui.datasetPredicate === undefined || dataset.predicate === ui.datasetPredicate));
  elements["dataset-select"].replaceChildren(custom, ...datasets.map((dataset) => {
    const option = document.createElement("option");
    option.value = dataset.case_id;
    option.textContent = datasetOptionLabel(dataset);
    return option;
  }));
  selectDataset(ui.dataset);
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
    if (highlight.selectedIndex === index) column.classList.add("is-selected");
    if (highlight.partitionBoundary !== undefined) {
      column.classList.add(index < highlight.partitionBoundary ? "is-matching" : "is-rejected");
      if (index === highlight.partitionBoundary) column.classList.add("is-boundary");
    }
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

function stableSortIsCorrect(values, output, originalIndices) {
  const sorted = output.every((value, index) => index === 0 || output[index - 1] <= value);
  const sortedIndices = [...originalIndices].sort((left, right) => left - right);
  const permutation = originalIndices.length === values.length
    && sortedIndices.every((originalIndex, index) => originalIndex === index);
  const valuesMatchOrigins = output.length === values.length
    && output.every((value, index) => values[originalIndices[index]] === value);
  const stable = output.every((value, index) => index === 0
    || output[index - 1] !== value
    || originalIndices[index - 1] < originalIndices[index]);
  return sorted && permutation && valuesMatchOrigins && stable;
}

function completedVisualMachine(values) {
  const ui = algorithmUi[activeAlgorithm];
  const machine = new VisualMachine(
    JSON.stringify(ui.dynamics.program),
    new Int32Array(values),
  );
  while (machine.step()) {}
  return machine;
}

function visualResultBit(observation, ui) {
  if (ui.resultView === "sortedness") return observation.has_result ? 2 : 1;
  if (ui.resultView === "stable_sorted") return (observation.comparisons % 251) + 1;
  if (ui.resultView === "reversed") return (observation.swaps % 251) + 1;
  return observation.has_result ? ((observation.result_index ?? 0) % 251) + 1 : 0;
}

function runGeneratedAlgorithm(values) {
  const ui = algorithmUi[activeAlgorithm];
  const observation = completedVisualMachine(values);
  const resultIndex = observation.result_index;
  const resultValue = observation.result_value;
  const expectedBit = visualResultBit(observation, ui);
  const timing = measureLocalCall(
    () => completedVisualMachine(values),
    (sample) => visualResultBit(sample, ui),
    expectedBit,
  );
  const partition = ui.resultView === "partition_boundary";
  const sortedness = ui.resultView === "sortedness";
  const stableSorted = ui.resultView === "stable_sorted";
  const reversed = ui.resultView === "reversed";
  const output = Array.from(observation.values);
  const originalIndices = ui.moved ? Array.from(observation.original_indices) : undefined;
  let restored;
  const correct = stableSorted
    ? stableSortIsCorrect(values, output, originalIndices)
    : reversed
      ? (() => {
        restored = completedVisualMachine(output);
        return output.length === values.length
          && output.every((value, index) => value === values[values.length - 1 - index])
          && originalIndices.every((origin, index) => origin === values.length - 1 - index)
          && Array.from(restored.values).every((value, index) => value === values[index]);
      })()
    : undefined;
  elements["sorted-result"].textContent = stableSorted || reversed
    ? correct
      ? reversed ? "Reversed + restored" : "Stable sorted"
      : "Correction failed"
    : partition
    ? String(resultIndex)
    : sortedness
      ? observation.has_result ? "Not sorted" : "Sorted"
      : observation.has_result ? String(resultValue) : "None";
  elements["sorted-result"].className = stableSorted || reversed
    ? correct ? "is-true" : "is-false"
    : sortedness
    ? observation.has_result ? "is-false" : "is-true"
    : observation.has_result ? "is-true" : "";
  elements["comparison-count"].textContent = reversed
    ? `${observation.reads} / ${observation.writes}`
    : String(observation[ui.primaryCounter]);
  elements["inversion-index"].textContent = observation[ui.secondaryCounter] ?? "None";
  renderSequence(output, stableSorted || reversed
    ? { originalIndices }
    : partition
    ? {
      partitionBoundary: resultIndex,
      originalIndices,
    }
    : sortedness ? { firstInversion: resultIndex } : { selectedIndex: resultIndex });
  prepareStepper(values, activeAlgorithm);
  displayTiming(timing, "generated-program construction and WASM execution");
  restored?.free();
  observation.free();
}

function runObservation() {
  try {
    const values = parseSequence();
    elements["input-count"].textContent = `${values.length} value${values.length === 1 ? "" : "s"}`;
    if (!wasmReady) throw new Error("WebAssembly runtime is not ready");
    runGeneratedAlgorithm(values);
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
  const observation = completedVisualMachine(values);
  const count = observation[algorithmUi[activeAlgorithm].primaryCounter];
  observation.free();
  return [count, algorithmUi[activeAlgorithm].comparisonLabel];
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
    const predicate = algorithmUi[activeAlgorithm].predicateLabel
      ? `; predicate ${algorithmUi[activeAlgorithm].predicateLabel}`
      : "";
    elements["dataset-context"].textContent = `Generated locally; ${regime}; ${profile}; length ${length}; seed ${seed}${predicate}; deterministic and ephemeral.`;
    runObservation();
  } catch (error) {
    elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
    setRuntimeStatus("Generator rejected", "error");
  }
}

function formatValue(value) {
  if (Array.isArray(value)) return value.length === 0 ? "None" : value.join("\n");
  if (value && typeof value === "object") {
    return [
      `mutates: ${value.mutates.length === 0 ? "none" : value.mutates.join(", ")}`,
      `I/O: ${value.io}`,
      `blocking: ${value.blocking}`,
      `allocation: ${value.allocation}`,
    ].join("\n");
  }
  return String(value);
}

function claimFact(claim, absentLabel = "Not recorded") {
  const fact = document.createElement("div");
  fact.className = "claim-fact";
  if (!claim) {
    fact.classList.add("is-absent");
    fact.textContent = absentLabel;
    return fact;
  }
  const value = document.createElement("div");
  value.className = "claim-value";
  value.textContent = formatValue(claim.value);
  const metadata = document.createElement("div");
  metadata.className = "claim-metadata";
  const level = document.createElement("span");
  level.className = `evidence-level is-${claim.level}`;
  level.textContent = claim.level;
  const source = document.createElement("code");
  source.textContent = claim.source;
  metadata.append(level, source);
  fact.append(value, metadata);
  return fact;
}

function renderClaimProvenance(container, claim) {
  const level = document.createElement("span");
  level.className = `evidence-level is-${claim.level}`;
  level.textContent = claim.level;
  const source = document.createElement("code");
  source.textContent = claim.source;
  container.replaceChildren(level, source);
}

function relationButton(relation) {
  const button = document.createElement("button");
  button.className = "entity-link";
  button.type = "button";
  button.dataset.entityId = relation.entity.id;
  const label = document.createElement("span");
  label.textContent = relation.relation;
  const id = document.createElement("code");
  id.textContent = relation.entity.id;
  button.append(label, id);
  return button;
}

function renderEntityDetail(record) {
  elements["entity-detail-kind"].textContent = record.kind;
  elements["entity-detail-name"].textContent = displayName(record);
  elements["entity-detail-id"].textContent = record.entity.id;

  const relations = relatedRecords(projection, record);
  elements["entity-relation-list"].replaceChildren(...(relations.length > 0
    ? relations.map(relationButton)
    : [Object.assign(document.createElement("p"), {
      className: "empty-relation",
      textContent: "No related entity is recorded.",
    })]));

  elements["entity-claim-list"].replaceChildren(...claimEntries(record).map((entry) => {
    const row = document.createElement("div");
    row.className = "entity-claim";
    const label = document.createElement("h5");
    label.textContent = entry.label;
    row.append(label, claimFact(entry.claim));
    return row;
  }));

  const presentation = executablePresentation(projection, record);
  const algorithm = record.kind === "algorithm";
  elements["entity-execution-status"].hidden = !algorithm;
  elements["entity-execution-status"].textContent = presentation
    ? "Interactive WASM model available; implementation evidence is separate"
    : "No interactive WASM model; implementation evidence remains available";
  elements["entity-execution-status"].classList.toggle("is-available", Boolean(presentation));
  elements["entity-execution-status"].classList.toggle("is-unavailable", algorithm && !presentation);
  elements["entity-execute"].hidden = !presentation;
  elements["entity-execute"].dataset.presentationKey = presentation?.key ?? "";
}

function renderComparisonOptions(record) {
  const records = catalogRecords(projection).filter((candidate) => candidate.kind === record.kind);
  const options = (selectedId) => records.map((candidate) => {
    const option = document.createElement("option");
    option.value = candidate.entity.id;
    option.textContent = `${displayName(candidate)} - ${candidate.entity.id}`;
    option.selected = candidate.entity.id === selectedId;
    return option;
  });
  const other = records.find((candidate) => candidate.entity.id !== record.entity.id) ?? record;
  elements["compare-left"].replaceChildren(...options(record.entity.id));
  elements["compare-right"].replaceChildren(...options(other.entity.id));
}

function renderComparison() {
  const left = findRecord(projection, elements["compare-left"].value);
  const right = findRecord(projection, elements["compare-right"].value);
  if (!left || !right) return;
  elements["comparison-grid"].replaceChildren(...comparableRows(left, right).map((comparison) => {
    const row = document.createElement("div");
    row.className = "comparison-row";
    const label = document.createElement("h4");
    label.textContent = comparison.label;
    row.append(label, claimFact(comparison.left), claimFact(comparison.right));
    return row;
  }));
}

function openComparison(rightId) {
  const record = findRecord(projection, selectedEntityId);
  if (!record) return;
  renderComparisonOptions(record);
  const right = findRecord(projection, rightId);
  if (right?.kind === record.kind) elements["compare-right"].value = right.entity.id;
  renderComparison();
  elements["comparison-panel"].hidden = false;
}

function selectCatalogEntity(id, reveal = false) {
  const record = findRecord(projection, id);
  if (!record) return;
  selectedEntityId = id;
  renderEntityDetail(record);
  let selectedItem;
  elements["catalog-results"].querySelectorAll("[data-entity-id]").forEach((item) => {
    const selected = item.dataset.entityId === id;
    item.classList.toggle("is-selected", selected);
    item.setAttribute("aria-selected", String(selected));
    if (selected) selectedItem = item;
  });
  if (!reveal || !selectedItem) return;
  const results = elements["catalog-results"];
  const itemTop = selectedItem.offsetTop;
  const itemBottom = itemTop + selectedItem.offsetHeight;
  if (itemTop < results.scrollTop || itemBottom > results.scrollTop + results.clientHeight) {
    results.scrollTop = itemTop - (results.clientHeight - selectedItem.offsetHeight) / 2;
  }
}

function renderCatalog() {
  const records = filterCatalog(
    projection,
    elements["catalog-search"].value,
    elements["catalog-kind"].value,
  );
  elements["catalog-result-count"].textContent = `${records.length} result${records.length === 1 ? "" : "s"}`;
  elements["catalog-results"].replaceChildren(...records.map((record) => {
    const button = document.createElement("button");
    button.className = "catalog-result";
    button.type = "button";
    button.dataset.entityId = record.entity.id;
    button.setAttribute("role", "option");
    const kind = document.createElement("span");
    kind.className = "kind-label";
    kind.textContent = record.kind;
    const name = document.createElement("strong");
    name.textContent = displayName(record);
    const id = document.createElement("code");
    id.textContent = record.entity.id;
    button.append(kind, name, id);
    return button;
  }));

  if (records.length === 0) {
    const empty = document.createElement("p");
    empty.className = "catalog-empty";
    empty.textContent = "No registry entity matches these filters.";
    elements["catalog-results"].append(empty);
    return;
  }
  if (!records.some((record) => record.entity.id === selectedEntityId)) {
    selectedEntityId = records[0].entity.id;
  }
  selectCatalogEntity(selectedEntityId, true);
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
  renderClaimProvenance(elements["time-provenance"], algorithm.time_worst);
  elements["space-complexity"].textContent = algorithm.auxiliary_memory.value;
  renderClaimProvenance(elements["space-provenance"], algorithm.auxiliary_memory);
  elements["execution-boundary"].textContent = ui.boundary;
  elements["result-label"].textContent = ui.resultLabel;
  elements["comparison-label"].textContent = ui.comparisonLabel;
  elements["secondary-label"].textContent = ui.secondaryLabel;
  elements["sequence-heading"].textContent = ui.sequenceHeading;
  elements["legend-text"].textContent = ui.legend;
  elements["legend-text"].parentElement.classList.toggle("is-moved", ui.moved);
  elements["legend-text"].parentElement.classList.add("is-selected");
  const dynamics = projection.dynamics.find((item) => item.algorithm_id === algorithm.id);
  elements["dynamics-panel"].hidden = !dynamics;
  if (dynamics) {
    renderPseudocode(dynamics);
  } else {
    clearTrace("No validated semantic execution is exposed for this algorithm.");
  }
  renderCatalog();
}

function setView(view) {
  document.querySelectorAll("[data-view]").forEach((item) => {
    const selected = item.dataset.view === view;
    item.classList.toggle("is-active", selected);
    item.setAttribute("aria-selected", String(selected));
    document.getElementById(`${item.dataset.view}-view`).hidden = !selected;
  });
  if (view === "catalog" && projection && selectedEntityId) {
    selectCatalogEntity(selectedEntityId, true);
  }
}

document.querySelector(".algorithm-selector").addEventListener("click", (event) => {
  const option = event.target.closest("[data-algorithm]");
  if (!option) return;
  selectAlgorithm(option.dataset.algorithm);
  if (!projection) return;
  populateDatasets();
  applyProjection();
  runObservation();
});

document.querySelectorAll("[data-view]").forEach((tab) => {
  tab.addEventListener("click", () => setView(tab.dataset.view));
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
elements["catalog-kind"].addEventListener("change", renderCatalog);
elements["catalog-results"].addEventListener("click", (event) => {
  const item = event.target.closest("[data-entity-id]");
  if (item) selectCatalogEntity(item.dataset.entityId);
});
elements["entity-relation-list"].addEventListener("click", (event) => {
  const item = event.target.closest("[data-entity-id]");
  if (!item) return;
  elements["catalog-search"].value = "";
  elements["catalog-kind"].value = "all";
  renderCatalog();
  selectCatalogEntity(item.dataset.entityId, true);
});
elements["entity-compare"].addEventListener("click", () => openComparison());
elements["comparison-close"].addEventListener("click", () => {
  elements["comparison-panel"].hidden = true;
});
elements["compare-left"].addEventListener("change", renderComparison);
elements["compare-right"].addEventListener("change", renderComparison);
elements["entity-execute"].addEventListener("click", () => {
  const key = elements["entity-execute"].dataset.presentationKey;
  if (!algorithmUi[key]) return;
  selectAlgorithm(key);
  populateDatasets();
  applyProjection();
  runObservation();
  setView("execute");
  document.getElementById("execute-view").scrollIntoView({ block: "start" });
});
elements["execution-evidence-link"].addEventListener("click", () => {
  selectedEntityId = algorithmUi[activeAlgorithm].id;
  elements["catalog-search"].value = "";
  elements["catalog-kind"].value = "all";
  renderCatalog();
  setView("catalog");
});

const query = new URLSearchParams(window.location.search);
renderPlaybackSpeeds();
syncSeedMode(true);
const requestedAlgorithm = query.get("algorithm");
const requestedEntity = query.get("entity");
const requestedComparison = query.get("compare");

try {
  const [projectionResponse] = await Promise.all([
    fetch("./data/atlas.json"),
    init("./pkg/atlas_web_bg.wasm"),
  ]);
  if (!projectionResponse.ok) throw new Error(`cannot load registry projection (${projectionResponse.status})`);
  projection = await projectionResponse.json();
  hydrateGeneratedAlgorithms();
  if (requestedAlgorithm && algorithmUi[requestedAlgorithm]) selectAlgorithm(requestedAlgorithm);
  if (requestedEntity && findRecord(projection, requestedEntity)) selectedEntityId = requestedEntity;
  populateDatasets();
  applyProjection();
  if (requestedEntity) {
    setView("catalog");
    if (requestedComparison) openComparison(requestedComparison);
  }
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
