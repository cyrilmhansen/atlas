#!/bin/sh
set -eu

compiler=${RISCV64_CC:-riscv64-linux-gnu-gcc}
objdump=${RISCV64_OBJDUMP:-riscv64-linux-gnu-objdump}
emulator=${QEMU_RISCV64:-qemu-riscv64}
workspace=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
prefix=${TMPDIR:-/tmp}/atlas-mir-rv64-generator-$$
binary=$prefix.elf
code=$prefix.bin
disassembly=$prefix.txt
probe_object=$prefix-probe.o
mir_object=$prefix-mir.o
generator_object=$prefix-generator.o
trap 'rm -f "$binary" "$code" "$disassembly" "$probe_object" "$mir_object" "$generator_object"' EXIT

for tool in "$compiler" "$objdump" "$emulator" file; do
  if ! command -v "$tool" >/dev/null; then
    echo "required RV64 generator probe tool not found: $tool" >&2
    exit 1
  fi
done

if ! grep -q 'MIR_gen_set_code_observer' "$workspace/vendor/mir/mir-gen.h"; then
  echo "vendored MIR observer patch is not applied; run scripts/apply-mir-patches.sh" >&2
  exit 1
fi

"$compiler" -std=gnu11 -O2 -Wall -Wextra -Werror -march=rv64gc -mabi=lp64d \
  -I "$workspace/vendor/mir" -c \
  "$workspace/crates/atlas-mir/tests/rv64_mir_generator.c" -o "$probe_object"
"$compiler" -std=gnu11 -O2 -march=rv64gc -mabi=lp64d -I "$workspace/vendor/mir" \
  -c "$workspace/vendor/mir/mir.c" -o "$mir_object"
"$compiler" -std=gnu11 -O2 -march=rv64gc -mabi=lp64d -I "$workspace/vendor/mir" \
  -c "$workspace/vendor/mir/mir-gen.c" -o "$generator_object"
"$compiler" -march=rv64gc -mabi=lp64d -static "$probe_object" "$mir_object" \
  "$generator_object" -ldl -lm -o "$binary"
file "$binary" | grep -q 'ELF 64-bit LSB executable, UCB RISC-V'

result=$("$emulator" "$binary" "$code")
printf '%s\n' "$result" | grep -Eq '^mir-rv64:add:42:bytes:[1-9][0-9]*$'
test -s "$code"

"$objdump" -D -b binary -m riscv:rv64 "$code" >"$disassembly"
grep -Eq '[[:space:]](add|c\.add)[[:space:]]' "$disassembly"
grep -Eq '[[:space:]]ret([[:space:]]|$)' "$disassembly"

printf '%s\n' "$result"
grep -E '[[:space:]](add|c\.add|ret)([[:space:]]|$)' "$disassembly"
