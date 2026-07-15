# Phase 3 - Explorer product trial

Status: active under DEC-071

Active program: Atlas Explorer

Inputs: K-M7 mixed Phase 2 audit, the closed MVP 5/MVP 6 artifact and current
31/36/40 registry projection

## Phase question

Can Atlas turn its existing qualified knowledge and bounded local executions
into a useful, inspectable static product for technically literate visitors,
without weakening provenance or pretending that presentation supplies missing
selection semantics?

The phase tests product usefulness, not a new ontology or runtime.

## Program statuses

| Program | Status | Allowed work |
|---|---|---|
| Atlas Explorer | `active` | catalog workflows, relation navigation, factual comparison, existing interactive execution, responsive presentation |
| Atlas Knowledge | `maintained` | correctness, provenance, schema 0.1 compatibility and blocking projection fixes only |
| Atlas Execution Lab | `frozen` | CI/correction maintenance and use of the five accepted visual programs only |

No repository or crate split follows from these ownership boundaries.

## Baseline

`scripts/check-web.sh` currently demonstrates:

- a deterministic static bundle with no application server or runtime network;
- all 31 problems, 36 algorithms and 40 implementations in the private
  projection;
- one flat searchable catalog table;
- five generated sequence programs through one private WASM visual machine;
- editable/generated inputs, step/run/reset, exact operation counters, sourced
  complexity and qualified local timing;
- native/WASM/differential gates and multi-width browser evidence retained from
  MVP 5 and MVP 6.

The primary baseline gap is not entity coverage. Visitors cannot naturally
move through Problem -> Algorithm -> Implementation chains, inspect all relevant
claims in context or compare two candidates without mentally reconstructing the
registry from table rows.

## Product boundary

The primary audience is a technically literate visitor who wants to answer:

1. What problem does this algorithm solve?
2. Which implementations realize it, under which target and effects?
3. Which claims are declared, inferred, tested, observed or proven, and where
   do they come from?
4. How do two entities differ factually without Atlas inventing a ranking?
5. Is a bounded local visual execution available, and what does it actually
   demonstrate?

The product must also say when Atlas cannot qualify, compare semantically or
execute an entity. Absence is a visible state, not an invitation to fabricate a
generic animation.

## Milestones

### E-M0 - Baseline and information architecture

Status: complete with this activation.

- preserve the 31/36/40 bundle and current five executions;
- define entity discovery, chain navigation, evidence inspection, factual
  comparison and execution handoff as the core workflows;
- retain static/local distribution and private derived formats;
- freeze runtime growth until a workflow demonstrates need.

### E-M1 - Relational catalog and factual comparison

Status: complete. Review: `docs/phase3/e-m1-review.md`.

Smallest vertical slice:

- replace the flat-table-only workflow with selectable entity detail while
  retaining fast text search;
- navigate exact `solves` and `implements` relations in both directions;
- show existing claims with their value, evidence level and provenance;
- compare two same-kind entities side by side, displaying absent claims as
  absent and never producing a score or inferred winner;
- expose whether an algorithm has an accepted interactive presentation and
  hand off to the existing execution view when it does;
- use the full viewport for catalog/detail/compare on narrow and wide screens.

Probable implementation surface:

- `crates/atlas/src/web_projection.rs` for private projection of already stored
  claims and reverse relation data;
- `web/index.html`, `web/app.js` and `web/styles.css` for the workflow;
- `web/tests/projection.cjs` plus focused DOM/browser tests;
- existing build and Chrome viewport gates.

Acceptance:

- every projected entity is reachable by search and exact relation navigation;
- Problem -> Algorithm -> Implementation and reverse links preserve registry
  identity exactly;
- comparison renders only sourced registry claims and makes missing facts
  explicit;
- all five current executable algorithms retain their correction and visual
  behavior;
- 390, 768, 1440 and 1920 pixel views have no incoherent overlap or page-wide
  overflow;
- `scripts/check-web.sh` remains deterministic from a clean archive.

### E-M2 - Evidence-centered execution handoff

Status: next active milestone.

Connect entity detail, sourced complexity, DatasetSpec context and the existing
visual workbench so visitors can distinguish what is known, what is executed and
what is merely observed. Begin with a usability protocol over one executable and
one non-executable algorithm, then make only the smallest handoff changes that
the observed task failures justify. Do not add another executable algorithm
solely to fill an empty state.

### E-M3 - Distribution decision

Status: complete under DEC-072 (`pages-A`).

The reproducible bundle is published by a dedicated GitHub Pages workflow at
`https://cyrilmhansen.github.io/atlas/`. The repository URL is the supported
Phase 3 entry point. Publication does not stabilize the private projection,
WASM facade, generated program or deep-link query parameters.

## Phase exit evidence

Phase 3 may close supported, mixed or falsified when:

- representative visitors can find an entity, traverse its chain and locate a
  claim's provenance without reading YAML;
- side-by-side comparison remains factual and exposes missing information;
- executable and non-executable entities are clearly distinguished;
- current local execution remains correct, bounded and semantically separate
  from theoretical claims and timing observations;
- mobile and desktop workflows are usable and verified;
- the bundle remains reproducible, static and network-independent;
- the audit records whether the Explorer changes comprehension or task
  completion, not merely whether more UI was built.

## Explicit exclusions

- schema 0.2 or promotion of the K-M5 overlay;
- new corpus growth or generic composition work;
- new MIR, RV64, guest-memory or visual-machine capability;
- automatic visualization of algorithms lacking a reviewed executable model;
- a server, account system, remote execution, analytics service or network
  dependency;
- another deployment target, custom domain or publication channel without a
  separate decision;
- stabilizing `atlas-web-private-v0`, visual bytecode or WASM bindings by use.

## Revisit triggers

Return to a Knowledge decision when an Explorer workflow requires a fact that
schema 0.1 cannot preserve rather than merely display. Return to Execution Lab
only when a named user workflow needs semantics absent from the five bounded
programs. In both cases, record the failing workflow before proposing new
infrastructure.
