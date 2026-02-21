# M4 PRD — Syscalls Phase B/C and Documentation Hardening

- Milestone: `M4-syscalls-b-and-doc-hardening`
- Status: Draft
- Owner: Orchestrator + Codex roles
- Depends on: `M3-userspace-and-syscalls-a`

## 1. Problem Statement

M3 established a minimal syscall path but left ABI Phase B/C incomplete. Without Phase B/C behavior and explicit deviation docs, user-space expectations drift and reviewer validation becomes ambiguous.

## 2. Scope

### In Scope

- Implement syscall subset from `docs/abi/linux-syscall-subset.md` Phase B/C:
  - `read`, `brk`, `mmap`, `munmap`, `uname`, `getpid`
- Add ABI conformance unit tests for implemented syscalls.
- Publish known Linux deviation notes for all supported syscalls.
- Add deterministic M4 runtime marker in QEMU smoke flow.

### Out of Scope (Non-Goals)

- Real hardware `syscall/sysret` gate.
- Full virtual memory manager or VFS-backed file APIs.
- Multi-process/thread semantics.
- Full Linux `utsname` memory layout compatibility.

## 3. MVP Definition

Minimal demoable result:

1. Kernel syscall dispatcher handles Phase B/C syscall numbers with Linux-style success/error shape.
2. Unit tests validate success and error behavior for each new syscall.
3. QEMU log contains `m4:abi ...` marker indicating Phase B/C smoke checks passed.
4. ABI doc clearly records known deviations from Linux behavior.

## 4. Functional Requirements

- FR-1: `read` supports `fd=0`, rejects unsupported fds and null buffer.
- FR-2: `brk` returns current break for `addr=0`, updates only within configured range.
- FR-3: `mmap` supports anonymous private mappings and returns synthetic user address.
- FR-4: `munmap` releases valid synthetic mappings and rejects mismatched ranges.
- FR-5: `uname` validates user buffer pointer shape and returns Linux-style status.
- FR-6: `getpid` returns stable per-task identifier.
- FR-7: Unknown syscall keeps `-ENOSYS` behavior.

## 5. Acceptance Criteria (Testable)

- AC-1: Layer A passes on touched scope (`fmt/check/clippy`).
- AC-2: Layer B passes with updated unit tests and build smoke.
- AC-3: Layer C reproduces `m2b:*`, `m3:*`, and `m4:abi ...` markers.
- AC-4: ABI doc lists semantics/error behavior for all supported syscalls.
- AC-5: ABI doc includes explicit known deviations from Linux behavior.

## 6. Demo Plan

1. `cargo fmt --check`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test -p stardust-kernel --lib`
5. `scripts/build.sh`
6. `scripts/test-layer-c.sh` and verify `m4:abi ...` marker

## 7. Risks & Open Questions

- `mmap/munmap` in this milestone intentionally model mapping table behavior and do not represent full VM subsystem semantics.
- `uname` data payload population is deferred; this milestone validates ABI call contract shape first.
