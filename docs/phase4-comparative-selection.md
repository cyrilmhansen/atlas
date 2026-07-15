# Phase 4 - Comparative foreign selection

Status: active under DEC-074

Active program: Atlas Knowledge

Inputs: Phase 2 mixed selection verdict, K-M5 private evaluator evidence and
the closed-supported Phase 3 Explorer artifact

## Phase question

Can Atlas discover and correctly accept or reject independently sourced
competitors for foreign problems from declared contracts and qualified
evidence, without scenario-specific candidate code or premature schema 0.2?

## Competing hypotheses

- **H1:** the current entity model plus a small recurring qualification model
  can support the same decisions across graph, dynamic and streaming families.
- **H0a:** schema 0.1 already carries enough authoritative information and only
  a generic query is missing.
- **H0b:** the decisions require family-specific concepts or source-specific
  branches, so a shared selection model is not yet justified.

A negative result is informative. Atlas must report `unsupported` when required
facts are absent rather than choosing a plausible candidate.

## Program statuses

| Program | Status | Allowed work |
|---|---|---|
| Atlas Knowledge | `active` | bounded competitor imports, frozen decision requests, candidate discovery and selection evidence |
| Atlas Explorer | `maintained` | public artifact, compatibility and blocking correctness fixes only |
| Atlas Execution Lab | `frozen` | CI/correction maintenance only; no new executable representation |

## Experimental controls

- schema 0.1 and `registry/atlas.yaml` remain authoritative;
- the existing `atlas qualify` surface is measured as-is before extension;
- candidate sets must originate from `solves` and `implements` relations;
- requests and oracle reasons are frozen before competitor manifests are added;
- source wording is not normalized merely to force agreement;
- implementation behavior may be tested, but tests do not establish complexity;
- the K-M5 overlay remains private, frozen evidence and is not registry input;
- no public field is proposed until the same missing concept changes a decision
  in at least two families.

## Milestones

### K4-M0 - Frozen comparison protocol and baseline

Status: complete. Protocol: `docs/phase4/k4-m0-protocol.md`.

- inventory the exact current candidates for the three target problems;
- freeze positive, negative and insufficient-evidence requests per family;
- record expected reasons without naming future candidate identifiers;
- execute schema 0.1/public CLI controls and record unsupported decisions;
- define source-independence and implementation-provenance acceptance.

Exit evidence: the protocol can falsify automatic discovery and distinguishes
missing authoritative facts from missing query machinery.

### K4-M1 - Graph reachability competition

Status: complete. Result: `docs/phase4/k4-m1-result.md`.

Add a depth-first reachability strategy competing with the existing breadth-
first traversal for `graph.reachable_traversal`. Compare traversal-order needs,
frontier memory, output contract and early-target behavior without changing the
problem identity merely to separate the strategies.

Exit evidence: the new algorithm and implementation enter the candidate set
through registry relations; frozen requests accept or reject both with sourced
reasons, or explicitly demonstrate an unsupported qualification.

### K4-M2 - Dynamic priority-queue competition

Status: next after B2 qualification-representation choice and source review.

Add a structurally distinct priority-queue strategy for at least one existing
construct/push/pop problem. Test state compatibility, capacity conditions,
allocation behavior and worst/amortized operation guarantees.

Exit evidence: selection remains generic across persistent state and does not
encode binary-heap or competitor identifiers.

### K4-M3 - Exact bounded top-k competition

Add a second exact bounded-top-k strategy with a materially different
time/memory tradeoff from the existing minimum heap. Preserve capacity-zero,
duplicate and output-order semantics explicitly.

Exit evidence: the same qualification machinery handles a streaming candidate
and exposes absent facts rather than borrowing them from the implementation.

### K4-M4 - Cross-family synthesis

Compare recurring decision facts, authoring cost, evaluator cost and public
control failures. Recommend deletion, continued private experimentation or the
smallest schema 0.2 proposal.

Exit evidence: at least one new candidate is discovered automatically in each
family; positive and negative decisions are explained; no accepted decision
depends on a source-family branch; any public schema proposal is supported by
decision-changing evidence from at least two families.

## Decisions still open

### B1 - Source and implementation for each competitor

Prefer reviewed, permissively licensed external implementations or stable
primary algorithm sources. Decide per corpus batch after source review; adding
a library dependency requires its own dependency-cost check.

K4-M1 review: `docs/phase4/k4-m1-dfs-source-review.md`. Recommendation:
`dfs-A`, the existing pinned petgraph 0.8.3 DFS with its same-upstream evidence
limitation stated explicitly.

### B2 - Phase-local qualification representation

Do not choose before K4-M0. Options are: reuse an unchanged K-M5 evaluator with
a separate phase-local input, implement a smaller disposable comparison matrix,
or stop at the schema 0.1 negative control. Recommendation will follow measured
request needs and line cost.

K4-M1 now supplies that evidence. Options and recommendation are recorded in
`docs/phase4/k4-b2-options.md`; recommendation: `b2-A`, reuse the unchanged
K-M5 evaluator with separate Phase 4 input and no public surface.

### C1 - Public schema or CLI promotion

Any schema 0.2 field, persistent qualification format or stable generic query
requires explicit validation after K4-M4. Phase 4 activation alone does not
authorize it.

## Explicit exclusions

- new composition scenarios or a general planner;
- automatic ranking or benchmark-derived selection;
- schema 0.2 implementation during the corpus milestones;
- AST, MIR, RISC-V, WASM or visual-machine expansion;
- Explorer feature work unrelated to a blocking compatibility defect;
- importing many shallow entries instead of the three controlled comparisons.
