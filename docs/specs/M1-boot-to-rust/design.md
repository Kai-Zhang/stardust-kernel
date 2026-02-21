# M1 Design — Boot to Rust

- Milestone: `M1-boot-to-rust`
- Status: Draft
- Depends on: `M0-workspace-bootstrap`

## 1. Architecture Overview

M1 uses a **direct UEFI application kernel** approach:

1. QEMU loads OVMF firmware.
2. OVMF loads `BOOTX64.EFI` from a FAT disk image.
3. `BOOTX64.EFI` is our Rust kernel binary (`x86_64-unknown-uefi`).
4. Kernel entrypoint prints boot diagnostics and halts.

This keeps the boot path minimal while validating Rust execution and diagnostics.

## 2. Components

### 2.1 Kernel crate (`kernel/`)

- Build target: `x86_64-unknown-uefi`
- Entry API: `#[entry] fn efi_main(...) -> Status`
- Responsibilities:
  - initialize UEFI services facade
  - print version banner
  - read UEFI memory map and summarize descriptors
  - provide panic path with visible message and halt loop

### 2.2 Build script (`scripts/build.sh`)

- Validate required tools (`cargo`, `qemu-img`, optional firmware path hint).
- Build kernel EFI artifact.
- Create FAT image layout with `EFI/BOOT/BOOTX64.EFI`.
- Emit artifact paths for run script.

### 2.3 Run script (`scripts/run-qemu.sh`)

- Resolve OVMF firmware path:
  - `OVMF_CODE`/`OVMF_VARS` env override first
  - fallback common system paths
- Launch QEMU x86_64 with UEFI firmware and FAT image.
- Route output to terminal (`-serial stdio`) for deterministic logs.

## 3. Data Structures / Interfaces

### 3.1 Boot diagnostics

- `const KERNEL_VERSION: &str`
- `struct MemorySummary { descriptor_count: usize, conventional_pages: u64, total_pages: u64 }`

### 3.2 Internal interfaces

- `fn collect_memory_summary(...) -> MemorySummary`
- `fn print_boot_banner(...)`
- `fn print_memory_summary(...)`

## 4. Failure Modes & Handling

- Firmware not found:
  - run script exits with actionable guidance.
- QEMU missing:
  - run script exits with install hint.
- UEFI memory-map retrieval fails:
  - print failure reason and return non-success status.
- Panic in kernel:
  - panic handler prints message; halt loop.

## 5. Compatibility / Constraints

- Initial validation target: x86_64 + QEMU + OVMF.
- No stable ABI guarantees yet.
- No cross-arch support in M1.

## 6. Rollback Plan

If UEFI integration blocks progress:
- keep docs/specs + script scaffolding,
- downgrade to a minimal boot-banner-only variant,
- track memory-map requirement as explicit blocker (must be resolved before M1 close).

## 7. Commit-sized Implementation Plan

1. **Step 1 (docs/chore):** Add M1 PRD/design and script usage docs.
2. **Step 2 (feat/kernel):** Implement UEFI Rust entrypoint + banner + panic path.
3. **Step 3 (feat/kernel):** Add memory-map summary output.
4. **Step 4 (feat/scripts):** Add build script producing FAT image with BOOTX64.EFI.
5. **Step 5 (feat/scripts):** Add QEMU run script with OVMF detection and serial logs.
6. **Step 6 (test/docs):** Add smoke-check instructions and expected log snippets.

## 8. Test Plan by Layer

- **Layer A (fast local correctness):**
  - `cargo check --workspace`
  - shellcheck-style syntax check: `bash -n scripts/build.sh scripts/run-qemu.sh`
- **Layer B (local integration/smoke):**
  - `scripts/build.sh`
  - boot once via `scripts/run-qemu.sh` and verify expected banner + memory summary
- **Layer C (reviewer baseline):**
  - clean run on reviewer machine with same commands and captured logs.

## 9. Reviewer Checklist Seed

- Entrypoint actually executes in Rust code path.
- Version string visible in logs.
- Memory map summary visible in logs.
- Panic path message behavior documented.
- Scripts are reproducible and include dependency guidance.
