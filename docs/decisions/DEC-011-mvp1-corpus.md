# DEC-011 - MVP 1 corpus composition

- Status: accepted
- Class: B
- Date: 2026-07-11

## Decision

Use the balanced ten-problem, fifteen-algorithm corpus defined in
`docs/mvp1-corpus.md`. It covers sorting, searching, selection, filtering,
partitioning, reversal, merging, order validation, and deduplication.

## Consequences

The corpus satisfies the numeric MVP 1 target without introducing numeric
reduction semantics yet. Replacing a planned problem or algorithm requires an
explicit revision of this decision, but does not by itself change schema 0.1.
