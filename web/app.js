import init, {
  observe_insertion_sort_i32,
  observe_is_sorted_i32,
  observe_reverse_i32,
} from "./pkg/atlas_web.js";

const algorithmUi = {
  is_sorted: {
    id: "order.is_sorted.adjacent",
    dataset: "sort.regression.duplicates",
    boundary: "Read-only input; no output transport copy.",
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
    boundary: "Algorithm is in-place; the Web observation copies tagged output for display.",
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
    boundary: "Algorithm is in-place; the Web observation copies output for display.",
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
    "entity-count", "registry-digest", "source-commit", "algorithm-id", "algorithm-name",
    "execution-boundary", "result-label", "comparison-label", "secondary-label",
    "time-complexity", "time-provenance", "space-complexity", "space-provenance",
    "dataset-select", "dataset-context", "sequence-input", "input-count", "run-button", "runtime-status",
    "sorted-result", "comparison-count", "inversion-index", "local-time",
    "runtime-context", "sequence-heading", "sequence-visual", "legend-text",
    "catalog-search", "catalog-body",
  ].map((id) => [id, document.getElementById(id)]),
);

let projection;
let wasmReady = false;
let activeAlgorithm = "is_sorted";

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
}

function populateDatasets() {
  elements["dataset-select"].replaceChildren(...projection.datasets.map((dataset) => {
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

function renderSequence(values, highlight = {}) {
  const visual = elements["sequence-visual"];
  visual.replaceChildren();
  if (values.length === 0) {
    const empty = document.createElement("span");
    empty.className = "empty-state";
    empty.textContent = "Empty sequence";
    visual.append(empty);
    return;
  }
  const scale = Math.max(...values.map((value) => Math.abs(value)), 1);
  values.forEach((value, index) => {
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
    setRuntimeStatus("WASM ready", "ready");
  } catch (error) {
    elements["sorted-result"].textContent = "Invalid input";
    elements["sorted-result"].className = "is-false";
    elements["comparison-count"].textContent = "-";
    elements["inversion-index"].textContent = "-";
    elements["local-time"].textContent = "-";
    elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
    setRuntimeStatus("Input rejected", "error");
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
  try {
    const count = parseSequence().length;
    elements["input-count"].textContent = `${count} value${count === 1 ? "" : "s"}`;
  } catch {
    elements["input-count"].textContent = "Invalid value";
  }
});
elements["run-button"].addEventListener("click", runObservation);
elements["catalog-search"].addEventListener("input", renderCatalog);

const requestedAlgorithm = new URLSearchParams(window.location.search).get("algorithm");
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
  runObservation();
} catch (error) {
  setRuntimeStatus("Runtime unavailable", "error");
  elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
  renderSequence([]);
}
