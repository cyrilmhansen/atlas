# DEC-018 - Problem requirements

- Status: accepted
- Class: C
- Date: 2026-07-11

## Context

`sequence.merge_sorted` is defined on two sorted input sequences. This condition
belongs to the problem contract, unlike binary search's sorted-input assumption,
which is specific to one algorithm solving the broader `sequence.search` problem.

## Decision

Add `requires: Claim<Vec<String>>` as an optional `Problem` field. A present
requirement list must be non-empty and carry provenance.

## Consequences

Problem requirements define admissible inputs for every solving algorithm.
Algorithm requirements remain additional strategy-specific assumptions. Absence
at either level means no requirement is declared at that level, not that
unrestricted applicability has been proven.
