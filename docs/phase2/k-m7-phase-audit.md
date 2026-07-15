# K-M7 Phase 2 synthesis

Status: complete synthesis; Phase 2 closure decision pending

Date: 2026-07-15

Authority: DEC-066 through DEC-070 and
`docs/phase2-external-corpus.md`

## Verdict

Phase 2 is **mixed and informative**.

Atlas preserves source identity, transformations, evidence levels and
decision-relevant uncertainty across three structurally different foreign
families. Independent importers expose real semantic divergences instead of
being forced into textual agreement. Existing registry queries materially help
an agent identify and justify a component before source reveal.

The stronger product hypothesis is not yet supported. Schema 0.1 and the public
CLI cannot perform the qualified selections demonstrated by the private K-M5
overlay, and the composer remains a collection of reviewed scenarios. The new
foreign families contain no competing algorithms for the same imported problem
and no algorithm with multiple external implementations. Phase 2 therefore
tests breadth, fidelity and identity better than substitution or composition.

This is not a reason to extend the runtime, AST, Web artifact or private overlay.
It identifies the smallest missing experiment before a public knowledge-model
decision.

## Corpus accounting

The authoritative registry projection at K-M7 contains:

| Entity | Phase 2 baseline | K-M7 | Delta |
|---|---:|---:|---:|
| Problems | 10 | 31 | +21 |
| Algorithms | 15 | 36 | +21 |
| Implementations | 20 | 40 | +20 |
| Total entities | 45 | 107 | +62 |

The counts are reproducible with:

```text
atlas list problem
atlas list algorithm
atlas list implementation
atlas index /tmp/atlas-km7-phase2.db
```

The derived projection contains 36 `solves` and 40 `implements` relations and
has logical SHA-256
`2c500088cf52278055afd1addf402ce0c27f035e70b52f539a2bd000665d9db4`.
SQLite remains a regenerable measurement surface, not registry authority.

The secondary 30-to-50 sampling target is reached if it means total registered
algorithms: Atlas has 36. Only 21 algorithms were added during Phase 2, so the
target is not presented as 36 foreign imports.

### Structural families

| Family | Problems | Algorithms | Implementations | Principal pressure |
|---|---:|---:|---:|---|
| graphs | 4 | 4 | 2 | output variants, topology, numeric domain, callback effects |
| dynamic structures | 11 | 11 | 12 | persistent state, lifetime, amortization, allocation |
| streaming/approximation | 6 | 6 | 6 | bounded state, randomness, error, numerical stability |

The remaining 10/15/20 entities are the pre-Phase-2 sequence baseline. The
dynamic implementation count includes the hashbrown stable-deduplication
adapter attached to an existing sequence algorithm.

### Source diversity

Phase 2 uses several independent source classes:

- versioned library APIs and executable code: petgraph 0.8.3, Rust standard
  library API baseline and hashbrown 0.17.1;
- an established educational implementation source: algs4 graph contracts,
  retained without copying GPLv3 code;
- research publications: Welford, Vitter, Bloom, Misra-Gries, Pébay and
  Chan-Golub-LeVeque, with inaccessible or unread portions left unresolved.

At least thirteen Phase 2 implementation records execute upstream petgraph,
standard-library or hashbrown operations through thin tests. Paper-derived
streaming adapters are independently written Atlas test fixtures and are not
misrepresented as upstream implementations.

## Competition audit

The registry has exactly five problems solved by more than one algorithm:

- `sequence.sort`;
- `sequence.search`;
- `sequence.filter`;
- `sequence.partition`;
- `sequence.deduplicate`.

It also has exactly five algorithms with more than one implementation:

- `sort.merge.top_down`;
- `filter.copy.stable`;
- `partition.copy.stable`;
- `merge.sorted.two_way`;
- `deduplicate.hash.stable`.

These satisfy the literal Phase 2 exit quotas of five competing problems and
three multi-implementation algorithms. They do not validate competition in the
foreign corpus: all five problems predate Phase 2, and only
`deduplicate.hash.stable` gained an external Phase 2 candidate. Each graph,
dynamic-structure and streaming algorithm still has one registered
implementation at most.

This distinction matters because an ontology can preserve isolated contracts
while still failing to compare independently sourced alternatives.

## Milestone evidence

| Milestone | Result | Information gained |
|---|---|---|
| K-M0 | mixed pilot | identity and operational equivalence are more useful than textual equality; effort self-estimates were unusable |
| K-M1 | complete | exact graph variants remain reviewable, but projections and numeric domains are not queryable |
| K-M2 | complete | persistent operations and external libraries fit as separate entities; an added hashbrown adapter enters an existing candidate set generically |
| K-M3 | complete | exact, random, approximate and numerical contracts remain distinct in prose/evidence, but not as typed qualification predicates |
| K-M4 | mixed | three decision-changing divergences across six dual imports; inaccessible Welford source correctly remained unresolved |
| K-M4-W | strong agreement with two divergences | open neutral sources repair recurrence evidence; empty-state adaptation and `proven` semantics remain distinct questions |
| K-M5 | mixed under DEC-070 | a private generic evaluator makes correct qualified decisions, but costs 1,169 Rust lines and uses non-authoritative facts absent from schema 0.1 and the CLI |
| K-M6 | supported | Atlas gives an agent exact identity and evidence-grounded explanation; 29 queries and about 68 extra seconds expose interface cost |

