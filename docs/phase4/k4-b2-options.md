# B2 - Phase-local qualification representation

Status: Option A accepted under `b2-A`; transfer result mixed

Date: 2026-07-15

## Context

K4-M1 discovers BFS and DFS generically, but its frozen adjudication remains a
reviewed Markdown matrix. Schema 0.1 cannot represent graph-shape-conditioned
frontier costs or an explicit hop-order guarantee, and the public CLI cannot
query several facts that are present. Phase 4 needs an executable control
without turning the private K-M5 model into registry authority.

## Option A - Reuse the unchanged K-M5 evaluator

Create a separate Phase 4 input using the existing private
`phase2-km5-0` parser and evaluator. Add no evaluator feature. Encode only the
two discovered graph candidates and four frozen requests using existing
capability, guarantee, effect, condition and conditioned-cost concepts.

Cost and risks:

- one small phase-local YAML document and focused tests;
- reuses the already measured 1,169-line experimental implementation;
- risks prolonging a private vocabulary that was deliberately frozen;
- outcome classification must remain an experiment and must not be exposed as
  a public CLI or schema contract.

Reversibility: high. Delete the Phase 4 input and tests; schema 0.1 and K-M5
evidence remain unchanged.

## Option B - Build a smaller disposable matrix evaluator

Implement only the four K4-M0 outcome classes over a new Phase 4 structure.

Cost and risks:

- likely less vocabulary per request but new parser, validation and decision
  code;
- duplicates evidence filtering, effects and conditioned costs already present
  in K-M5;
- risks optimizing for the graph matrix before dynamic and streaming cases.

Reversibility: high in theory, but it adds a second experimental engine that
must be compared and removed later.

## Option C - Keep adjudication documentary through K4-M2

Import the dynamic competitor and review both matrices manually before choosing
any executable representation.

Cost and risks:

- zero immediate code or format growth;
- strongest protection against premature abstraction;
- postpones the automatic-decision falsifier and may let inconsistent meanings
  survive until the second batch.

Reversibility: complete. The frozen protocols remain useful.

## Recommendation

Recommend **Option A (`b2-A`)**, narrowly.

K-M5 already represents every concept required by the graph requests. Reusing
it unchanged tests whether its vocabulary transfers to a genuinely competing
foreign problem; that is higher-information than writing another evaluator.
The experiment must use a separate Phase 4 file, add no production CLI and stop
immediately if any graph request requires evaluator code or a family-specific
branch.

## Minimum experiment

1. Encode two candidates and four requests in a separate bounded overlay.
2. Validate all sources against the authoritative registry.
3. Require exact accepted/rejected sets and decision-relevant reasons.
4. Measure added YAML and test lines separately from the sunk K-M5 code.
5. If evaluator code changes, stop and return to B2 rather than expanding it.

Validation question: `b2-A`, `b2-B`, or `b2-C`?

Owner decision: `b2-A`. Result and follow-up options:
`docs/phase4/k4-b2-result.md`.
