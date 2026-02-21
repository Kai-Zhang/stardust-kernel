# M3 Design — Userspace + Syscalls Phase A

- Milestone: `M3-userspace-and-syscalls-a`
- Status: Draft
- Linked PRD: `docs/specs/M3-userspace-and-syscalls-a/prd.md`

## 1. Context and Goals

M3 introduces the first end-to-end userspace execution loop over a minimal Linux syscall ABI subset.

Goals:

- model ring transition for teaching and deterministic testing;
- support `write`, `exit`, `exit_group` syscall semantics;
- run one demo payload from `user` crate through kernel syscall dispatcher.

## 2. Interfaces and Contracts

Module: `kernel/src/userspace.rs`

Public API:

- `run_demo_program(program: &DemoUserProgram) -> DemoRunReport`
- `run_m3_demo_payload() -> DemoRunReport`

Key contracts:

- ring state progresses `Ring0 -> Ring3 -> Ring0`.
- syscall dispatcher returns Linux-style values:
  - success: non-negative
  - failure: negative `-errno`
- Phase A syscall numbers:
  - `write = 1`
  - `exit = 60`
  - `exit_group = 231`

Shared payload contract:

- `stardust_user::DemoUserProgram`
- `stardust_user::DemoSyscall`
- `stardust_user::DEMO_USER_PROGRAM`

## 3. State Model

States:

- `KernelReady` (Ring0)
- `UserRunning` (Ring3)
- `Terminated` (exit/exit_group observed)
- `KernelReturned` (Ring0)

Transitions:

1. `run_demo_program`: `KernelReady -> UserRunning`
2. `write`: remain in `UserRunning`
3. `exit`/`exit_group`: `UserRunning -> Terminated`
4. function epilogue: `Terminated -> KernelReturned`

Invariants:

- final report must always end at `final_ring = Ring0`.
- `exit_group` equals `exit` in single-task model.

## 4. Failure Modes and Recovery

- unsupported fd in `write`:
  - return `-EBADF`
  - payload may choose to continue or terminate later
- zero length write payload:
  - return `-EFAULT`
- unknown syscall number:
  - return `-ENOSYS`

## 5. Rollback Plan

If M3 boot wiring regresses existing boot path:

1. keep `userspace.rs` + tests in place;
2. temporarily remove M3 marker call from `kernel/src/main.rs`;
3. re-enable integration after baseline recovers.

## 6. Compatibility

- Keeps M1/M2/M2b markers intact.
- Aligns with `docs/abi/linux-syscall-subset.md` Phase A.
- No Phase B/C syscall behavior introduced.

## 7. Architect Commit Phases (Implementation Split)

1. **Phase 1 — Planner docs**
   - Add M3 PRD with MVP, non-goals, acceptance.
2. **Phase 2 — Userspace runtime + syscall dispatcher**
   - Add ring state model and Linux-style errno returns.
3. **Phase 3 — Demo payload path + boot marker**
   - Add shared payload in `user` crate and invoke from kernel boot flow.
4. **Phase 4 — Test/review/fundamentals evidence**
   - Add tests, fundamentals note, review checklist, and layer scripts/evidence.

## 8. Test Strategy

- Layer A: `fmt`, `check`, `clippy`.
- Layer B: `cargo test -p stardust-kernel --lib` + build smoke.
- Layer C: QEMU smoke capture includes `m3:demo ...` marker and M2b marker coexistence.
