# Review Report — M1-boot-to-rust

- Reviewer status: **PASS**
- Scope reviewed:
  - `docs/specs/M1-boot-to-rust/prd.md`
  - `docs/specs/M1-boot-to-rust/design.md`
  - `kernel/src/main.rs`
  - `scripts/build.sh`, `scripts/run-qemu.sh`
  - `scripts/README.md`

## Layer C Baseline Evidence

Environment preflight (required):
- `command -v cargo` -> `/Users/kai/.cargo/bin/cargo`
- `cargo --version` -> `cargo 1.95.0-nightly (ce69df6f7 2026-02-12)`
- `command -v qemu-system-x86_64` -> `/opt/homebrew/bin/qemu-system-x86_64`
- `qemu-system-x86_64 --version | head -n 1` -> `QEMU emulator version 10.2.1`

Validation commands and outcomes:
- `cargo check --workspace` -> pass (`Finished 'dev' profile ...`)
- `scripts/build.sh` -> pass
  - `Built EFI image: .../target/m1-boot.img`
  - `Kernel EFI: .../target/x86_64-unknown-uefi/debug/stardust-kernel.efi`
- `scripts/run-qemu.sh` -> boot log captured in 3 consecutive runs:
  - Run #1: `stardust-kernel 0.1.0`; `memmap: descriptors=102 total_pages=3342240 conventional_pages=118161`
  - Run #2: `stardust-kernel 0.1.0`; `memmap: descriptors=101 total_pages=3342240 conventional_pages=118162`
  - Run #3: `stardust-kernel 0.1.0`; `memmap: descriptors=101 total_pages=3342240 conventional_pages=118162`

## Checklist Against Acceptance Criteria

1. QEMU boot reaches kernel Rust code reliably (>=3 runs)
   - Evidence: 3 consecutive run logs captured with kernel banner and memory summary
   - Result: **Pass**

2. Boot logs include kernel version string
   - Evidence: `stardust-kernel 0.1.0` in all 3 runs
   - Result: **Pass**

3. Boot logs include memory map summary
   - Evidence: `memmap: descriptors=... total_pages=... conventional_pages=...` in all 3 runs
   - Result: **Pass**

4. Build/run scripts are documented and reproducible
   - Evidence: `scripts/build.sh`, `scripts/run-qemu.sh`, and `scripts/README.md`; successful local execution
   - Result: **Pass**

## Findings

### Minor

- **m1**: `run-qemu.sh` currently enters firmware UI after kernel returns, so automated runs require external timeout/kill for non-interactive CI.
  - Impact: low (does not affect boot-to-rust acceptance)
  - Suggested follow-up: add optional auto-exit behavior or expected stop condition flag for CI smoke runs.

## Recommendation

- **PASS** for M1 acceptance at Layer C baseline.
- Merge scope: PRD/design + UEFI boot path + early diagnostics + build/run scripts + script docs.
- Residual risk: low, mainly around run-script automation ergonomics rather than functional correctness.

## Accepted Scope / Non-blocking Optimizations

### Accepted scope (verified)

- UEFI boot path reaches Rust kernel entry in QEMU.
- Boot logs include kernel version string.
- Boot logs include memory map summary.
- Reproducible `scripts/build.sh` and `scripts/run-qemu.sh` flow with documented prerequisites.

### Non-blocking optimizations (tracked separately)

- Improve `scripts/run-qemu.sh` auto-exit behavior for non-interactive smoke runs.
- Improve interactive UX defaults and predictable stop conditions for CI/local demo mode split.
