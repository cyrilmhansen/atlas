# DEC-068 - Reserve proven evidence for an auditable proof mapping

## Status

Accepted on 2026-07-15 (`proven-A`).

## Context

Schema 0.1 enumerates `proven` but does not define its promotion rule. K-M4-W
showed that independent importers can reasonably disagree on whether a
published proposition makes the normalized Atlas claim `proven` or merely
source-declared.

## Decision

Use `proven` only when all of the following are inspectable:

1. the exact Atlas claim being supported;
2. a stable proof artifact containing the relevant argument or certificate;
3. an explicit mapping from the artifact's statement and assumptions to the
   Atlas claim;
4. the Atlas review or verifier method that accepted that mapping.

The artifact need not come from a proof assistant. Human-reviewed mathematical
proofs are eligible when the claim mapping, assumptions and review record are
auditable. A primary source stating or proving a theorem supports a `declared`
claim such as “source S proves X” until Atlas records that mapping and review.

`tested` and `observed` never promote automatically to `proven`. MIR, native,
WASM or benchmark agreement cannot serve as a semantic proof by backend
consensus.

## Consequences

- K-M4-W pairwise moment identities remain `declared` during the experiment.
- Future proof promotion is additive and does not change the claim value.
- A selection threshold using `proven` may rely only on validated mappings, not
  bibliographic reputation or the word “proof” in prose.
- The private K-M5 overlay must reject a `proven` fact without artifact, claim
  mapping and review metadata.
- Schema 0.1 syntax does not change; a public proof-artifact representation
  remains a separate class C decision.

## Alternatives considered

- Treat a source-presented proof as immediately `proven`: rejected because it
  conflates publication with Atlas verification and reproduced the K-M4-W
  importer divergence.
- Suspend all use of `proven`: rejected because auditable human or machine proof
  artifacts can already satisfy a meaningful stronger level.
