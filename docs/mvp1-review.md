# MVP 1 exit review

Review date: 2026-07-11.

## Result

The pilot corpus and reference CLI are complete. The project demonstrates that
one aggregate manifest can describe separate problems, algorithms, and concrete
implementations with qualified claims, explicit effects, and executable tests.
The algorithm crate also demonstrates a real `core` / `alloc` / optional hashing
runtime boundary.

MVP 1 should remain active until reproducibility and evidence-integrity criteria
are closed. Corpus completion alone is not sufficient.

## Evidence

| Area | Evidence | Assessment |
|---|---|---|
| Corpus | 10 problems, 15 algorithms, 20 implementations | Complete |
| CLI | Five vision commands implemented and integration-tested | Complete |
| Extension | Nine successive problem additions required no validator redesign | Complete |
| Provenance | Claims are qualified; local files, implementation IDs, and Rust tests resolve | Complete for MVP 1 |
| Runtime | 28 core-only tests; alloc and hash profiles isolated | Complete |
| Correction | 49 algorithm tests in the default profile | Complete |
| Registry | 49 projection, validation, and CLI tests | Complete |
| Git authority | Local root commit contains the complete MVP 1 baseline | Complete locally |
| SQLite | 45 entities, 35 relations, 313 claims; reproducible digest | Complete |

## Gaps

### Schema completeness

Implementation version, license, target, dependencies, and ABI are now mandatory
qualified claims. Compatibility and migration rules are documented. Types and
requirement expressions remain human-readable strings rather than a validated
grammar; this is an explicit pre-freeze limitation rather than missing metadata.

### Evidence integrity

DEC-026 defines the restricted textual source grammar. The validator resolves
workspace files, implementation IDs, and Rust test symbols and rejects stale or
malformed local references. Documentary references are syntax-checked but not
fetched; MVP 1 performs no network validation.

A coverage command is also needed to measure the 90% mandatory-property exit
criterion rather than infer it from successful deserialization.

### Reproducibility and indexing

Projection version 1 is implemented with system SQLite, normalized tables, and a
canonical SHA-256 logical digest. Tests rebuild twice, remove injected stale
rows, compare digests, and demonstrate digest changes after logical edits. YAML
remains authoritative; SQLite files are not compared byte-for-byte.

### Repository baseline

The local root commit establishes Git authority and commit provenance for MVP 1.
No remote publication has occurred. CI remains separate and must invoke the
accepted local gate rather than duplicate it.

### Documentation authority

The vision is now maintained in `docs/vision.md`. The original `.docx` is kept as
the immutable 0.1 snapshot under DEC-020. Imported HTML table fragments remain a
documentation-cleanup opportunity but do not prevent textual review.

## Recommended closure order

1. Select a CI platform and make it invoke `scripts/check-mvp1.sh`.
2. Publish only after separate human approval of a remote and branch policy.
