#!/usr/bin/env bash
set -euo pipefail

mkdir -p logs
LOG_FILE="logs/m4-layer-c-qemu.log"
TIMEOUT_SEC="${LAYER_C_TIMEOUT_SEC:-20}"
STATUS_FILE="logs/m4-layer-c-qemu.status"

python3 - <<'PY'
import os
import subprocess
from pathlib import Path

log_path = Path("logs/m4-layer-c-qemu.log")
status_path = Path("logs/m4-layer-c-qemu.status")
timeout_sec = int(os.environ.get("LAYER_C_TIMEOUT_SEC", "20"))

with log_path.open("w", encoding="utf-8") as f:
    timed_out = False
    return_code = 0
    try:
        completed = subprocess.run(
            ["scripts/run-qemu.sh"],
            stdout=f,
            stderr=subprocess.STDOUT,
            timeout=timeout_sec,
            check=False,
        )
        return_code = completed.returncode
    except subprocess.TimeoutExpired:
        timed_out = True

status_path.write_text(
        f"timeout_sec={timeout_sec}\ntimed_out={str(timed_out).lower()}\nreturn_code={return_code}\n",
        encoding="utf-8",
    )
PY

cat "$STATUS_FILE"
if grep -q "timed_out=true" "$STATUS_FILE"; then
  echo "layer-c: qemu run reached timeout after ${TIMEOUT_SEC}s (expected for non-interactive smoke)."
fi

grep -q "m2b:timer_ticks" "$LOG_FILE"
grep -q "m3:demo ring0_to_ring3=true returned_to_ring0=true" "$LOG_FILE"
grep -q "m4:abi read_ok=true brk_ok=true mmap_ok=true munmap_ok=true uname_ok=true getpid_ok=true" "$LOG_FILE"

echo "LAYER_C PASS"
