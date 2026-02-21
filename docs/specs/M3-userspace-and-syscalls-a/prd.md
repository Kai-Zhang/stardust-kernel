# M3 PRD — Userspace + Syscalls Phase A

- Milestone: `M3-userspace-and-syscalls-a`
- Status: Final (reviewed 2026-02-21)
- Owner: Orchestrator + Codex roles
- Depends on: `M2-memory-interrupt-foundation`, `M2b-timer-irq-controller-integration`

## 1. Problem Statement

After M2/M2b, the kernel can boot and keep deterministic interrupt progress, but there is no user-mode execution contract and no syscall interface for user payloads.

Without a first ring transition + syscall path, later process and memory ABI milestones cannot be validated incrementally.

## 2. Scope

### In Scope

- Add a teaching-friendly ring transition model from kernel context to user context and back.
- Implement Phase A syscall subset behavior:
  - `write(1)`
  - `exit(60)`
  - `exit_group(231)`
- Provide one demo user payload path that writes to console path and exits cleanly.
- Enforce Linux-style negative errno values for failing syscall paths.

### Out of Scope (Non-Goals)

- Full hardware privilege switching implementation (real CPU `syscall/sysret` path).
- ELF loader, process table, or scheduler.
- Any Phase B/C syscall work (`read`, `brk`, `mmap`, `uname`, etc.).

## 3. MVP Definition

Minimal demoable result:

1. Boot path runs M2b markers and then runs one M3 demo payload through userspace execution flow.
2. Demo payload performs `write` and `exit_group(0)`.
3. Boot log contains deterministic `m3:demo ...` marker showing ring return + bytes written + exit code.

## 4. Functional Requirements

- FR-1: Userspace runtime starts in ring0 model, enters ring3 model, and returns to ring0 after exit.
- FR-2: `write` supports only `fd=1` and `fd=2`; other fds return `-EBADF`.
- FR-3: Invalid write buffer pointer (`buf==NULL`) returns `-EFAULT`.
- FR-4: `exit` and `exit_group` both terminate the single-task runtime in Phase A.
- FR-5: Unknown syscall numbers return `-ENOSYS`.

## 5. Acceptance Criteria (Testable)

- AC-1: Layer A checks pass for touched scope.
- AC-2: Unit tests cover `-EBADF`, `-ENOSYS`, `exit/exit_group` semantics, and ring transition report.
- AC-3: Boot output includes marker:
  - `m3:demo ring0_to_ring3=true returned_to_ring0=true bytes_written=... exit_code=0`
- AC-4: Reviewer baseline (Layer C) reproduces M3 marker and confirms no M2b regression markers.

## 6. Demo Plan

1. `cargo fmt --check`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test -p stardust-kernel --lib`
5. `scripts/build.sh`
6. QEMU smoke run with timeout wrapper and capture `m2b:*` + `m3:*` lines.

## 7. Risks & Open Questions

- This milestone intentionally models ring transition semantics instead of implementing full hardware privilege transfer.
- Real hardware syscall gate and ELF/task loader are deferred to later milestones to keep M3 scope teachable and bounded.
