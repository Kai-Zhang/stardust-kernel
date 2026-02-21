#!/usr/bin/env bash
set -euo pipefail

mkdir -p logs
LOG_FILE="logs/m4-layer-c-qemu.log"

python3 - <<'PY'
import subprocess
from pathlib import Path

log_path = Path("logs/m4-layer-c-qemu.log")
with log_path.open("w", encoding="utf-8") as f:
    try:
        subprocess.run(["scripts/run-qemu.sh"], stdout=f, stderr=subprocess.STDOUT, timeout=8, check=False)
    except subprocess.TimeoutExpired:
        pass
PY

grep -q "m2b:timer_ticks" "$LOG_FILE"
grep -q "m3:demo ring0_to_ring3=true returned_to_ring0=true" "$LOG_FILE"
grep -q "m4:abi read_ok=true brk_ok=true mmap_ok=true munmap_ok=true uname_ok=true getpid_ok=true" "$LOG_FILE"

echo "LAYER_C PASS"
