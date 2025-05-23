#!/usr/bin/env bash
#
# Link wrapper for RISC-Zero guest via clang+lld

forward=()

for arg in "$@"; do
  case "$arg" in
    # drop all these
    --fatal-warnings|\
    -flavor|\
    gnu|\
    --as-needed|\
    --gc-sections|\
    -z|\
    noexecstack|\
    -nodefaultlibs|\
    -Wl,*|\
    -Ttext=* )
      # skip it
      ;;
    *)
      forward+=("$arg")
      ;;
  esac
done

exec clang \
  --target=riscv32-unknown-elf \
  -march=rv32im -mabi=ilp32 \
  -nostdlib -nostartfiles \
  -Wl,-Ttext=0x00200800 \
  "${forward[@]}"