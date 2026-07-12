# DEC-019 - Optional no_std hash set

- Status: accepted
- Class: C
- Date: 2026-07-11

## Context

The accepted corpus includes stable hash-based deduplication, while
`atlas-algorithms` must remain usable without `std`.

## Decision

Use `hashbrown` as an optional dependency enabled by the `hash-dedup` feature.
The feature also enables `alloc` and hashbrown's default hasher. It is part of
the workspace default feature set so the registered implementation is tested.

## Consequences

Core-only and alloc-without-hashing builds do not include hashbrown. The default
hasher is non-cryptographic and is not claimed to resist adversarial HashDoS
inputs. Changing the hasher or accepting hostile inputs requires a separate
review. Hash-based costs distinguish expected and worst-case time.
