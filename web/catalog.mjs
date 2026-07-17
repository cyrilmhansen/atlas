const CLAIMS = {
  condition: [
    ["statement", "Statement"],
  ],
  problem: [
    ["input", "Input"],
    ["requires", "Requirements"],
    ["output", "Output"],
    ["ensures", "Guarantees"],
  ],
  algorithm: [
    ["name", "Name"],
    ["requires", "Requirements"],
    ["stable", "Stable"],
    ["deterministic", "Deterministic"],
    ["in_place", "In place"],
  ],
  implementation: [
    ["language", "Language"],
    ["version", "Version"],
    ["license", "License"],
    ["target", "Target"],
    ["dependencies", "Dependencies"],
    ["abi", "ABI"],
    ["entrypoint", "Entrypoint"],
    ["signature", "Signature"],
    ["effects", "Effects"],
    ["tests", "Test evidence"],
  ],
};

export function catalogRecords(projection) {
  return [
    ...projection.conditions.map((entity) => ({ kind: "condition", entity })),
    ...projection.problems.map((entity) => ({ kind: "problem", entity })),
    ...projection.algorithms.map((entity) => ({ kind: "algorithm", entity })),
    ...projection.implementations.map((entity) => ({ kind: "implementation", entity })),
  ];
}

export function findRecord(projection, id) {
  return catalogRecords(projection).find((record) => record.entity.id === id);
}

export function claimEntries(record) {
  const entries = CLAIMS[record.kind]
    .map(([key, label]) => ({ key, label, claim: record.entity[key] }))
    .filter((entry) => entry.claim !== null && entry.claim !== undefined);
  if (record.kind !== "algorithm") return entries;
  return entries.concat(record.entity.costs.map((claim) => ({
    key: costKey(claim.value),
    label: `${claim.value.metric} · ${claim.value.regime}`,
    claim,
  })));
}

export function displayName(record) {
  if (record.kind === "condition") return record.entity.statement.value;
  if (record.kind === "algorithm") return record.entity.name.value;
  if (record.kind === "implementation") return record.entity.entrypoint.value;
  return record.entity.id.split(".").at(-1).replaceAll("_", " ");
}

export function relatedRecords(projection, record) {
  if (record.kind === "condition") {
    return projection.algorithms
      .filter((algorithm) => algorithm.costs.some((claim) => claim.value.requires.includes(record.entity.id)))
      .map((entity) => ({ kind: "algorithm", relation: "qualifies cost of", entity }));
  }
  if (record.kind === "problem") {
    return projection.algorithms
      .filter((algorithm) => algorithm.solves === record.entity.id)
      .map((entity) => ({ kind: "algorithm", relation: "solved by", entity }));
  }
  if (record.kind === "algorithm") {
    const problem = projection.problems.find((item) => item.id === record.entity.solves);
    const implementations = projection.implementations
      .filter((implementation) => implementation.implements === record.entity.id)
      .map((entity) => ({ kind: "implementation", relation: "implemented by", entity }));
    return [
      ...(problem ? [{ kind: "problem", relation: "solves", entity: problem }] : []),
      ...implementations,
      ...record.entity.costs.flatMap((claim) => claim.value.requires.map((conditionId) => {
        const entity = projection.conditions.find((condition) => condition.id === conditionId);
        return entity ? { kind: "condition", relation: "cost requires", entity } : null;
      }).filter(Boolean)),
    ];
  }
  const algorithm = projection.algorithms.find((item) => item.id === record.entity.implements);
  const problem = algorithm
    ? projection.problems.find((item) => item.id === algorithm.solves)
    : undefined;
  return [
    ...(algorithm ? [{ kind: "algorithm", relation: "implements", entity: algorithm }] : []),
    ...(problem ? [{ kind: "problem", relation: "ultimately solves", entity: problem }] : []),
  ];
}

export function filterCatalog(projection, term, kind = "all") {
  const normalized = term.trim().toLowerCase();
  return catalogRecords(projection).filter((record) => {
    if (kind !== "all" && record.kind !== kind) return false;
    if (normalized === "") return true;
    const related = relatedRecords(projection, record).map((item) => item.entity.id);
    return `${record.entity.id} ${displayName(record)} ${related.join(" ")} ${JSON.stringify(record.entity)}`
      .toLowerCase()
      .includes(normalized);
  });
}

export function comparableRows(left, right) {
  if (left.kind !== right.kind) throw new Error("comparison requires entities of the same kind");
  const leftClaims = new Map(claimEntries(left).map((entry) => [entry.key, entry]));
  const rightClaims = new Map(claimEntries(right).map((entry) => [entry.key, entry]));
  const base = CLAIMS[left.kind].map(([key, label]) => ({ key, label }));
  const dynamic = [...leftClaims.values(), ...rightClaims.values()]
    .filter((entry) => !base.some((item) => item.key === entry.key))
    .map(({ key, label }) => ({ key, label }));
  const rows = [...base, ...new Map(dynamic.map((entry) => [entry.key, entry])).values()];
  return rows.map(({ key, label }) => ({
    key,
    label,
    left: leftClaims.get(key)?.claim ?? null,
    right: rightClaims.get(key)?.claim ?? null,
  }));
}

function costKey(cost) {
  return `cost:${cost.metric}:${cost.regime}:${cost.bound}:${[...cost.requires].sort().join(",")}`;
}

export function executablePresentation(projection, record) {
  if (record.kind !== "algorithm") return undefined;
  const generated = projection.dynamics
    .find((dynamics) => dynamics.algorithm_id === record.entity.id && dynamics.presentation)
    ?.presentation;
  if (generated) return generated;
  if (record.entity.id === "disjoint_set.rank_path_halving.union") {
    return { key: "union_find" };
  }
  return undefined;
}
