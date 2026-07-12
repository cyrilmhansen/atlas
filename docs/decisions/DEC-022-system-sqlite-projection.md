# DEC-022 - System SQLite projection

- Status: accepted, system linkage verified
- Class: C
- Date: 2026-07-11

## Decision

Use `rusqlite` linked to a system SQLite library for the derived MVP 1 index.
Do not enable bundled SQLite by default. YAML and Git remain authoritative.

## Consequences

The build requires SQLite development headers, library, and discoverable linker
metadata. The current environment contains SQLite 3.53.3 but its `pkg-config`
search path omits `/usr/lib/pkgconfig`; validation therefore uses an explicit
`PKG_CONFIG_PATH` during the experiment rather than committing a host-specific
absolute path.

If ordinary supported environments cannot discover system SQLite reliably, this
decision must be amended before enabling the bundled feature.

Verified with `rusqlite` 0.40.1 linked to system SQLite 3.53.3 by setting
`PKG_CONFIG_PATH=/usr/lib/pkgconfig` in the current environment.
