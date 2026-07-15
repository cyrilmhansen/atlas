# DEC-069 - Test bounded equivalences between private encodings

## Status

Accepted on 2026-07-15 (`normalization-B`).

## Context

The independent K-M5 top-k author obtained all three frozen decisions but used
two taxonomies different from the reference experiment: exactness was fused
into an output capability, and allocation was an effect rather than a cost
assertion. Both encodings preserve the source, yet broader requests can select
different candidates.

Canonicalizing one encoding would reduce ambiguity but could overfit Atlas to
the first author. A general ontology or rule engine would exceed the bounded
purpose of DEC-067.

## Decision

Test private, explicitly authored equivalences between small sets of assertions.
An equivalence is bidirectional, evidence-bearing and non-recursive. It applies
only when every assertion on one side is directly present with an accepted
evidence level; it can then satisfy an assertion on the other side.

The experiment permits only:

- at most four equivalences;
- one to three assertions on each side;
- capability, guarantee, effect and exact unconditional cost assertions;
- one equivalence application per requested assertion;
- exact cost matching with no arithmetic or asymptotic ordering.

It prohibits chaining, transitive closure, variables, negation, state
conversion, conditional costs, candidate-specific evaluator branches and prose
inference. Mapping evidence and source-assertion evidence must both satisfy the
request. Forbidden effects remain conservative and ignore evidence thresholds,
matching existing evaluator behavior.

## Consequences

- The original reference overlay and independent submission remain unchanged.
- A separate top-k fixture compares fused and decomposed encodings.
- The private parser and evaluator may grow, but their complete line cost is
  recorded before any K-M5 conclusion.
- Schema 0.1, registry authority, SQLite, CLI and public query semantics remain
  unchanged.
- Evidence from this experiment cannot authorize a public equivalence language.

## Alternatives considered

- Canonical private decomposition (`normalization-A`): rejected by the human
  decision because it discards independently reasonable source encodings.
- Close K-M5 as mixed (`normalization-C`): deferred while a bounded compatibility
  experiment can measure value and complexity.
