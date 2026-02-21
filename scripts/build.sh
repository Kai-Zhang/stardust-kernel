#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TARGET="x86_64-unknown-uefi"
EFI_OUT="$ROOT_DIR/target/$TARGET/debug/teaching-kernel.efi"
IMAGE_DIR="$ROOT_DIR/target/m1-image"
IMAGE_PATH="$ROOT_DIR/target/m1-boot.img"

command -v cargo >/dev/null || { echo "cargo not found"; exit 1; }
command -v mformat >/dev/null || { echo "mtools (mformat) not found"; exit 1; }
command -v mmd >/dev/null || { echo "mtools (mmd) not found"; exit 1; }
command -v mcopy >/dev/null || { echo "mtools (mcopy) not found"; exit 1; }

rustup target add "$TARGET" >/dev/null

cargo build -p teaching-kernel --target "$TARGET"

rm -rf "$IMAGE_DIR"
mkdir -p "$IMAGE_DIR/EFI/BOOT"
cp "$EFI_OUT" "$IMAGE_DIR/EFI/BOOT/BOOTX64.EFI"

rm -f "$IMAGE_PATH"
dd if=/dev/zero of="$IMAGE_PATH" bs=1m count=64 >/dev/null 2>&1
mformat -i "$IMAGE_PATH" -F ::
mmd -i "$IMAGE_PATH" ::/EFI ::/EFI/BOOT
mcopy -i "$IMAGE_PATH" "$IMAGE_DIR/EFI/BOOT/BOOTX64.EFI" ::/EFI/BOOT/BOOTX64.EFI

echo "Built EFI image: $IMAGE_PATH"
echo "Kernel EFI: $EFI_OUT"
