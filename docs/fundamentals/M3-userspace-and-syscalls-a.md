# M3 Theory Note: Userspace and Syscalls Phase A

- Related milestone id: `M3-userspace-and-syscalls-a`
- Links:
  - `docs/specs/M3-userspace-and-syscalls-a/prd.md`
  - `docs/specs/M3-userspace-and-syscalls-a/design.md`
  - `docs/reviews/M3-userspace-and-syscalls-a/review.md`

## 1) What This Milestone Proves

M3 proves that the kernel can run a first user payload through a minimal syscall surface instead of only kernel-side initialization logic.

A run is considered successful when logs show both the M2b timer marker and an M3 marker that confirms:

- transition from ring0 model to ring3 model,
- syscall-driven write progress,
- syscall-driven exit and return to ring0.

This provides the minimum ABI bridge needed before adding loader, memory syscalls, and process abstractions.

## 2) Required Concepts (Minimal Theory)

1. **Ring model (teaching abstraction)**
   - In this milestone we model ring transitions as explicit states (`Ring0`, `Ring3`) in Rust.
   - File: `kernel/src/userspace.rs`
   - Why it matters: we can verify control flow before implementing hardware privilege transition details.

2. **Linux syscall return convention**
   - Syscalls return non-negative success or negative errno (`-EBADF`, `-ENOSYS`, etc.).
   - Files: `kernel/src/userspace.rs`, `docs/abi/linux-syscall-subset.md`
   - Why it matters: user ABI contracts remain compatible with Linux-style expectations.

3. **Demo payload path**
   - Demo payload syscall sequence is defined in the `user` crate and consumed by kernel runtime.
   - Files: `user/src/lib.rs`, `kernel/src/userspace.rs`
   - Why it matters: this is the first cross-crate userspace-to-kernel behavior path.

## 3) Execution Path (Step by Step)

1. Build image with `scripts/build.sh`.
2. Run kernel via `scripts/run-qemu.sh`.
3. UEFI enters Rust entry at `kernel/src/main.rs` (`efi_main`).
4. Kernel completes M2b setup and logs timer marker.
5. `main.rs` calls `userspace::run_m3_demo_payload()`.
6. Runtime in `kernel/src/userspace.rs` enters `Ring3`, executes payload syscalls from `user/src/lib.rs`.
7. `write` updates write count; `exit_group(0)` terminates payload.
8. Runtime returns to `Ring0` and prints `m3:demo ...` marker.

## 4) Scope Boundaries (In Scope vs Deferred)

Included now:

- ring transition state model,
- syscall subset A (`write`, `exit`, `exit_group`),
- one deterministic demo payload path.

Deferred intentionally:

- real `syscall/sysret` hardware transition,
- ELF loader and process table,
- Phase B/C syscalls from `docs/milestones/ROADMAP.md` M4 direction.

Deferral keeps M3 focused on ABI control flow and testability.

## 5) Observability and Validation

Commands:

- `cargo fmt --check`
- `cargo check --workspace`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test -p stardust-kernel --lib`
- `scripts/build.sh`
- run QEMU smoke and grep markers (`m2b:` and `m3:`)

Expected markers:

- `m2b:timer_ticks ...`
- `m3:demo ring0_to_ring3=true returned_to_ring0=true bytes_written=... exit_code=0`

Common failures:

- missing ring return marker -> inspect `kernel/src/userspace.rs`
- syscall mismatch with ABI doc -> inspect `docs/abi/linux-syscall-subset.md`
- boot marker missing -> inspect `kernel/src/main.rs`

## 6) Mapping Table: Concept -> File

| Concept | File(s) |
| --- | --- |
| Ring transition model | `kernel/src/userspace.rs` |
| Syscall Phase A contract | `docs/abi/linux-syscall-subset.md` |
| Demo user payload | `user/src/lib.rs` |
| Boot integration marker | `kernel/src/main.rs` |
| Reviewer evidence | `docs/reviews/M3-userspace-and-syscalls-a/review.md` |
