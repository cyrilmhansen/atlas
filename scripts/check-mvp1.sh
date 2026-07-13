#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

run() {
    printf '\n+ %s\n' "$*"
    "$@"
}

run ./scripts/apply-mir-patches.sh
run cargo fmt --all -- --check

run cargo test -p atlas-algorithms --no-default-features --locked --offline
run cargo test -p atlas-algorithms --no-default-features --features alloc --locked --offline
run cargo test -p atlas-algorithms --all-features --locked --offline
run cargo test --workspace --locked --offline

run cargo clippy -p atlas-algorithms --no-default-features --all-targets --locked --offline -- -D warnings
run cargo clippy -p atlas-algorithms --no-default-features --features alloc --all-targets --locked --offline -- -D warnings
run cargo clippy --workspace --all-features --all-targets --locked --offline -- -D warnings

run cargo run -q -p atlas --locked --offline -- validate

database="${TMPDIR:-/tmp}/atlas-mvp1-index-$$.sqlite3"
trap 'rm -f "$database"' EXIT HUP INT TERM

printf '\n+ atlas index determinism\n'
first=$(cargo run -q -p atlas --locked --offline -- index "$database")
second=$(cargo run -q -p atlas --locked --offline -- index "$database")
printf '%s\n' "$first"
if [ "$first" != "$second" ]; then
    printf '%s\n' "MVP 1 check failed: repeated index output differs" >&2
    printf '%s\n' "$second" >&2
    exit 1
fi

printf '\nMVP 1 acceptance gate passed.\n'
