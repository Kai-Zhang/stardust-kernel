# Roadmap

This roadmap is formatted for humans and coding agents. Keep milestone state and acceptance criteria current.

## Status Vocabulary

- `planned`: defined, not started
- `active`: currently being implemented
- `blocked`: waiting on dependency or design decision
- `done`: acceptance criteria satisfied

## Milestones

### M0

- id: `M0-workspace-bootstrap`
- status: `done`
- objective: Establish project scaffolding, baseline docs, and contribution workflow.
- deliverables:
  - Directory layout (`kernel/`, `user/`, `tools/`, `scripts/`, `docs/`, `.github/`)
  - Baseline docs (`README`, `CONTRIBUTING`, `AGENTS`, ABI + roadmap)
  - Rust workspace manifests
- dependencies: none
- acceptance:
  - Repository has a clean initial commit
  - New contributors can identify project goals and next tasks in under 10 minutes

### M1

- id: `M1-boot-to-rust`
- status: `done` (Final PASS)
- objective: Boot Rust kernel under QEMU via UEFI and print early boot diagnostics.
- deliverables:
  - UEFI boot path and handoff structures
  - Early console output and panic path
  - Repeatable `scripts/build.sh` and `scripts/run-qemu.sh`
- dependencies:
  - `M0-workspace-bootstrap`
- acceptance:
  - QEMU boot reaches kernel Rust code reliably
  - Boot logs include memory map summary and kernel version string

### M2

- id: `M2-memory-interrupt-foundation`
- status: `planned`
- objective: Introduce memory management and minimal interrupt handling.
- deliverables:
  - Physical frame allocator
  - Basic virtual memory mapping API
  - IDT/GDT/TSS setup with timer interrupt wiring
- dependencies:
  - `M1-boot-to-rust`
- acceptance:
  - Kernel handles periodic timer interrupts without faulting
  - Mapping tests pass in-kernel smoke tests

### M3

- id: `M3-userspace-and-syscalls-a`
- status: `planned`
- objective: Run first user payload and support minimal syscall subset Phase A.
- deliverables:
  - Ring transition path for user execution
  - Syscalls: `write`, `exit`, `exit_group`
  - Demo user binary prints and exits cleanly
- dependencies:
  - `M2-memory-interrupt-foundation`
  - `docs/abi/linux-syscall-subset.md` Phase A contract
- acceptance:
  - Demo user payload runs via syscall interface in QEMU
  - Error paths return Linux-style negative errno values

### M4

- id: `M4-syscalls-b-and-doc-hardening`
- status: `planned`
- objective: Implement syscall subset Phase B/C and tighten documentation/testing loop.
- deliverables:
  - Syscalls from Phase B/C in ABI document
  - ABI conformance tests for implemented syscalls
  - Documented known deviations from Linux behavior
- dependencies:
  - `M3-userspace-and-syscalls-a`
- acceptance:
  - ABI tests pass in CI/emulator pipeline
  - Every supported syscall has documented semantics and error behavior
