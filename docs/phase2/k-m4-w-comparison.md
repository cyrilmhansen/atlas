# K-M4-W independent online-moments comparison

Status: complete

Protocol: `k-m0.2`

Frozen packet: `docs/phase2/k-m4-w-source-packet.md`

Raw submissions: `docs/phase2/imports/k-m4-w/importer-a/` and
`docs/phase2/imports/k-m4-w/importer-b/`

## Experiment record

Two isolated importers read the same mandatory pages from Pébay 2008 and Chan,
Golub and LeVeque 1979. Both verified the frozen PDF digests, reported no human
intervention and wrote one worksheet without reading the registry, previous
imports, Git history or peer output.

Importer A recorded `01:41:11` to `01:44:15` (3 minutes 4 seconds). Importer B
recorded `01:41:16` to `01:44:57` (3 minutes 41 seconds), Europe/Paris on
2026-07-15. The orchestrator did not expose a reliable external start time, so
these local elapsed durations are retained without presenting them as active
authoring time.

## Agreement matrix

| Subject | Identity | Semantic | Taxonomic | Operational | Documentary |
|---|---|---|---|---|---|
| Incremental second central moment | compatible | divergent | equivalent | divergent | compatible |

The divergence is narrow. Both imports agree on the recurrence, maintained
quantity, finalization domains, incremental/pairwise boundary, complexity
inferences and all three headline request outcomes.

## Strong agreement

Both imports independently conclude that:

- the normalized identity must not be named after Welford when the 1962 text was
  not read;
- the maintained object is `(count, mean, M2)`, where `M2` is the corrected sum
  of squared deviations rather than either variance convention;
- population variance and unbiased sample variance are separate finalizations
  with domains `n > 0` and `n > 1`;
- incremental update and two-state pairwise combination are related but distinct
  algorithms, with singleton specialization linking them;
- O(n) total work and O(1) retained scalar state for incremental evaluation are
  inferences from the fixed-size recurrence;
- exact-arithmetic identities do not establish bit-for-bit equality between
  IEEE-754 evaluation or partition orders;
- no executable implementation or public schema change belongs in K-M4-W.

The common requests receive the same headline decisions:

1. one-pass `(count, mean, M2)` with O(1) retained state: `accept
   conditionally`;
2. total unbiased variance over empty and singleton inputs: `reject`;
3. bitwise equality across all sequential and pairwise IEEE-754 orders:
   `reject`.

## Divergence 1: empty input

Importer A recommends a source-faithful nonempty problem. An empty-state wrapper
may be added later as a declared adaptation. Importer B recommends a totalized
problem whose state is `(0, absent, 0)` for empty input, while explicitly
identifying that convention as an Atlas adaptation rather than a source fact.

- Decision affected: selection for an empty finite stream and identity of a
  total versus partial accumulator API.
- Cause: protocol ambiguity about whether normalization may totalize a source
  contract, plus source ambiguity because neither report defines an empty mean.
- Minimal discriminant: request a result for an empty stream with no external
  wrapper and require every output component to be source-declared.
- Adjudication: retain the nonempty mathematical problem as the conservative
  source contract. Record `(0, absent, 0)` only as a possible API adaptation
  until partial outputs or optional state have an accepted model.

This does not reject total APIs. It prevents an adaptation from silently
becoming the source algorithm's guarantee.

## Divergence 2: proof level

Importer A records report equations and domain facts as `declared`, with no
imported claim at `proven`. Importer B observes that Pébay's Proposition 2.1 has
a source proof and could be described as `proven` in the worksheet, while also
noting that Atlas has no structural external-proof artifact.

- Decision affected: any future selection requiring a `proven` rather than
  `declared` algebraic identity.
- Cause: protocol ambiguity and model insufficiency. Schema 0.1 enumerates
  `proven` but does not define what proof artifact, review or verifier promotes a
  published argument to that level.
- Minimal discriminant: require the pairwise second-moment identity with evidence
  level `proven` using only the cited report and no imported proof object.
- Adjudication: keep the claim `declared` pending the ontology review. The review
  must define evidence-level semantics before any automatic promotion.

## Documentary differences

The proposed IDs differ only in spelling conventions. A uses
`online_second_central_moment_state` and
`incremental_centered_second_moment`; B uses
`maintain-online-second-central-moment` and
`incremental-centered-second-moment-update`. Both preserve the same problem and
strategy boundaries. Differences in `S` versus `M2`, running sum versus mean,
and citation granularity are acceptable declared transformations.

## Source result

The open reports repair the algorithmic evidence gap without pretending to
repair historical fidelity:

- Pébay supplies an openly readable recurrence, pairwise formula and singleton
  specialization;
- Chan, Golub and LeVeque independently supply updating and pairwise formulations
  plus numerical motivation;
- Welford 1962 remains an unresolved historical attribution because its text was
  not part of this packet.

K-M4-W therefore changes the result from “algorithm unavailable” to “algorithm
sourceable under a neutral identity,” not to “Welford 1962 verified.”

## Gate consequence

K-M4-W is **complete with strong agreement and two bounded divergences**. It
adds two concrete items to the already required ontology review:

1. partial source contract versus totalized API adaptation;
2. operational meaning and promotion rule for evidence level `proven`.

Corpus growth remains paused. No schema, registry, AST, implementation or
runtime change is justified by this addendum alone.
