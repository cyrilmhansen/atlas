#!/bin/sh
set -eu

cd "$(dirname "$0")/.."

if [ "$#" -ne 1 ]; then
    printf '%s\n' "usage: scripts/run-benchmark-linux.sh CPU" >&2
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
for command in taskset nproc awk sed cargo; do
    if ! command -v "$command" >/dev/null 2>&1; then
        printf '%s\n' "required command not found: $command" >&2
        exit 2
    fi
done
if ! taskset --cpu-list "$cpu" true >/dev/null 2>&1; then
    printf '%s\n' "CPU $cpu is not available to this process" >&2
    exit 2
fi

load_one=$(awk '{ print $1 }' /proc/loadavg)
logical_cpus=$(nproc)
if ! awk -v load_value="$load_one" -v cpus="$logical_cpus" \
    'BEGIN { exit !(load_value + 0 <= cpus / 2) }'; then
    printf '%s\n' "refusing benchmark: 1-minute load $load_one exceeds half of $logical_cpus available CPUs" >&2
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

printf '%s\n' "Linux benchmark preflight:"
printf '  CPU: %s\n' "$cpu"
printf '  governor: %s (not modified)\n' "$governor"
printf '  maximum frequency: %s kHz\n' "$maximum_frequency_khz"
printf '  1-minute load: %s / %s available CPUs\n' "$load_one" "$logical_cpus"

cargo build --release -q -p atlas-bench --locked --offline --example compare_sorts

status=0
for implementation in \
    sort.merge.rust.slice.v1 \
    sort.merge_with_scratch.rust.slice.v1 \
    sort.insertion.rust.slice.v1
do
    printf '\n+ taskset --cpu-list %s target/release/examples/compare_sorts %s\n' \
        "$cpu" "$implementation"
    if ! taskset --cpu-list "$cpu" \
        target/release/examples/compare_sorts "$implementation"
    then
        status=1
    fi
done
exit "$status"
