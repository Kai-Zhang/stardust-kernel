# M1 PRD — Boot to Rust

- Milestone: `M1-boot-to-rust`
- Status: Draft
- Owner: Orchestrator + Codex roles

## Problem Statement

The repository currently has no executable boot path. Contributors cannot validate kernel startup behavior in QEMU or observe early diagnostics.

## Objective

Boot Rust kernel code under QEMU via UEFI and print deterministic early boot diagnostics.

## MVP Scope

1. Build a UEFI-loadable kernel artifact from Rust code.
2. Provide a stable entrypoint that executes Rust startup code.
3. Print early diagnostics to console:
   - kernel version string
   - memory map summary (total descriptors + selected aggregate stats)
4. Provide repeatable scripts:
   - `scripts/build.sh`
   - `scripts/run-qemu.sh`

## Non-Goals

- Interrupts, scheduler, virtual memory manager, user mode transitions.
- Linux syscall implementation.
- Real hardware support beyond QEMU + OVMF demo target.

## Users / Stakeholders

- Kernel learners who need a deterministic first boot demo.
- Contributors who need a reproducible local dev workflow.

## Functional Requirements

- FR1: `scripts/build.sh` produces a bootable artifact for QEMU UEFI boot.
- FR2: `scripts/run-qemu.sh` launches QEMU with OVMF and boots the artifact.
- FR3: On boot, Rust code prints:
  - startup banner with kernel version
  - memory map summary line(s)
- FR4: Panic path prints a panic message and halts.

## Acceptance Criteria

- AC1: QEMU boot reaches Rust kernel entrypoint reliably (>=3 consecutive runs).
- AC2: Boot log includes kernel version string.
- AC3: Boot log includes memory map summary.
- AC4: Build and run scripts are documented and runnable on a standard dev host with QEMU+OVMF installed.

## Quality Gates

- Implementer done gate: Layer A + Layer B pass.
- Reviewer precondition: Layer C pass.

## Risks

- UEFI crate API/version mismatch on chosen toolchain.
- OVMF firmware path differences across hosts.
- QEMU serial/log capture differences.

## Open Questions

1. Should the kernel artifact be a UEFI app in M1, or do we require an explicit handoff stage now?
   - Proposed M1 decision: use direct UEFI kernel app now; refine handoff structure in M2.
2. Which host OS baseline should scripts target first?
   - Proposed M1 decision: macOS/Linux with environment-variable firmware path override.
