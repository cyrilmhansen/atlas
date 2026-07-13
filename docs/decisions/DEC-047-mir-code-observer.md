# DEC-047 - Prepare an upstream MIR code observer

## Status

Accepted on 2026-07-13 (`mir-observer-A`). The patch is vendored by Atlas;
external submission and any upstream submodule update require separate review.

## Decision

Prepare a narrow upstream-compatible generator callback that reports a fully
generated function after its machine code has been published and relocated.
The callback receives the MIR context, function item, executable address, exact
byte length and caller-owned data.

The callback is synchronous, disabled by default and must not reenter its MIR
context. The observed address remains valid until `MIR_finish`. Separately
generated lazy basic-block versions are outside the initial contract.

The reviewable upstream diff is kept in `patches/mir/code-observer.patch`.
`scripts/apply-mir-patches.sh` verifies the exact original commit and applies
the patch idempotently. The parent repository does not record an unpublished
submodule commit: clean clones and CI retain a retrievable original gitlink.

## Consequences

- Atlas can copy exact relocated machine-code bytes without debug text, files,
  `gcc`, `objcopy` or `objdump` subprocesses.
- Scalar addition and guest-memory `is_sorted` provide two materially different
  consumers: the latter includes control flow and relocated import calls.
- Machine-code length and executable allocation footprint remain distinct.
- A digest of relocated bytes is a local diagnostic, not a portable identity;
  addresses and target-specific relocations may change it.
- An embedded disassembler remains a separate dependency decision. It will
  consume a bounded byte slice instead of guessing a function boundary.
- Building `atlas-mir` from a fresh checkout requires the patch bootstrap;
  acceptance scripts and CI execute it explicitly.
- The patch and its upstream test must be reviewed before any external
  submission or submodule pointer update.
