#!/bin/sh
set -eu

workspace=$(CDPATH= cd -- "$(dirname "$0")/.." && pwd)
mir="$workspace/vendor/mir"
patch="$workspace/patches/mir/code-observer.patch"
expected_commit=a8ab7c31cd5f9b23b77d84c60b3d83e62d9d304c

if [ ! -f "$mir/mir-gen.c" ]; then
    printf '%s\n' "MIR patch failed: initialize vendor/mir first" >&2
    exit 1
fi
if [ ! -f "$patch" ]; then
    printf '%s\n' "MIR patch failed: missing $patch" >&2
    exit 1
fi

actual_commit=$(git -C "$mir" rev-parse HEAD)
if [ "$actual_commit" != "$expected_commit" ]; then
    printf '%s\n' \
        "MIR patch failed: expected $expected_commit, found $actual_commit" >&2
    exit 1
fi

if git -C "$mir" apply --unidiff-zero --check "$patch" >/dev/null 2>&1; then
    git -C "$mir" apply --unidiff-zero "$patch"
    printf '%s\n' "Applied vendored MIR code-observer patch."
elif git -C "$mir" apply --unidiff-zero --reverse --check "$patch" >/dev/null 2>&1; then
    printf '%s\n' "Vendored MIR code-observer patch is already applied."
else
    printf '%s\n' \
        "MIR patch failed: vendor/mir is neither clean nor correctly patched" >&2
    exit 1
fi
