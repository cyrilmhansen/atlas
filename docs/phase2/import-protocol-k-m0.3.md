# Phase 2 import protocol k-m0.3

Status: accepted for imports after K-M4-W

Predecessor: `k-m0.2`, preserved for completed K-M0 through K-M4-W worksheets

Authority: DEC-066, DEC-067 and DEC-068

## Purpose

Revision `k-m0.3` narrows normalization choices that caused independent imports
to make different knowledge decisions. It changes import procedure, not schema
0.1 or historical worksheets.

The manual worksheet and equivalence rubric remain applicable. A future
worksheet using this revision states `Protocol: k-m0.3` and applies the rules
below in addition to their existing sections.

## Problem and algorithm identity

- Define `Problem` from observable input, required domain, output and guarantee.
- Keep representation, orientation, data structure and implementation mechanism
  in `Algorithm` or implementation requirements unless they change the requested
  observable result.
- Do not put a conventional strategy name in a source-faithful identity when the
  mandatory source does not establish that strategy. It may appear as an
  `inferred` alias with the inference explicit.
- Preserve exact source contracts before recording projection, specialization,
  guarded finalization or stage/refinement relationships.
- Two operations over shared persistent state remain distinct contracts when
  their inputs, outputs, effects or validity domains differ.

Minimal check: a second representation solving the same observable request
should share the proposed `Problem`; a request for a different output or
guarantee should not.

## Source contract versus adaptation

- Do not silently totalize a partial source contract. Empty-state wrappers,
  sentinel values, default outputs and error policies are declared
  transformations.
- Record source input domain separately from an implementation's wider accepted
  domain.
- Keep corrected sums, population estimators, unbiased estimators and other
  guarded finalizers distinct when their output meaning or domain differs.
- A later source can support a neutral algorithm identity without retroactively
  establishing textual fidelity to an inaccessible historical source.

Minimal check: evaluate empty, singleton and first-invalid-boundary inputs before
claiming two imports have equivalent domains.

## Evidence attachment

- A contextual source may define vocabulary or a broad convention; it does not
  establish a version-specific implementation claim unless the packet states
  and justifies that evidentiary role.
- Record source-local claims before Atlas inference. Complexity restatements
  inferred from fixed-size recurrences remain `inferred` even when obvious.
- Apply DEC-068 to `proven`. A published proof without an imported, reviewed
  claim mapping remains `declared` evidence.
- Source access failure is a packet/source-coverage result, not model
  insufficiency. Supply a frozen readable primary alternative where lawful.
- Open access permits inspection but does not imply a software or documentation
  license.

## Qualified behavior

Every worksheet explicitly separates, when applicable:

- worst, expected and amortized cost, including the operation and sequence being
  bounded;
- deterministic transition from distributional/randomized quality;
- exact arithmetic from a specific floating-point environment;
- exact output, sound superset, one-sided error and distributional guarantee;
- persistent state, auxiliary scratch, output storage and allocation;
- algorithm guarantee from implementation adaptation and tested observation.

If schema 0.1 cannot represent a distinction, keep it in the worksheet or the
DEC-067 experiment. Do not choose a misleading required field value.

## Required operational discriminants

Before submission, each imported subject includes at least three identical
selection requests shared by all importers. At least one request must exercise a
boundary or failure regime, and at least one must distinguish a source guarantee
from an Atlas adaptation or evidence level.

Two imports are operationally equivalent only when the same normalized request,
conditions and evidence threshold produce the same accept/reject result.
Matching headline words such as “accept conditionally” are insufficient when
their conditions differ.

## Protocol change control

- Completed `k-m0.2` worksheets are never rewritten to appear compliant.
- A correction that changes a prior decision is an addendum with its own packet
  and comparison.
- New protocol rules require an observed divergence or repeated ambiguity.
- No protocol revision may add an automatic importer, ontology merger, prose
  parser or new public schema field.
