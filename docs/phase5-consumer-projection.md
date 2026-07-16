# Phase 5 - Consumer-driven projection

Status: complete; bounded projection supported

Date: 2026-07-17

## Question

Can Atlas answer one concrete consumer request by projecting candidates and
decision facts directly from the authoritative registry, without a handwritten
overlay or candidate-specific selection code?

The request is: select an exact `stream.top_k` implementation that performs no
allocation. The correct current result is rejection of both candidates because
their implementation effects declare bounded allocation.

## Experiment

The private in-memory projector:

1. finds Algorithms whose `solves` relation names the requested Problem;
2. finds Implementations whose `implements` relation names those Algorithms;
3. projects the declared problem-contract capability from `solves` and
   `implements`;
4. projects the allocation effect from `Implementation.effects`;
5. evaluates the consumer request with the unchanged private evaluator.

It writes no overlay, introduces no persistent format and does not interpret
cost strings. The test derives its expected candidate set independently from
the same registry relations. Therefore a newly registered conforming candidate
changes the expected set and must be discovered without projector changes.

## Acceptance

- both current top-k candidates are discovered without candidate IDs in the
  projecter;
- both are rejected with the explicit forbidden-allocation reason;
- an unknown Problem produces an actionable error;
- schema, CLI and persistent formats remain unchanged.

## Result and verdict

The committed registry yields both current top-k implementations, and both are
rejected with `forbidden effect effect.allocates_storage`. An unknown Problem
is rejected before projection.

The falsifier executes a test-only caller-storage top-k fixture and verifies
its exact descending result without capacity growth. It then adds matching
Algorithm and Implementation manifests through structured YAML and
deserializes the augmented registry. The projector discovers the third candidate automatically and the
unchanged evaluator accepts it. Neither projector nor request contains an
existing candidate identifier, and the fixture is not added to the
authoritative corpus.

**Verdict: supported for this bounded request.** Atlas can project candidate
identity, declared problem compatibility and one structured implementation
effect directly from schema 0.1 into the private evaluator. This removes the
manual overlay step for the tested request without a new format or public
interface.

The result does not generalize to conditioned costs, state compatibility,
negative evidence or algebraic complexity comparison. Those facts are not
silently inferred, and this single successful projection does not justify a
generic projector. Phase 5 closes after this experiment.
