#!/usr/bin/env bash
set -euo pipefail

cargo test -p stardust-kernel --lib
rustup target add x86_64-unknown-uefi >/dev/null
cargo clippy -p stardust-kernel --target x86_64-unknown-uefi --bin stardust-kernel -- -D warnings
scripts/build.sh

echo "LAYER_B PASS"
