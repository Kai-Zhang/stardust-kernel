# Review Report — M4-syscalls-b-and-doc-hardening

- Reviewer status: **PASS**
- Scope reviewed:
  - `docs/specs/M4-syscalls-b-and-doc-hardening/prd.md`
  - `docs/specs/M4-syscalls-b-and-doc-hardening/design.md`
  - `docs/fundamentals/M4-syscalls-b-and-doc-hardening.md`
  - `docs/abi/linux-syscall-subset.md`
  - `kernel/src/userspace.rs`
  - `kernel/src/main.rs`
  - `scripts/test-layer-c.sh`

## Layer C Baseline Evidence

Validation commands and outcomes (2026-02-21):

- `cargo fmt --check` -> pass
- `cargo check --workspace` -> pass
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass
- `cargo test -p stardust-kernel --lib` -> pass
- `scripts/build.sh` -> pass
- `scripts/test-layer-c.sh` -> pass

Captured runtime markers:

- `m2b:timer_ticks hz=100 vector=32 ticks=8 acks=8`
- `m3:demo ring0_to_ring3=true returned_to_ring0=true bytes_written=24 exit_code=0`
- `m4:abi read_ok=true brk_ok=true mmap_ok=true munmap_ok=true uname_ok=true getpid_ok=true`

## Checklist Against Acceptance Criteria

1. Layer A checks pass
   - Result: **Pass**
2. Layer B integration checks pass
   - Result: **Pass**
3. Layer C reproduces M4 ABI marker and prior milestone markers
   - Result: **Pass**
4. Every supported syscall has documented semantics and error behavior
   - Result: **Pass**
5. Known deviations from Linux behavior are documented
   - Result: **Pass**

## Findings / Constraints

- `uname` currently validates pointer contract but does not fill Linux-identical `utsname` bytes.
- `mmap/munmap` uses a synthetic fixed-size mapping table and exact tuple unmap contract.
- These constraints are documented and accepted for teaching scope.

## Layer D Release Gate Evidence

- Main branch PASS: **BLOCKED (requires CI run URL)**
- Release branch PASS (`release/m4-syscalls-b-and-doc-hardening`): **BLOCKED (requires CI run URL)**

## Recommendation

- **Local Final PASS (Layers A/B/C)** for M4 scoped implementation.
- **Milestone completion blocked** until Layer D pass URLs are attached per release gate policy.