## Independent-import agreement

K-M4 compared six subjects under identity, semantic, taxonomic, operational and
documentary dimensions.

- Bloom and Misra-Gries converged operationally and semantically.
- Binary-heap push converged operationally but diverged on problem identity.
- hashbrown insert diverged on attaching contextual expected-cost evidence.
- Vitter Algorithm R diverged on the unstated zero-size boundary.
- Welford remained unresolved because the packet exposed metadata but not the
  algorithm-bearing text.

No importer error was found. The differences were attributable to protocol
ambiguity, source ambiguity or model insufficiency. The protocol was tightened
without forcing a schema field.

K-M4-W then used two open reports for online second moments. The importers
strongly agreed on recurrence, state, finalizers and incremental/pairwise
separation. They still diverged on totalizing empty input and on whether a
published proof alone merits `proven`. DEC-068 resolves the latter
conservatively by requiring an auditable claim-to-proof mapping.

## Authoring effort

Comparable effort evidence remains sparse and must not be converted into an
average cost per algorithm.

| Experiment | Unit | Recorded elapsed | Intervention |
|---|---|---:|---|
| K-M4 importer A | six worksheets | 10 min 10 s | none |
| K-M4 importer B | six worksheets | 11 min 42 s | none |
| K-M4-W importer A | one worksheet | 3 min 4 s | none |
| K-M4-W importer B | one worksheet | 3 min 41 s | none |
| K-M5 independent author | one overlay candidate and three requests | 2 min 27 s | one structure-only clarification |
| K-M6 assisted consumer | one selection | 177.133 s | none; 29 CLI queries |
| K-M6 control consumer | one selection | 109.411 s | none |

K-M0 self-estimates were explicitly rejected as incomparable. K-M1 through
K-M3 did not record isolated batch-authoring time. The measurements above mix
reading, reasoning, tool latency and writing; they show protocol/interface cost,
not human productivity or scalable import throughput.

## Model-friction synthesis

The same decision-relevant losses recur across independent families:

| Need | Independent evidence | Current status |
|---|---|---|
| directional contract projection/refinement | Dijkstra output variants; Misra-Gries stages; Bloom negative result | private K-M5 relation only |
| scoped or conditioned claims | heap capacity/resize; hash workload; graph numeric domain; Bloom parameters | prose in schema 0.1; private K-M5 facts |
| typed persistent-state flow | union-find, heap/map, Bloom, online moments | prose only |
| qualified output semantics | exact top-k, sound Misra-Gries superset, one-sided Bloom answer, Dijkstra projection | prose or separate identities |
| explicit variation source | graph iteration order, reservoir seed/generator, Bloom hash model, IEEE-754 order | flattened boolean or prose |
| auditable evidence threshold | source theorem, tested adapter, observed benchmark | protocol rule under DEC-068; no public proof mapping |
| aliases and transformation provenance | generic/source-specific identity differences across K-M0/K-M4 | import protocol and reports, not registry fields |

The ontology review proves that these are not one-family requests. K-M5 proves
that structured versions can change decisions. It does not prove that its exact
overlay vocabulary, equivalence rules or 1,169-line implementation should
become schema 0.2.

### Evidence-level conclusion

- `declared`, `inferred`, `tested` and `observed` remained operationally
  distinct throughout the corpus and K-M6.
- `proven` is now defined conservatively by DEC-068, but no imported registry
  claim exercises a complete auditable proof mapping.
- Benchmarks and interactive executions remain observations, not theoretical
  complexity evidence.

## AST coverage

The private AST remains sequence-oriented. It does not type graph adjacency,
node/edge identity, priority queues, parent forests, maps, persistent object
state, random sources, probabilistic answers, floating error or amortized
operation sequences.

K-M1 records graph coverage as absent; K-M2 records dynamic structures as
absent; K-M3 deliberately imports streaming semantics only into contracts and
test adapters. This is useful falsification evidence: the knowledge model can
accept foreign algorithms without requiring an executable AST.

No AST extension is recommended from Phase 2. A later Execution Lab experiment
must choose one concrete foreign algorithm because instrumentation requires it,
not because the knowledge registry requires universal executable coverage.

## Exit-criteria audit

