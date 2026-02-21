# Teaching Kernel (Rust)

A practical teaching/demo kernel workspace focused on building a small x86_64 kernel in pure Rust, booted through UEFI, with a minimal Linux-compatible syscall layer for user programs.

## Project Goals

- Build a readable, incremental kernel implementation suitable for teaching.
- Stay in pure Rust for kernel and user-space demo components.
- Boot on x86_64 with UEFI in QEMU first, then real hardware later.
- Provide a narrow Linux ABI subset to run simple statically linked demo programs.
- Keep docs, milestones, and contributor workflows explicit for humans and coding agents.

## Non-Goals (Initial Phases)

- Full Linux compatibility.
- Multi-architecture support (x86_64 only at first).
- Production hardening, advanced scheduling, containers, or networking stack parity.
- ABI stability guarantees before milestone M4.

## Architecture Direction

- **Boot path**: UEFI loads kernel image and hands off memory map + boot metadata.
- **Kernel model**: Monolithic teaching kernel with clear subsystem boundaries.
- **Target ISA**: `x86_64`.
- **Execution mode**: Long mode, higher-half kernel (planned).
- **User ABI**: Documented Linux syscall subset (see `docs/abi/linux-syscall-subset.md`).
- **Primary dev platform**: Linux host toolchain + QEMU/OVMF.

## Repository Layout

- `kernel/`: kernel crate(s) and core subsystems.
- `user/`: demo user-space components and ABI-facing libraries.
- `tools/`: host-side build/inspection utilities.
- `scripts/`: automation entry points (build, run, test).
- `docs/`: design docs, ABI references, and milestones.
- `.github/`: repository automation templates/workflows.

## Quick Start (Placeholders)

1. Install Rust nightly with `rust-src` and `llvm-tools`.
2. Install QEMU and OVMF firmware files.
3. Run workspace checks:
   - `cargo check --workspace`
4. Build + run flow will be added in milestone M1:
   - `scripts/build.sh`
   - `scripts/run-qemu.sh`

## Status

This repository is intentionally early-stage. See `docs/milestones/ROADMAP.md` for concrete milestones, acceptance criteria, and sequencing.
