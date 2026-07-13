#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

if [ "$#" -ne 1 ]; then
    printf '%s\n' "usage: scripts/record-sort-comparison-linux.sh CPU" >&2
    exit 2
fi

cpu=$1
case "$cpu" in
    ''|*[!0-9]*)
        printf '%s\n' "CPU must be a non-negative integer" >&2
        exit 2
        ;;
esac

if [ "$(uname -s)" != "Linux" ]; then
    printf '%s\n' "this runner requires Linux" >&2
    exit 2
fi
for command in git taskset nproc sed cargo awk tr; do
    if ! command -v "$command" >/dev/null 2>&1; then
        printf '%s\n' "required command not found: $command" >&2
        exit 2
    fi
done
if ! taskset --cpu-list "$cpu" true >/dev/null 2>&1; then
    printf '%s\n' "CPU $cpu is not available to this process" >&2
    exit 2
fi
if [ -n "$(git status --porcelain --untracked-files=normal)" ]; then
    printf '%s\n' "refusing comparison campaign: the worktree must be clean" >&2
    exit 2
fi

load_one=$(awk '{ print $1 }' /proc/loadavg)
logical_cpus=$(nproc)
if ! awk -v load_value="$load_one" -v cpus="$logical_cpus" \
    'BEGIN { exit !(load_value + 0 <= cpus / 2) }'; then
    printf '%s\n' "refusing campaign: 1-minute load $load_one exceeds half of $logical_cpus available CPUs" >&2
    exit 1
fi

cpufreq="/sys/devices/system/cpu/cpu${cpu}/cpufreq"
governor=unavailable
maximum_frequency_khz=unavailable
if [ -r "$cpufreq/scaling_governor" ]; then
    governor=$(sed -n '1p' "$cpufreq/scaling_governor")
fi
if [ -r "$cpufreq/cpuinfo_max_freq" ]; then
    maximum_frequency_khz=$(sed -n '1p' "$cpufreq/cpuinfo_max_freq")
fi

printf '%s\n' "Recording one clean sorting comparison campaign on CPU $cpu."
printf '  governor: %s (not modified)\n' "$governor"
printf '  maximum frequency: %s kHz\n' "$maximum_frequency_khz"
printf '  1-minute load: %s / %s available CPUs\n' "$load_one" "$logical_cpus"
printf '%s\n' "Each implementation is measured once; a rejected quality gate stops the campaign."

cargo build --release -q -p atlas-bench --locked --offline --example record_sort_benchmark

execution_ids=""
for implementation in \
    sort.merge.rust.slice.v1 \
    sort.merge_with_scratch.rust.slice.v1 \
    sort.insertion.rust.slice.v1
do
    printf '\n+ taskset --cpu-list %s target/release/examples/record_sort_benchmark %s\n' \
        "$cpu" "$implementation"
    taskset --cpu-list "$cpu" \
        target/release/examples/record_sort_benchmark "$implementation"
    output="build/executions/$(printf '%s' "$implementation" | tr '.' '-')-uniform-2048-benchmark.yaml"
    execution_id=$(awk '/^id: / { print $2; exit }' "$output")
    if [ -z "$execution_id" ]; then
        printf '%s\n' "missing execution ID in $output" >&2
        exit 1
    fi
    execution_ids="$execution_ids $execution_id"
done

# Deliberately invoke comparison once: the script never retries a measurement.
# shellcheck disable=SC2086
cargo run -q -p atlas --locked --offline -- compare $execution_ids
