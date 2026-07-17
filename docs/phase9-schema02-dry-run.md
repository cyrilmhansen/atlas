# Phase 9 - Schema 0.2 dry run

Status: closed supported as a non-authoritative dry run (2026-07-17)

## Question and acceptance

Can Atlas deterministically transform the complete authoritative schema 0.1
document into the accepted generic-cost shape without writing it or losing
entity identity, claims, provenance or conditions?

The dry run must:

- produce identical YAML on repeated in-memory migrations;
- preserve Problem, Algorithm, Implementation and Execution counts;
- consume every `time_worst`, optional `time_expected` and
  `auxiliary_memory` Algorithm claim into a typed cost profile;
- preserve each migrated value, evidence level and source;
- qualify hash-map expected time and add the reviewed heap time/allocation
  profiles with resolvable condition IDs;
- reject unknown conditions, duplicate profiles and incomplete claims;
- leave `registry/atlas.yaml` unchanged.

The draft uses `schema_version: 0.2-draft`; it is an ephemeral serialization,
not an accepted public version or persistent artifact.

## Result and verdict

Two independent migrations of the complete aggregate document produce
byte-identical YAML. The typed draft retains 31 Problems, 39 Algorithms, 43
Implementations and the empty Execution collection. Every legacy worst-time,
optional expected-time and auxiliary-memory claim is consumed exactly once;
its bound, evidence level and source survive the conversion.

The reviewed hash-map `O(1)` expected claim gains the nonadversarial-distribution
condition. Both heap-push algorithms retain their unconditional growth cost and
gain the reviewed `O(log n)` time and no-allocation profiles under spare
capacity. Unknown condition references, duplicate profiles and empty provenance
are rejected by independent negative fixtures.

Phase 9 therefore supports the migration algorithm and draft shape. It does not
validate SQLite, CLI or Explorer consumers and does not authorize replacing
schema 0.1. No authoritative or derived data file is written.

Verification:

```text
cargo test -p atlas schema02_ --locked --offline
```
