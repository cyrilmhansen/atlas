# Atlas aggregate schema 0.1

Schema 0.1 is one UTF-8 YAML document. It contains `schema_version` and the
four separate entity collections `problems`, `algorithms`, `implementations`,
and `executions`. The entire document is loaded in memory before its references
are validated.

The `executions` collection is present but empty in the first slice. Execution
fields will only be added when a real MVP 1 case requires them; extending the
public schema requires a class C decision.

## Identities and references

`id`, `solves`, and `implements` are structural fields. IDs are globally unique
and consist of lowercase ASCII letters, digits, dots, underscores, or hyphens.
References must resolve to an entity of the expected kind.

## Claims

Every non-structural property is a claim:

```yaml
stable:
  value: true
  level: declared
  source: "vision:0.1#6"
```

`level` is one of `declared`, `inferred`, `tested`, `observed`, or `proven`.
`source` follows the restricted MVP 1 grammar accepted in DEC-026:

```text
source := scheme ":" target (";" target)*
```

Targets are non-empty and share the source scheme. Recognized schemes are
`file`, `test`, `tests`, `implementation`, `implementations`, `definition`,
`vision`, `command`, `analysis`, and `docs`. Local files, implementation IDs,
and Rust test symbols are resolved during repository validation. Documentary
schemes are syntax-checked only; no network access is performed. Semicolons are
not escaped in schema 0.1.

## Collections

A problem contains claims named `input`, `output`, and `ensures`. The optional
`requires` claim lists conditions inherent to the problem contract and must be
non-empty when present.

An algorithm contains a `solves` reference and the common claims `name`,
`deterministic`, `time_worst`, and `auxiliary_memory`. The typed claims `stable`
and `in_place` are optional because they are specific to algorithm families
where order preservation or mutation strategy is meaningful. Absence makes no
assertion and must not be interpreted as `false`.

The optional `requires` claim is a list of requirements beyond the referenced
problem contract. Its absence means no additional requirement is declared, not
that unrestricted applicability has been proven. Schema 0.1 treats requirement
expressions as human-readable strings.

The optional `time_expected` claim records an expected-time bound separately
from the required `time_worst` claim. An expected bound must never be interpreted
as a worst-case guarantee.

An implementation contains an `implements` reference and claims named
`language`, `version`, `license`, `target`, `dependencies`, `abi`, `entrypoint`,
`signature`, `effects`, and `tests`. Effects explicitly record mutation, I/O,
blocking, and allocation behavior. An empty dependency list is valid. ABI values
must state when no stable ABI is provided rather than implying binary stability.

## Versioning rules

Schema 0.1 remains pre-freeze until the MVP 1 acceptance gate passes.

- Adding an optional field is backward-compatible for readers that reject no
  unknown fields only after they explicitly support the new field.
- Adding a required field, changing field meaning or type, changing identifier
  semantics, or removing a field is breaking and requires a new schema version.
- A breaking version requires a deterministic whole-document migration and
  validation before replacing the authoritative manifest.
- Readers reject unsupported versions; they never silently coerce them.
- Derived SQLite schemas have their own projection version and never determine
  the source schema version.

Unknown fields are rejected so a typo cannot silently change the model.
