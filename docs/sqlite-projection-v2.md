# SQLite projection version 2

The SQLite database remains a disposable, transactionally rebuilt index of the
Git-authoritative YAML registry.

Version 2 adds `condition` to entity kinds. Every generic Algorithm cost profile
is stored as `claims.path = costs[i]` using a deterministic length-prefixed
`c:4` encoding of metric, regime, bound and required-condition list. Each
condition reference is also indexed as a relation named `costs[i].requires`.

All version 1 tables, strictness rules and logical-digest ordering remain. The
metadata records projection version `2` and source schema version `0.2`.
Version 1 databases are not migrated in place; `atlas index` rebuilds them from
the authoritative manifest.

```sh
cargo run -p atlas -- index
cargo run -p atlas -- index path/to/atlas.sqlite3
```
