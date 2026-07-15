# K-M6 blind agent-consumer result

Status: complete; verdict **supported**

Date: 2026-07-15

Authority: `docs/phase2/k-m6-agent-consumer-protocol.md`

Baseline: `bf5bcc973d96556cf1c5a87e1e3f79b3ece30a7d`

## Question

Does Atlas's existing CLI and registry improve an agent's selection, evidence
discipline or explanation for an exact bounded-top-k task, compared with an
otherwise equivalent agent given only general project documentation?

The task, response template, oracle, packet hashes and source-reveal procedure
were frozen before either agent started. The private K-M5 overlay was absent
from both arms.

## Run record

Both independent agents started at `2026-07-15T05:02:55.790649629Z`. Neither
requested clarification or received a human intervention.

| Arm | Initial end | Elapsed | Atlas queries |
|---|---:|---:|---:|
| assisted | `05:05:52.924021148Z` | 177.133 s | 29 |
| control | `05:04:45.201989684Z` | 109.411 s | 0 |

The control agent reported its end time out of band but left the corresponding
template field as `pending orchestrator`. The frozen response is preserved as
produced; the table records the externally received timestamp.

The assisted agent used only `search`, `show`, `explain` and `qualify` through
the controlled launcher. The control used packet-local documentation and
general knowledge. Both initial responses were copied and hashed before either
was read or the source was revealed.

| Frozen artifact | SHA-256 |
|---|---|
| assisted initial response | `2bc2628aae63b6618e24651d257b1f800cea91617304cc65ea1c1f7f40b1f33d` |
| control initial response | `1195cea91d58b7bed8affdea74fb16361c758cfcbc7a905cefff45316e77f66b` |
| reveal source | `a90d28c91f007bcd2bd3893264999ba83945394e8683ddaa5f75cdfac24d0f43` |
| assisted final response | `9c5a32de0a4ecb07f9ec1b62571c93050ff1c2b462d08d2a3a9c13d574a0f14a` |
| control final response | `6d05303bdf018c158cdc4e185210ef23fee7934be6368be4a0388a539c58712e` |

## Initial comparison

| Dimension | Assisted | Control |
|---|---|---|
| identity | exact Problem, Algorithm and Implementation IDs | all three deliberately unresolved |
| contract correctness | exactness, multiplicity, zero capacity and implementation fit supported | requirements correctly retained, but no component claims available |
| evidence discipline | preserved `tested`, `declared` and `inferred` levels | clearly separated task facts, general knowledge and missing evidence |
| alternatives | three catalog-backed rejections | five decision-relevant strategy rejections from task/general knowledge |
| interface insight | directly demonstrated qualification gaps and absent composition | correctly inferred the same bounded interface limits from documentation |
| unsupported claims | none material | none material |
| human intervention | none | none |

The assisted arm selected the frozen oracle chain:

- Problem `stream.top_k`;
- Algorithm `stream.top_k.min_heap`;
- Implementation `stream.top_k.rust.std_binary_heap.v1`.

The control independently chose a size-`k` minimum heap as the provisional
strategy, but correctly refused to invent Atlas identities, implementation
properties or registry evidence. This is a useful control outcome rather than
a semantic failure.

Atlas therefore changed the actionable result before source reveal: it turned
a sound generic strategy into a fully identified component chain with explicit
claim levels, sources, effects and rejected catalog alternatives.

## Identical source reveal

After both initial responses were frozen, each arm received the same source.
The orchestrator ran:

```text
cargo test -p atlas --test external_streaming_adapters \
  bounded_top_k_is_exact_and_never_exceeds_its_budget --locked
```

Result: `1 passed; 0 failed; 0 ignored; 0 measured; 4 filtered out`.

The assisted arm retained the exact chain. The control retained the min-heap
strategy but still left registry identities unresolved because source alone
does not establish them. Both distinguished fixture evidence from general proof
and identified remaining test, invariant and complexity gaps.

The post-reveal updates took 54.924 s for the assisted arm and 57.367 s for the
control. Neither revised its frozen initial response.

## Verdict

K-M6 is **supported** under its frozen rubric.

Atlas materially improved identity and evidence-grounded explanation without
increasing unsupported claims or human intervention. The control remained
semantically competent, showing that Atlas's contribution in this experiment
is qualified knowledge and exact component identity rather than invention of
the min-heap strategy itself.

The benefit has an interaction cost: the assisted arm took 67.722 s longer and
issued 29 CLI queries. Many searches were exploratory because the CLI cannot
qualify exactness, multiplicity, streaming retention, determinism or bounded
allocation in one request. This is a measured interface limitation, not a
request to build a new agent API during Phase 2.

## Limits

- One task and two agents cannot establish general agent effectiveness.
- The task came from a family already present in the registry.
- Source isolation was procedural because query commands validate provenance
  against a full workspace; any direct source read would have invalidated the
  experiment. No such read was reported.
- The assisted and control prompts necessarily differed by the treatment, even
  though model settings, common packet and start window were matched.
- Elapsed time includes agent reasoning and tool overhead; it is not a CLI
  benchmark.
- The passing integration test is not proof of all inputs or asymptotic bounds.

## Consequence

K-M6 closes without changing schema 0.1, the CLI, the private overlay or the
runtime. Its positive result and interface-cost observation feed K-M7 Phase 2
synthesis. K-M7 must compare this result with corpus diversity, import
agreement, ontology friction and K-M5's mixed generic-decision result before
recommending any public model change.
