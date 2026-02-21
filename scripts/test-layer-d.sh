#!/usr/bin/env bash
set -euo pipefail

scripts/test-layer-b.sh
scripts/test-layer-c.sh

echo "LAYER_D PASS"