| Phase 2 criterion | Result | Qualification |
|---|---|---|
| three structurally different families | pass | graphs, dynamic structures, streaming/approximation |
| five problems with competing algorithms | literal pass, weak phase evidence | exactly five, all from the sequence baseline |
| three algorithms with multiple implementations | literal pass, weak phase evidence | exactly five, only stable dedup gained a Phase 2 external candidate |
| two external source types | pass | libraries, educational source and research papers |
| two external implementations with provenance/tests | pass | petgraph and hashbrown, plus standard-library operations |
| two cases requiring experimental annotation | pass | at least seven recurring discriminants enter the private overlay |
| substantive independent-import divergence | pass | heap identity, hash cost evidence, reservoir zero boundary and moments totalization |
| source-fidelity and unresolved-mapping records | pass | K-M0/K-M1 through K-M4-W reports preserve both |
| dual-agent import agreement report | pass | K-M4 plus K-M4-W repair |
| generic candidate discovery without implementation branch | pass, bounded | K-M2 public qualifier and K-M5 private evaluator |
| blind agent-consumer comparison | pass | K-M6 supported with measured query cost |
| evidence-based model-change list | pass | ontology review and this cross-family synthesis |
| AST findings without automatic extension | pass | absent foreign-family coverage retained as a boundary |
| generic public qualified selection | fail | required facts remain outside schema 0.1 and public CLI |
| new manifest-driven composition | fail | composer scenarios remain code-selected |

The first thirteen criteria permit K-M7 synthesis. The last two explain why the
phase verdict is mixed rather than supported.

## Verification snapshot

- `cargo test -p atlas --locked`: 156 tests passed across the library and four
  integration suites;
- `atlas index /tmp/atlas-km7-phase2.db`: 107 entities, 76 relations and 688
  claims with the logical digest recorded above;
- SQL grouping over `solves` and `implements`: five competing problems and five
  multi-implementation algorithms;
- `git diff --check`: clean.

## Phase-question adjudication

### Preserve independent knowledge

**Supported.** Atlas can store exact foreign problem/algorithm/implementation
boundaries, provenance, transformations and evidence without coupling the
knowledge model to MIR, Web or a universal AST.

### Normalize independent sources

**Mixed.** Operational convergence is often strong, but identity, contextual
evidence, partial domains and taxonomy require explicit adjudication. Protocol
rules reduce some variance; recurring semantic concepts require structured
representation experiments.

### Select components generically

**Mixed.** A new external candidate entered an existing public qualifier with
no query branch, and K-M6 showed real agent value. More expressive decisions
work only in the private overlay, not from authoritative manifests.

### Compose components generically

**Unsupported in Phase 2.** No new foreign pipeline was assembled from manifest
contracts without scenario logic. Persistent state and directional output
relations remain untyped in the public model.

## Strategic options

### Option A - Phase 3 comparative foreign selection

Close Phase 2 as mixed and activate a bounded Atlas Knowledge phase that adds
one genuinely competing algorithm to an existing problem in each foreign
family before proposing schema 0.2.

Minimum experiment:

- graph: a second reachability strategy for `graph.reachable_traversal`;
- dynamic structure: a non-binary or otherwise structurally distinct priority
  queue strategy for an existing queue operation;
- streaming: a second exact bounded-top-k strategy with a different
  time/memory tradeoff;
- freeze selection requests before import and require candidate entry without
  source-family query branches;
- draft, but do not implement, the smallest schema 0.2 delta only after the
  three comparisons expose which qualified facts actually change decisions.

Cost: a small corpus batch, three independent source reviews and one selection
matrix. Risk: extending corpus work without immediately improving the public
query. Reversibility: high; schema 0.1 remains authoritative.

### Option B - Direct schema 0.2 proposal

Close Phase 2 and immediately design a public schema containing a minimal
subset of directional contract relations, qualified claims and state flow,
using K-M5 as the executable control.

Cost: migration design, compatibility rules, validator/index/CLI changes and
new authoring guidance. Risk: promoting an ontology tested mainly on isolated
foreign candidates and a private overlay with a high infrastructure ratio.
Reversibility: low after external use.

### Option C - Explorer-first product phase

Accept current Knowledge limits, keep schema 0.1 and focus the next phase on
catalog search, comparison and interactive presentation.

Cost: product work with known semantic qualification gaps. Risk: optimizing
presentation before Atlas can compare alternatives in its foreign corpus.
Reversibility: medium because Knowledge artifacts remain intact.

## Recommendation

Recommend **Option A**.

It addresses the largest validity gap using the smallest reversible experiment:
the new families need competing candidates before Atlas stabilizes the facts
used to compare them. It also prevents K-M5's private vocabulary from becoming
the public schema merely because it is executable.

Phase 2 should remain active only administratively until this closure and next
phase are accepted. Execution Lab stays frozen and Explorer maintained. No code,
schema, dependency, format or public CLI change is authorized by this audit.
