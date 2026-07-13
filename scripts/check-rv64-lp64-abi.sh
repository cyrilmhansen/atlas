#!/bin/sh
set -eu

compiler=${RISCV64_CC:-riscv64-linux-gnu-gcc}
emulator=${QEMU_RISCV64:-qemu-riscv64}
workspace=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
binary=${TMPDIR:-/tmp}/atlas-rv64-lp64-abi-$$
trap 'rm -f "$binary"' EXIT

command -v "$compiler" >/dev/null
command -v "$emulator" >/dev/null

"$compiler" -march=rv64gc -mabi=lp64d -static \
  "$workspace/crates/atlas-mir/tests/rv64_lp64_abi.c" -o "$binary"
file "$binary" | grep -q 'ELF 64-bit LSB executable, UCB RISC-V'
"$emulator" "$binary" | grep -qx 'rv64-lp64:8:1122334455667788'
