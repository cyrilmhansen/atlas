import init, { observe_is_sorted_i32 } from "./pkg/atlas_web.js";

const datasets = {
  sorted: [1, 2, 3, 5, 8, 13],
  duplicates: [-2, 0, 0, 4, 4, 9],
  inversion: [1, 2, 5, 4, 6],
  descending: [8, 6, 4, 2, 0, -2],
};

const elements = Object.fromEntries(
  [
    "entity-count", "registry-digest", "source-commit", "algorithm-name",
    "time-complexity", "time-provenance", "space-complexity", "space-provenance",
    "dataset-select", "sequence-input", "input-count", "run-button", "runtime-status",
    "sorted-result", "comparison-count", "inversion-index", "local-time",
    "runtime-context", "sequence-visual", "catalog-search", "catalog-body",
  ].map((id) => [id, document.getElementById(id)]),
);

let projection;
let wasmReady = false;

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

function renderSequence(values, firstInversion = null) {
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
    if (firstInversion !== null && (index === firstInversion || index === firstInversion - 1)) {
      column.classList.add("is-inversion");
    }
    const bar = document.createElement("div");
    bar.className = "value-bar";
    bar.style.height = `${Math.max(8, Math.round((Math.abs(value) / scale) * 130))}px`;
    bar.title = `Index ${index}: ${value}`;
    const label = document.createElement("span");
    label.textContent = String(value);
    column.append(bar, label);
    visual.append(column);
  });
}

function measureLocalCall(values, expectedSorted) {
  const input = new Int32Array(values);
  let iterations = 1;
  let executedIterations = 0;
  let elapsedMilliseconds = 0;
  let checksum = 0;

  while (true) {
    executedIterations = iterations;
    checksum = 0;
    const start = performance.now();
    for (let index = 0; index < iterations; index += 1) {
      const observation = observe_is_sorted_i32(input);
      checksum ^= observation.sorted ? 1 : 0;
      observation.free();
    }
    elapsedMilliseconds = performance.now() - start;
    if (elapsedMilliseconds >= 12 || iterations >= 131072) break;
    iterations *= 2;
  }
  if (Boolean(checksum & 1) !== (expectedSorted && executedIterations % 2 === 1)) {
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

function runObservation() {
  try {
    const values = parseSequence();
    elements["input-count"].textContent = `${values.length} value${values.length === 1 ? "" : "s"}`;
    if (!wasmReady) throw new Error("WebAssembly runtime is not ready");

    const observation = observe_is_sorted_i32(new Int32Array(values));
    const firstInversion = observation.first_inversion;
    const timing = measureLocalCall(values, observation.sorted);

    elements["sorted-result"].textContent = observation.sorted ? "Sorted" : "Not sorted";
    elements["sorted-result"].className = observation.sorted ? "is-true" : "is-false";
    elements["comparison-count"].textContent = String(observation.comparisons);
    elements["inversion-index"].textContent = firstInversion ?? "None";
    elements["local-time"].textContent = timing.microsecondsPerCall === null
      ? "Below timer resolution"
      : `${timing.microsecondsPerCall.toFixed(2)} us/call`;
    elements["runtime-context"].textContent = `${timing.iterations} repeated JS/WASM calls in ${timing.elapsedMilliseconds.toFixed(1)} ms; ${navigator.userAgent}. Not algorithm-only or portable benchmark evidence.`;
    renderSequence(values, firstInversion);
    observation.free();
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

  const algorithm = projection.algorithms.find((item) => item.id === "order.is_sorted.adjacent");
  if (!algorithm) throw new Error("derived projection is missing order.is_sorted.adjacent");
  elements["algorithm-name"].textContent = algorithm.name.value;
  elements["time-complexity"].textContent = algorithm.time_worst.value;
  elements["time-provenance"].textContent = `${algorithm.time_worst.level}: ${algorithm.time_worst.source}`;
  elements["space-complexity"].textContent = algorithm.auxiliary_memory.value;
  elements["space-provenance"].textContent = `${algorithm.auxiliary_memory.level}: ${algorithm.auxiliary_memory.source}`;
  renderCatalog();
}

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
  const values = datasets[elements["dataset-select"].value];
  elements["sequence-input"].value = values.join(", ");
  runObservation();
});
elements["sequence-input"].addEventListener("input", () => {
  elements["dataset-select"].value = "";
  try {
    const count = parseSequence().length;
    elements["input-count"].textContent = `${count} value${count === 1 ? "" : "s"}`;
  } catch {
    elements["input-count"].textContent = "Invalid value";
  }
});
elements["run-button"].addEventListener("click", runObservation);
elements["catalog-search"].addEventListener("input", renderCatalog);

try {
  const [projectionResponse] = await Promise.all([
    fetch("./data/atlas.json"),
    init("./pkg/atlas_web_bg.wasm"),
  ]);
  if (!projectionResponse.ok) throw new Error(`cannot load registry projection (${projectionResponse.status})`);
  projection = await projectionResponse.json();
  applyProjection();
  wasmReady = true;
  setRuntimeStatus("WASM ready", "ready");
  runObservation();
} catch (error) {
  setRuntimeStatus("Runtime unavailable", "error");
  elements["runtime-context"].textContent = error instanceof Error ? error.message : String(error);
  renderSequence(datasets.inversion);
}
