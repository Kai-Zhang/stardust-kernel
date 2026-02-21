#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
IMAGE_PATH="$ROOT_DIR/target/m1-boot.img"

command -v qemu-system-x86_64 >/dev/null || { echo "qemu-system-x86_64 not found"; exit 1; }

if [[ ! -f "$IMAGE_PATH" ]]; then
  echo "Boot image not found: $IMAGE_PATH"
  echo "Run scripts/build.sh first."
  exit 1
fi

find_ovmf_code() {
  local candidates=(
    "${OVMF_CODE:-}"
    "/opt/homebrew/share/qemu/edk2-x86_64-code.fd"
    "/usr/share/OVMF/OVMF_CODE.fd"
    "/usr/share/OVMF/OVMF_CODE_4M.fd"
    "/usr/share/edk2/x64/OVMF_CODE.fd"
  )
  for p in "${candidates[@]}"; do
    [[ -n "$p" && -f "$p" ]] && { echo "$p"; return 0; }
  done
  return 1
}

OVMF_CODE_PATH="$(find_ovmf_code || true)"
if [[ -z "$OVMF_CODE_PATH" ]]; then
  echo "OVMF_CODE firmware not found. Set OVMF_CODE=/path/to/OVMF_CODE.fd"
  exit 1
fi

qemu-system-x86_64 \
  -machine q35,accel=tcg \
  -m 512M \
  -display none \
  -serial stdio \
  -drive if=pflash,format=raw,readonly=on,file="$OVMF_CODE_PATH" \
  -drive format=raw,file="$IMAGE_PATH"
