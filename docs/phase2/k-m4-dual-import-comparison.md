# K-M4 independent dual-import comparison

Status: complete

Protocol: `k-m0.2`

Frozen packet: `docs/phase2/k-m4-source-packet.md`

Raw submissions: `docs/phase2/imports/k-m4/importer-a/` and
`docs/phase2/imports/k-m4/importer-b/`

## Experiment record

Two importers received the same six-subject packet and wrote twelve worksheets
without reading the registry, earlier corpus reports, Git history or each
other's output. Both reported no human intervention. Importer A recorded
`01:12:41` to `01:22:51` (10 minutes 10 seconds); importer B recorded
`01:12:49` to `01:24:31` (11 minutes 42 seconds), Europe/Paris on 2026-07-15.
The orchestrator recorded the concurrent launch at `01:12:53` and completed the
comparison at `01:27:04`. Because the launch observation followed both local
start timestamps, importer-reported durations are retained and no synthetic
externally timed duration is claimed.

The comparison uses the frozen five-dimension rubric. It does not score textual
similarity or compute an aggregate agreement percentage. `Divergent` means that
the two normalizations can change a knowledge decision; `unresolved` means the
packet evidence cannot support adjudication.

## Agreement matrix

| Subject | Identity | Semantic | Taxonomic | Operational | Documentary |
|---|---|---|---|---|---|
| Binary-heap push | divergent | equivalent | compatible | equivalent | compatible |
| hashbrown insert | compatible | divergent | equivalent | divergent | compatible |
| Welford corrected moments | compatible | unresolved | unresolved | equivalent | compatible |
| Vitter Algorithm R | compatible | divergent | equivalent | divergent | compatible |
| Bloom method 2 | compatible | equivalent | compatible | equivalent | compatible |
| Misra-Gries | compatible | equivalent | compatible | equivalent | compatible |

The labels above compare decisions, not prose. Both imports consistently
separated abstract behavior from an executable implementation and refused to
request a public schema change during the experiment.

## Adjudication

### Binary-heap push

Both imports preserve mutation, ordering requirements, possible resize and the
three distinct cost regimes. They make the same three operational decisions.
Importer A proposes the representation-independent problem
`priority_queue.push`; importer B proposes `priority_queue.push_max_heap` and
names a conventional sift strategy that the API source does not describe,
although it correctly marks that name inferred.

- Decision affected: lookup and substitution across priority-queue
  representations or min/max comparator configurations.
- Cause: protocol ambiguity about the boundary between a generic operation and
  its representation/orientation.
- Minimal discriminant: import a push operation for a non-binary priority queue
  or for a reversed comparator. A's problem can remain shared; B's cannot
  without an alias or a broader parent identity.
- Adjudication: retain a generic problem identity and a representation-specific
  algorithm identity; do not infer undocumented internal steps as source facts.

### hashbrown insert

Both imports agree on insert-or-replace semantics, retained resident-key
identity, `Eq`/`Hash` obligations, collision sensitivity, possible allocation
and the default hash builder's unsuitable adversarial boundary. They disagree
on evidence attachment. Importer A conditionally attaches the supplied standard
collection convention of expected amortized O(1); importer B keeps it as
context because the frozen hashbrown pages state no hashbrown-specific bound.

- Decision affected: performance-qualified selection under trusted keys.
- Cause: protocol ambiguity, reinforced by source ambiguity. The packet called
  the standard page a cost convention but did not define whether it was evidence
  for the exact third-party implementation.
- Minimal discriminant: request a source-declared expected O(1) guarantee for
  `hashbrown` 0.17.1 itself. A accepts conditionally; B reports it unverified.
- Adjudication: B's narrower evidence attachment is retained. A convention may
  explain a cost vocabulary without proving a versioned implementation claim.

### Welford corrected moments

Both importers reached DOI metadata but no readable article body. Both refused
to reconstruct the recurrence, arithmetic domain, cost or stability properties
from memory, created only provisional documentary identities and rejected all
algorithmic selection requests pending source access.

- Decision affected: none yet; all selection remains rejected.
- Cause: source availability caused by a packet defect, not schema evidence.
- Minimal discriminant: provide a lawful frozen readable copy or transcription
  of the algorithm-bearing pages and repeat this subject independently.
- Adjudication: algorithmic and taxonomic equivalence remain unresolved. This
  subject cannot justify a model or schema conclusion.

### Vitter Algorithm R

Both imports agree on unknown input length, one pass, uniform sampling without
replacement, O(N) processing, O(n) reservoir state and required randomness.
Importer A requires `1 <= n <= N`; importer B proposes `0 <= n <= N` while
explicitly recording that the paper does not specify `n = 0`.

- Decision affected: selection for an empty requested sample.
- Cause: source ambiguity at an unstated boundary, exposed by importer inference.
- Minimal discriminant: request `n = 0` for an empty or nonempty stream.
- Adjudication: keep `n = 0` unresolved rather than extending the source
  contract. A later implementation may support it as a declared adaptation.

### Bloom method 2

Both imports preserve the monotone N-bit state, d distinct repeatable addresses,
definitive negative result, possible false positive, no deletion and
model-conditioned error analysis. Their modern identities and result words
differ, but their three selection outcomes agree.

- Decision affected: none in the written discriminants.
- Cause of residual wording differences: documentary normalization and protocol
  latitude.
- Adjudication: compatible. Preserve the source's one-sided guarantee and make
  any modern `possibly-member` vocabulary an explicit transformation.

### Misra-Gries

Both imports distinguish the bounded one-pass candidate summary from the exact
two-pass result, including strict `> n/k`, O(k) state, replayability and the
paper's O(n log k) AVL realization. Importer B proposes both problem identities
directly; importer A selects the exact identity and records the candidate
identity as an alternative because schema 0.1 lacks a stage relation. The same
three requests receive the same outcomes.

- Decision affected: none after retaining the stage boundary.
- Cause of residual structure difference: protocol ambiguity about whether one
  source subject may propose two related problems.
- Adjudication: compatible. Exact output and candidate-superset output must not
  be substitutable even while their relationship remains experimental.

## Cause classification

No importer error was found. The decision-changing differences have three
causes:

- protocol ambiguity: generic problem boundaries and the evidentiary role of a
  contextual cost source;
- source ambiguity: Algorithm R's zero-size boundary and several unstated
  probability/cost premises;
- model insufficiency: recurring inability to qualify persistent state,
  conditioned/amortized costs, randomness distributions, one-sided error and
  staged exactness as selectable properties.

Model insufficiency did not itself create disagreement on Bloom or Misra-Gries,
but both importers independently identified the same lossy mappings already
seen in different Phase 2 families. This satisfies the evidence threshold for
an ontology review, not for silently choosing public fields.

## Gate result

K-M4 is **complete with a mixed, informative result**:

- five source-readable subjects are semantically useful and three converge at
  the operational level;
- three substantive divergences have minimal discriminating cases and human
  adjudication;
- one source subject remains deliberately unresolved;
- no public schema, AST, registry or runtime change was made to improve apparent
  agreement.

The phase rule says to pause corpus growth when remaining differences change a
selection or when lossy mappings recur across structural families. Both
conditions now hold. K-M5 should therefore not start immediately. The next
milestone should be a bounded ontology review that:

1. repairs the import protocol's identity-boundary and contextual-evidence
   rules;
2. groups recurring model pressures by the decisions they must support;
3. presents at most three schema/ontology alternatives as a class C decision;
4. leaves K-M5 manifest-driven discovery intact as the next falsification test
   after that decision.

The review must not add a general importer, numeric agreement score or more
instrumentation.
