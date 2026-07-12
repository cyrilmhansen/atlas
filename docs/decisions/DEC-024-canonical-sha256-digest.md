# DEC-024 - Canonical logical SHA-256 digest

- Status: accepted
- Class: C
- Date: 2026-07-12

## Decision

Compute SHA-256 over projection metadata, entities, relations, and claims read in
explicit sorted order. Each field is encoded as UTF-8 with an explicit byte
length before hashing. Exclude the digest metadata row itself.

## Consequences

The digest identifies logical rows independently of SQLite page layout and
insertion history. It is a reproducibility identifier, not a signature or proof
of trust. The implementation uses RustCrypto `sha2`.
