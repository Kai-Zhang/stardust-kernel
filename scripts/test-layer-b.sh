#!/usr/bin/env bash
set -euo pipefail

cargo test -p stardust-kernel --lib
scripts/build.sh

echo "LAYER_B PASS"
