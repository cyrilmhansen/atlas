#!/bin/sh
set -eu

compiler=${RISCV64_CC:-riscv64-linux-gnu-gcc}
objdump=${RISCV64_OBJDUMP:-riscv64-linux-gnu-objdump}
emulator=${QEMU_RISCV64:-qemu-riscv64}
workspace=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
prefix=${TMPDIR:-/tmp}/atlas-mir-rv64-generator-$$
binary=$prefix.elf
add_code=$prefix-add.bin
sorted_code=$prefix-is-sorted.bin
reverse_code=$prefix-reverse.bin
add_disassembly=$prefix-add.txt
sorted_disassembly=$prefix-is-sorted.txt
reverse_disassembly=$prefix-reverse.txt
probe_object=$prefix-probe.o
mir_object=$prefix-mir.o
generator_object=$prefix-generator.o
trap 'rm -f "$binary" "$add_code" "$sorted_code" "$reverse_code" "$add_disassembly" "$sorted_disassembly" "$reverse_disassembly" "$probe_object" "$mir_object" "$generator_object"' EXIT

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

result=$("$emulator" "$binary" "$add_code" "$sorted_code" "$reverse_code")
printf '%s\n' "$result" | grep -Eq '^mir-rv64:add:42:bytes:[1-9][0-9]*$'
printf '%s\n' "$result" | grep -Eq '^mir-rv64:is_sorted:cases:4:loads:10:bytes:[1-9][0-9]*$'
printf '%s\n' "$result" | grep -Eq '^mir-rv64:reverse:cases:4:loads:12:stores:12:bytes:[1-9][0-9]*$'
test -s "$add_code"
test -s "$sorted_code"
test -s "$reverse_code"

"$objdump" -D -b binary -m riscv:rv64 "$add_code" >"$add_disassembly"
grep -Eq '[[:space:]](add|c\.add)[[:space:]]' "$add_disassembly"
grep -Eq '[[:space:]]ret([[:space:]]|$)' "$add_disassembly"
"$objdump" -D -b binary -m riscv:rv64 "$sorted_code" >"$sorted_disassembly"
grep -Eq '[[:space:]](jal|jalr)[[:space:]]' "$sorted_disassembly"
grep -Eq '[[:space:]]b(eq|ne|lt|ge|ltu|geu)[[:space:]]' "$sorted_disassembly"
grep -Eq '[[:space:]]ret([[:space:]]|$)' "$sorted_disassembly"
"$objdump" -D -b binary -m riscv:rv64 "$reverse_code" >"$reverse_disassembly"
test "$(grep -Ec '[[:space:]](jal|jalr)[[:space:]]' "$reverse_disassembly")" -ge 4
grep -Eq '[[:space:]]b(eq|ne|lt|ge|ltu|geu)[[:space:]]' "$reverse_disassembly"
grep -Eq '[[:space:]]ret([[:space:]]|$)' "$reverse_disassembly"

printf '%s\n' "$result"
grep -E '[[:space:]](add|c\.add|ret)([[:space:]]|$)' "$add_disassembly"
grep -E '[[:space:]](jal|jalr|b(eq|ne|lt|ge|ltu|geu)|ret)([[:space:]]|$)' \
  "$sorted_disassembly"
grep -E '[[:space:]](jal|jalr|b(eq|ne|lt|ge|ltu|geu)|ret)([[:space:]]|$)' \
  "$reverse_disassembly"
