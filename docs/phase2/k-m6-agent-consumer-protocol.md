# K-M6 blind agent-consumer protocol

Status: executed; result in `docs/phase2/k-m6-result.md`

Code and registry baseline: `bf5bcc973d96556cf1c5a87e1e3f79b3ece30a7d`

Authority: DEC-066, DEC-070 and `docs/phase2-external-corpus.md`

## Hypothesis

Does access to Atlas's existing CLI and registry improve an agent's component
selection, evidence discipline or explanation for a new task, compared with the
same agent given only general project documentation?

K-M6 tests Atlas as it exists after K-M5. The private overlay, equivalence
resolver and K-M5 result documents are excluded from both arms.

## Frozen task

Both arms receive `k-m6-task.md` and `k-m6-response-template.md`. The task asks
for exact bounded top-k over a finite integer stream. It is drawn from the K-M3
corpus and has no reviewed `compose` scenario.

The orchestrator-only expected selection and comparison rubric are frozen in
`k-m6-oracle.md`. That file must never enter either selection packet.

## Arms

Use two independent agents with identical model/settings, no shared context, no
Git history and no network access.

### Common packet

- the README and common documentation are extracted from the frozen baseline
  commit, not copied from the working tree;
- `TASK.md`, copied from `k-m6-task.md`;
- `RESPONSE.md`, copied from `k-m6-response-template.md`;
- `common/README.md`;
- `common/docs/schema-0.1.md`;
- `common/docs/vision.md`;
- `common/docs/project-vocabulary.md`.

### Assisted-only additions

- executable `atlas` built from the frozen code baseline;
- `registry/atlas.yaml`, needed by the executable at its default relative path.

The existing query commands validate every evidence reference against a full
workspace. The orchestrator therefore also creates an immutable checkout of
the frozen baseline outside the packet and exposes a launcher which runs the
packet binary from that checkout. The assisted agent may invoke only `search`,
`show`, `explain` and `qualify` through this launcher. It must not inspect the
checkout or read `registry/atlas.yaml` directly. This tests the existing query
surface rather than raw-YAML or source extraction.

### Control exclusions

The control receives neither executable, registry, corpus-specific report,
query output nor implementation source. It may reason from general knowledge,
but must distinguish that reasoning from packet evidence.

## Frozen digests

| Packet input | SHA-256 |
|---|---|
| `TASK.md` | `98e3ce4aa3d236899a83dc9da4b82b76d537383b178c1e522924831df40def5b` |
| `RESPONSE.md` initial template | `7841b52bd6bb6fb4b110eecc7854ab92c2f080997ec6255161988de8c100c06d` |
| `common/README.md` | `0c0b56b16a6ddd2e055207bf3a3288002891beeba9c18560decf6b65893f5021` |
| `common/docs/schema-0.1.md` | `fd4889aecd119038ad96d192bb3d343185a6dac5c29de061747dca4e0c619e15` |
| `common/docs/vision.md` | `32f0e9886317eb7acf856277ab4bd5b1b712a8e4ee4d2f831decb6bcf3f1af5d` |
| `common/docs/project-vocabulary.md` | `b86e95f2fd3776f0a0aff7f006fd520a207c0c758ce926f0badca3068d81132a` |
| assisted `atlas` | `75b02e95bda35e477c6ea37a6bb0180c2b013c6848d9f0ee3644086192d32a30` |
| assisted `registry/atlas.yaml` | `b4ef8ebebf7b6ccdd6e0986477726f744f9ece1c618a65e2fe9ddc1752a70f8c` |
| post-reveal source | `a90d28c91f007bcd2bd3893264999ba83945394e8683ddaa5f75cdfac24d0f43` |
| orchestrator-only oracle | `fb3ad65d3673d7b50927bb1ab088cc56e66e2e1dfd711409bd9700c43429a976` |

The 1,542,368-byte assisted binary was built with rustc 1.96.0 for
`x86_64-unknown-linux-gnu`. It dynamically requires the same-host glibc,
`libgcc_s` and SQLite libraries. It is a temporary experiment tool, not a
distribution artifact.

## Stage 1 - Blind selection

1. Create both packet directories and verify every copied digest.
2. Record both start timestamps externally before launching agents concurrently.
3. Instruct each agent to inspect only its packet directory.
4. Give only the assisted agent the controlled CLI launcher; record its exact
   invocations and outputs.
5. Require completion of `RESPONSE.md` without changing supplied files.
6. Record each end timestamp before reading either response.
7. Preserve every clarification request and externally measured intervention.

Do not repair, normalize or show one response to the other. A missing identity
or explicit uncertainty is valid evidence.

## Stage 2 - Identical source reveal

After both initial responses are frozen:

1. copy the same frozen `external_streaming_adapters.rs` into both packets;
2. run the existing bounded-top-k integration test once as orchestrator;
3. supply the exact command/result to both agents;
4. ask each agent to complete only the post-reveal addendum;
5. record revised end timestamps and any corrected selection.

The implementation test is evidence about the fixture and adapter. It does not
promote general correctness or complexity to `proven`.

## Comparison

Use the frozen oracle dimensions: identity, contract correctness, evidence
discipline, alternatives, interface insight, correction and intervention.
Report each dimension directly; do not combine them into an arbitrary scalar
score.

The result is **supported** only if Atlas materially improves at least one of
correctness, evidence-grounded explanation or required human intervention
without increasing unsupported claims. Equivalent arms produce a **neutral**
result. Better identifiers but worse semantic judgment is **mixed**. A plan
alone is never sufficient.

## Boundaries

- no new CLI command, structured agent API or private-overlay access;
- no implementation source before both initial responses are frozen;
- no network, benchmark, MIR, Web or execution-record work;
- no attempt to generalize from one task to all agents or algorithms;
- no push or publication as part of K-M6.

The source boundary is procedural rather than enforced by filesystem
permissions because the current CLI validates evidence against its workspace.
Any direct read of the external checkout invalidates the run. This limitation
is part of the K-M6 interface finding and must be reported, not hidden.
