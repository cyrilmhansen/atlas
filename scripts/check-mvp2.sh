#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

run() {
    printf '\n+ %s\n' "$*"
    "$@"
}

run ./scripts/check-mvp1.sh
run cargo run -q -p atlas --locked --offline --example dataset_specs
run cargo run -q -p atlas --locked --offline --example record_sort_correction
run cargo run -q -p atlas --locked --offline --example record_partition_correction
sort_execution_id=$(awk '/^id: / { print $2; exit }' \
    build/executions/sort-insertion-uniform-64-correction.yaml)
run cargo run -q -p atlas --locked --offline -- replay "$sort_execution_id"
run cargo run -q -p atlas --locked --offline -- qualify sequence.sort --stable --in-place --allocation none

for execution in \
    build/executions/sort-insertion-uniform-64-correction.yaml \
    build/executions/partition-in-place-alternating-64-correction.yaml
do
    if [ ! -s "$execution" ]; then
        printf '%s\n' "MVP 2 check failed: missing generated execution $execution" >&2
        exit 1
    fi
done

printf '\nMVP 2 acceptance slice passed. Benchmark timing is intentionally not run.\n'
