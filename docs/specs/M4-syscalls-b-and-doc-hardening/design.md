# M4 Design — Syscalls Phase B/C and Documentation Hardening

- Milestone: `M4-syscalls-b-and-doc-hardening`
- Status: Draft
- Linked PRD: `docs/specs/M4-syscalls-b-and-doc-hardening/prd.md`

## 1. Context and Goals

M4 extends the teaching syscall dispatcher from Phase A into Phase B/C while keeping scope constrained and auditable.

Goals:

- implement Linux-style return contract for `read`, `brk`, `mmap`, `munmap`, `uname`, `getpid`;
- keep behavior deterministic for unit tests and QEMU smoke;
- harden docs by making deviations explicit in ABI spec.

## 2. Interfaces and Contracts

Module: `kernel/src/userspace.rs`

Public API additions:

- `run_m4_abi_smoke() -> M4AbiSmokeReport`

Key internal contract changes:

- syscall dispatcher now handles Linux syscall numbers for Phase B/C.
- `KernelState` stores synthetic ABI state:
  - read accounting and available input bytes,
  - break (`brk_min`, `brk_cur`, `brk_max`),
  - fixed pid,
  - fixed-size mapping table for `mmap/munmap`.

Boot integration:

- `kernel/src/main.rs` prints marker:
  - `m4:abi read_ok=... brk_ok=... mmap_ok=... munmap_ok=... uname_ok=... getpid_ok=...`

## 3. State Model

Additional state elements:

- `read` path: synthetic input budget read by `read(fd=0)`.
- `brk` path: bounded moving break pointer.
- `mmap` path: linear synthetic address allocator + fixed slot table.
- `munmap` path: exact tuple match removal.
- `getpid` path: stable pid constant.

Invariants:

- syscall return values always follow Linux shape (success >= 0, errors < 0).
- `mmap` only succeeds when `MAP_ANONYMOUS | MAP_PRIVATE` is present.
- `munmap` only succeeds for active exact mappings.

## 4. Failure Modes and Recovery

- invalid fd/pointer in `read`/`write` -> `-EBADF` / `-EFAULT`.
- invalid `mmap` flags or empty length -> `-EINVAL`.
- `munmap` miss or invalid range -> `-EINVAL`.
- unsupported syscall -> `-ENOSYS`.

Recovery strategy in this model:

- errors are returned to user payload; kernel state remains coherent.
- no panic path for expected syscall misuse.

## 5. Rollback Plan

If M4 marker causes boot instability:

1. keep dispatcher + tests in place;
2. temporarily remove M4 marker emission from `kernel/src/main.rs`;
3. restore once baseline boot path is stable.

## 6. Compatibility

- Preserves M1/M2/M2b/M3 markers.
- Keeps Phase A syscall behavior unchanged.
- Aligns docs and runtime behavior in `docs/abi/linux-syscall-subset.md`.

## 7. Architect Commit Phases (Implementation Split)

1. **Phase 1 — Planner docs**
   - Add M4 PRD with MVP, non-goals, and acceptance.
2. **Phase 2 — Dispatcher + state extension**
   - Extend syscall dispatch and kernel state for Phase B/C behavior.
3. **Phase 3 — ABI conformance tests + boot marker**
   - Add unit tests and QEMU marker for M4 ABI smoke.
4. **Phase 4 — Documentation hardening + review evidence**
   - Update ABI deviations, fundamentals note, and reviewer checklist.

## 8. Test Strategy

- Layer A: `fmt`, `check`, `clippy`.
- Layer B: kernel unit tests + build smoke.
- Layer C: QEMU smoke with `m2b`, `m3`, and `m4` markers.
- Layer D: CI workflow run on `main` and release branch (handled by release gate policy).
