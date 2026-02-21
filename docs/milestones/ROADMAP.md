# Roadmap

This roadmap is formatted for humans and coding agents. Keep milestone state and acceptance criteria current.

## Status Vocabulary

- `planned`: defined, not started
- `active`: currently being implemented
- `blocked`: waiting on dependency or design decision
- `done`: acceptance criteria satisfied

## Completion Evidence Standard

A milestone can be marked `done` only when all required evidence is linked in docs/reviews:

- Layer C reviewer baseline evidence
- Layer D passing run URL on `main`
- Layer D passing run URL on corresponding `release/*` branch

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
- status: `done` (Final PASS)
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

### M2b

- id: `M2b-timer-irq-controller-integration`
- status: `done` (Final PASS)
- objective: Complete hardware-level periodic timer IRQ controller integration on top of M2 foundations.
- deliverables:
  - Programmable timer source setup and frequency control
  - IRQ controller routing/acknowledge path for timer interrupts
  - Observable periodic tick behavior with deterministic counters/logs
- dependencies:
  - `M2-memory-interrupt-foundation`
- acceptance:
  - Configurable periodic timer ticks are observable in QEMU logs
  - IRQ acknowledge path is validated for repeated interrupts
  - No spurious fault during sustained timer interrupt load (smoke scenario)

### M3

- id: `M3-userspace-and-syscalls-a`
- status: `done` (Final PASS)
- objective: Run first user payload and support minimal syscall subset Phase A.
- deliverables:
  - Ring transition path for user execution
  - Syscalls: `write`, `exit`, `exit_group`
  - Demo user binary prints and exits cleanly
- dependencies:
  - `M2-memory-interrupt-foundation`
  - `M2b-timer-irq-controller-integration`
  - `docs/abi/linux-syscall-subset.md` Phase A contract
- acceptance:
  - Demo user payload runs via syscall interface in QEMU
  - Error paths return Linux-style negative errno values

### M4

- id: `M4-syscalls-b-and-doc-hardening`
- status: `done` (Final PASS)
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

### M5

- id: `M5-busybox-l2-foundation`
- status: `planned`
- objective: Bridge from current syscall subset to an L2 target capable of running a BusyBox subset in QEMU.
- dependencies:
  - `M4-syscalls-b-and-doc-hardening`
- sub-milestones:
  - `M5-I1-userspace-file-minimal-loop`
    - objective: first end-to-end file path (`openat`/`read`/`close`/`fstat`) on read-only rootfs
    - demo: user program reads `/etc/motd` and prints to stdout
    - acceptance: Layer B smoke script passes 3 consecutive QEMU runs
  - `M5-I2-busybox-minimal-command-set`
    - objective: run minimal BusyBox subset commands in QEMU
    - demo: stable command group (`echo`, `cat`, `ls` subset)
    - acceptance: compatibility matrix + Layer C reproducible smoke pass
  - `M5-I3-l2-consolidation-and-release-gate`
    - objective: close ABI/documentation gaps and reach M5 release quality
    - demo: one script from boot to BusyBox subset command sequence
    - acceptance: Layer B/C/D pass + review evidence complete (`main` + `release/*` URLs)

### M6

- id: `M6-smp-and-simd-foundation`
- status: `planned`
- objective: Introduce modern hardware capabilities in a teaching-friendly, switchable way.
- dependencies:
  - `M5-busybox-l2-foundation`
- sub-milestones:
  - `M6-I1-smp-boot-bringup`
    - objective: AP bring-up with deterministic markers under `CONFIG_SMP`
    - demo: boot logs show BSP/AP initialization and online CPU count
    - acceptance: single-core and SMP boot paths both pass Layer B smoke
  - `M6-I2-multicore-scheduling-and-sync-baseline`
    - objective: basic multicore scheduling/synchronization primitives for demo workload
    - demo: deterministic multicore counter/work-queue scenario
    - acceptance: no deadlock/race in repeated QEMU smoke runs; documented invariants
  - `M6-I3-simd-context-baseline`
    - objective: SIMD save/restore baseline under `CONFIG_SIMD`
    - demo: user/kernel transition with SIMD workload and integrity checks
    - acceptance: SIMD smoke passes without regressing syscall/demo path; docs updated

### M7

- id: `M7-hardware-bringup-and-reproducibility`
- status: `planned`
- objective: Move from QEMU-only validation to reproducible real-hardware bring-up.
- dependencies:
  - `M6-smp-and-simd-foundation`
- sub-milestones:
  - `M7-I1-target-hardware-profile-and-boot-checklist`
    - objective: lock target machine profile + reproducible bring-up checklist
    - demo: documented cold-boot checklist from firmware to kernel markers
    - acceptance: checklist reproducible by reviewer on same hardware profile
  - `M7-I2-hardware-smoke-and-parity`
    - objective: align real-hardware smoke with QEMU baseline behavior
    - demo: same command/script path runs on QEMU and real machine
    - acceptance: parity report with known deltas + hardware run logs in Layer C evidence
  - `M7-I3-release-hardening-and-teaching-packaging`
    - objective: package stable teaching release with hardware troubleshooting guidance
    - demo: release script + troubleshooting doc walk-through
    - acceptance: Layer D + review evidence complete on `main` and `release/*`
