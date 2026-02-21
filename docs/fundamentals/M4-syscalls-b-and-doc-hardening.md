# M4 Theory Note: Syscalls Phase B/C and Documentation Hardening

- Related milestone id: `M4-syscalls-b-and-doc-hardening`
- Links:
  - `docs/specs/M4-syscalls-b-and-doc-hardening/prd.md`
  - `docs/specs/M4-syscalls-b-and-doc-hardening/design.md`
  - `docs/reviews/M4-syscalls-b-and-doc-hardening/review.md`

## 1) What This Milestone Proves

M4 proves that the teaching kernel can expose a broader Linux-like syscall ABI surface beyond Phase A and verify that behavior with deterministic checks.

The milestone is successful when:

- Phase B/C syscalls return expected Linux-style values,
- ABI conformance tests pass,
- runtime log includes `m4:abi ...` marker,
- deviations from Linux are explicitly documented.

## 2) Required Concepts (Minimal Theory)

1. **Contract-first syscall behavior**
   - Syscalls are validated by return shape and deterministic state updates.
   - File: `kernel/src/userspace.rs`

2. **Synthetic VM model for teaching**
   - `brk` and `mmap` are implemented with bounded/simple state, not full kernel VM.
   - Files: `kernel/src/userspace.rs`, `docs/abi/linux-syscall-subset.md`

3. **Documentation hardening loop**
   - Runtime behavior and known deviations are captured in ABI docs during implementation, not after.
   - File: `docs/abi/linux-syscall-subset.md`

## 3) Execution Path (Step by Step)

1. Build image with `scripts/build.sh`.
2. Run `scripts/run-qemu.sh`.
3. `kernel/src/main.rs` completes M2b setup and M3 demo.
4. `userspace::run_m4_abi_smoke()` executes representative Phase B/C syscall checks.
5. Kernel prints `m4:abi ...` marker with boolean results per syscall family.

## 4) Scope Boundaries (In Scope vs Deferred)

Included now:

- `read`, `brk`, `mmap`, `munmap`, `uname`, `getpid` teaching behavior,
- ABI conformance tests,
- explicit Linux deviation documentation.

Deferred intentionally:

- full VMA manager and partial unmap logic,
- hardware syscall gate and process model,
- byte-level Linux `utsname` structure population.

## 5) Observability and Validation

Commands:

- `cargo test -p stardust-kernel --lib`
- `scripts/test-layer-b.sh`
- `scripts/test-layer-c.sh`

Expected markers:

- `m2b:timer_ticks ...`
- `m3:demo ...`
- `m4:abi read_ok=true brk_ok=true mmap_ok=true munmap_ok=true uname_ok=true getpid_ok=true`
