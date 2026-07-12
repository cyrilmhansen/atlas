# SQLite projection version 1

The SQLite database is a disposable, derived index of the validated aggregate
YAML registry. It is never edited as an authoritative source and is ignored by
Git.

## Build

```sh
cargo run -p atlas -- index
cargo run -p atlas -- index path/to/atlas.sqlite3
```

The default path is `build/atlas.sqlite3`. The command replaces all logical rows
inside one transaction and prints row counts and the logical SHA-256 digest.

## Tables

- `projection_meta(key, value)` stores projection version, source schema version,
  and logical digest.
- `entities(id, kind, ordinal)` stores problem, algorithm, and implementation
  identities.
- `relations(source_id, relation, target_id, ordinal)` stores `solves` and
  `implements` references with foreign keys.
- `claims(entity_id, path, value, level, source, ordinal)` stores every qualified
  property.

All tables are SQLite `STRICT` tables. Projection version 1 contains no execution
rows because schema 0.1 does not yet define execution fields.

## Canonical claim values

Values are UTF-8 strings with explicit type and byte lengths:

- string: `s:<bytes>:<value>`;
- boolean: `b:0` or `b:1`;
- list: `l:<count>:<bytes>:<item>...`;
- effects: `e:4` followed by four length-prefixed encoded fields for mutations,
  I/O, blocking, and allocation.

This encoding distinguishes empty values and embedded separators without making
SQLite the semantic parser.

## Logical digest

SHA-256 is computed over projection metadata excluding the digest itself, then
entities, relations, and claims. Queries use explicit sort orders. Every field
is prefixed by its big-endian 64-bit UTF-8 byte length before hashing.

The digest identifies logical content independently of SQLite pages, file
timestamps, and insertion history. It is not a signature or trust proof.
