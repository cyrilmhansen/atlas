# K-M5 independent overlay authoring packet

Status: completed; frozen inputs preserved, result in `k-m5-independent-authoring-result.md`

Frozen at Atlas commit: `459b4f089d75af6ebef15d20ea610f72f321829b`

Protocol: `k-m0.3`

## Question

Can an author who has not seen the reference overlay represent a new bounded
top-k candidate with the private vocabulary and obtain the same three
operational decisions as an adjudicator?

This is an authoring-convergence experiment, not a corpus import, public schema
proposal or test of YAML formatting similarity.

## Isolation

The orchestrator supplies only the files listed under **Mandatory packet** in a
fresh directory without Git history. In particular, the author must not receive
or inspect:

- `docs/phase2/k-m5-overlay.yaml`;
- `docs/phase2/k-m5-overlay-result.md`;
- `crates/atlas/src/decision_evaluator.rs` or its tests;
- another author's submission or adjudication notes.

The author may ask for clarification. Every answer and its elapsed intervention
time is appended to the work record. No source search or Web research is needed
for this experiment.

## Mandatory packet

| File | SHA-256 |
|---|---|
| `docs/phase2/k-m5-decision-overlay.md` | `6012f7b0884424d07636bce7ed6cbedaf449d435a40a61de225543f3014a9b9a` |
| `docs/phase2/import-protocol-k-m0.3.md` | `77a140cb7c25f202f4ed23eceee925ba4ae95a7d6f47be5390ab86b8ac22a8bf` |
| `docs/phase2/k-m3-streaming-approximation.md` | `c8fd14a4db89c54152169abbf7dfd0984562f9955a7d050746a910bbce3617d4` |
| `registry/atlas.yaml` | `b4ef8ebebf7b6ccdd6e0986477726f744f9ece1c618a65e2fe9ddc1752a70f8c` |
| `crates/atlas/tests/external_streaming_adapters.rs` | `a90d28c91f007bcd2bd3893264999ba83945394e8683ddaa5f75cdfac24d0f43` |
| `docs/schema-0.1.md` | `fd4889aecd119038ad96d192bb3d343185a6dac5c29de061747dca4e0c619e15` |

Only the bounded top-k material is in scope. Facts about other subjects present
in aggregate files must not enter the submission.

## Subject

Represent the existing implementation
`stream.top_k.rust.std_binary_heap.v1` as one overlay candidate. Preserve these
source distinctions:

- the semantic result is the greatest `min(k, n)` input occurrences, including
  multiplicity;
- capacity zero yields an empty result;
- descending output order is an adapter presentation choice, not the universal
  top-k contract;
- the algorithm retains `O(k)` elements;
- the implementation allocates retained heap and output storage;
- exactness, inferred cost and tested implementation effects may have different
  evidence levels.

Do not add a second candidate, a public entity or a claim not supported by the
mandatory packet.

## Frozen operational requests

The YAML must contain three requests with author-chosen IDs. The work record
maps each ID to exactly one semantic request below.

| Request | Supplied condition and required result | Expected decision |
|---|---|---|
| `zero-capacity` | capacity is zero; require the exact empty-sequence projection | accept the candidate |
| `bounded-retained-memory` | require exact top-k membership and worst retained memory matching the source's `O(k)` claim | accept the candidate |
| `allocation-free` | require exact top-k membership and forbid allocation | reject the candidate because the implementation allocates |

The zero-capacity result is a conditioned output projection, not an algorithm
precondition: the implementation also accepts nonzero capacities. Opaque cost
bounds may preserve the exact source wording. The evaluator performs no
asymptotic algebra.

## Submission

Return exactly two UTF-8 text files:

1. `submission.yaml`, a standalone `phase2-km5-0` overlay;
2. `work-record.md`, containing:
   - author identifier;
   - externally observed start and end time supplied by the orchestrator;
   - interventions and tools used;
   - request-ID mapping;
   - source locator and evidence rationale for every fact;
   - ambiguities and rejected alternative encodings;
   - concepts the closed vocabulary could not represent.

The standalone overlay is bounded to 8 atoms, 1 candidate, 1 relation and 3
requests. It must add no dependency, executable code or schema field.

## Adjudication

The orchestrator, not the author:

1. records the end time before inspecting the files;
2. verifies packet digests and parses `submission.yaml` with the existing K-M5
   validator;
3. resolves the candidate source against the frozen registry;
4. evaluates all three requests with the unchanged generic evaluator;
5. normalizes IDs by the work-record mapping, not by textual similarity;
6. compares identity, semantics, taxonomy and operational decisions separately;
7. records reused concepts, genuinely new concepts, authoring lines and all
   decision-changing differences.

Textual YAML equality is neither required nor scored. A parse failure, missing
source mapping or unexpected operational result is retained as evidence; the
orchestrator does not repair the submission before comparison.

## Exit rule

This single submission may close the independent-authoring gate only if all
three operational results agree and remaining differences do not change
selection, substitution or composition. It cannot by itself authorize schema
0.2. Divergence triggers analysis of protocol, vocabulary and evaluator causes
before any second attempt.
