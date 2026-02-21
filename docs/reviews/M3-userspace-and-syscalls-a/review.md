# Review Report — M3-userspace-and-syscalls-a

- Reviewer status: **PASS**
- Scope reviewed:
  - `docs/specs/M3-userspace-and-syscalls-a/prd.md`
  - `docs/specs/M3-userspace-and-syscalls-a/design.md`
  - `docs/fundamentals/M3-userspace-and-syscalls-a.md`
  - `kernel/src/userspace.rs`
  - `kernel/src/main.rs`
  - `user/src/lib.rs`

## Layer C Baseline Evidence

Validation commands and outcomes (2026-02-21):

- `cargo fmt --check` -> pass
- `cargo check --workspace` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test -p stardust-kernel --lib` -> pass
- `scripts/build.sh` -> pass
- QEMU smoke timeout wrapper -> pass, markers captured

Captured runtime markers:

- `m2b:timer_ticks hz=100 vector=32 ticks=8 acks=8`
- `m3:demo ring0_to_ring3=true returned_to_ring0=true bytes_written=24 exit_code=0`

## Checklist Against Acceptance Criteria

1. Layer A checks pass
   - Result: **Pass**
2. Unit tests cover errno/error path + exit behavior + ring transition report
   - Result: **Pass**
3. M3 marker visible in boot output
   - Result: **Pass**
4. Reviewer reproducibility (Layer C)
   - Result: **Pass**

## Findings / Constraints

- M3 ring transition is intentionally a teaching model, not hardware `syscall/sysret` implementation.
- This constraint is documented in PRD/design/fundamentals and does not block milestone Phase A goals.

## Layer D Release Gate Evidence

- Main branch PASS: https://github.com/Kai-Zhang/stardust-kernel/actions/runs/22254292571
- Release branch PASS (`release/m3-userspace-and-syscalls-a`): https://github.com/Kai-Zhang/stardust-kernel/actions/runs/22254311214

## Recommendation

- **Final PASS** for M3 Phase A scope: ring transition model, syscall subset (`write`, `exit`, `exit_group`), and demo payload path.
- Release gate complete with both required Layer D PASS URLs.
