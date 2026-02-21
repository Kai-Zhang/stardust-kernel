# M1 Theory Note: Boot to Rust (UEFI + QEMU + OVMF)

This note explains the current M1 runtime path in plain language, from emulator start to Rust code execution.

Related milestone artifacts:

- PRD: `docs/specs/M1-boot-to-rust/prd.md`
- Design: `docs/specs/M1-boot-to-rust/design.md`
- Review evidence: `docs/reviews/M1-boot-to-rust/review.md`

## 1) What M1 Proves

M1 is about proving a reliable first boot pipeline:

1. Build a UEFI-loadable Rust binary.
2. Boot it in QEMU using OVMF firmware.
3. Reach our Rust entrypoint.
4. Print deterministic boot diagnostics.

In this repository, success is visible as serial logs such as:

- `stardust-kernel <version>`
- `memmap: descriptors=<n> total_pages=<n> conventional_pages=<n>`

See `scripts/README.md` for expected output markers.

## 2) UEFI Fundamentals (Only What We Need for M1)

UEFI is firmware with a standardized API. For M1, only a few parts matter:

1. UEFI firmware can load `.EFI` executables from a FAT filesystem.
2. A default removable-media path exists: `EFI/BOOT/BOOTX64.EFI` on x86_64.
3. When a UEFI app starts, it can use firmware services (console, memory map, status codes).
4. The app returns a UEFI `Status` value to indicate success or failure.

How this maps to our code:

- Build target is `x86_64-unknown-uefi` in `scripts/build.sh`.
- The output binary is copied to `EFI/BOOT/BOOTX64.EFI` inside `target/m1-boot.img`.
- Rust entrypoint is `efi_main` in `kernel/src/main.rs`.
- UEFI memory map is read via `uefi::boot::memory_map(...)` in `kernel/src/main.rs`.

## 3) QEMU + OVMF Boot Flow in This Repo

At a high level:

1. `scripts/build.sh` compiles the Rust kernel EFI binary.
2. `scripts/build.sh` creates a FAT disk image (`target/m1-boot.img`) and places `BOOTX64.EFI` in the UEFI default path.
3. `scripts/run-qemu.sh` starts `qemu-system-x86_64`.
4. QEMU boots OVMF (`OVMF_CODE.fd`) as firmware.
5. OVMF loads `EFI/BOOT/BOOTX64.EFI` from the image.
6. Control enters our Rust UEFI entrypoint.
7. Boot banner and memory summary are printed to serial (`-serial stdio`).

Concrete references:

- Build pipeline: `scripts/build.sh`
- Run pipeline: `scripts/run-qemu.sh`
- Kernel entry + diagnostics: `kernel/src/main.rs`

## 4) How Control Reaches Our Rust Entrypoint

The control handoff chain is:

1. Host shell executes `scripts/run-qemu.sh`.
2. `qemu-system-x86_64` starts with OVMF firmware.
3. OVMF behaves like firmware and searches the EFI boot path.
4. OVMF loads and executes `BOOTX64.EFI`.
5. `BOOTX64.EFI` is the Rust artifact built from `kernel/`.
6. The `#[entry]` function in `kernel/src/main.rs` runs:
   - currently: `fn efi_main() -> Status`
7. `efi_main` initializes UEFI helpers, prints version, reads memory map, prints summary, returns status.

Important teaching point: in M1, this `efi_main` function is effectively our first kernel "main", even though it is a UEFI app entrypoint, not a traditional freestanding kernel start routine yet.

## 5) Why M1 Scope Is Boot + Observability (Not Full Memory Management)

M1 intentionally limits scope to reduce uncertainty and make debugging straightforward.

Why this is the right first milestone:

1. Boot path first: without a reliable boot and entrypoint, deeper kernel work is not verifiable.
2. Observability first: deterministic logs let contributors confirm behavior quickly across machines.
3. Fast feedback: script-driven build/run makes regressions obvious.
4. Controlled complexity: full allocators, paging policy, and interrupt orchestration are deferred to M2 (`docs/milestones/ROADMAP.md`).

What M1 does not try to solve:

- Complete physical/virtual memory management
- Interrupt handling framework
- User mode execution and syscall support

Those are explicit later milestones, not missing pieces in M1.

## 6) Practical Walkthrough (Local)

Run:

```bash
scripts/build.sh
scripts/run-qemu.sh
```

Then confirm:

1. Banner line appears: `stardust-kernel ...`
2. Memory line appears: `memmap: descriptors=...`
3. No immediate boot-path failure message from scripts (missing QEMU/OVMF/image)

If something fails, start from script checks:

- Tool checks in `scripts/build.sh`
- Firmware discovery logic in `scripts/run-qemu.sh`
- Error status branch in `kernel/src/main.rs`

## 7) Mental Model to Keep

For M1, think of the system as:

`QEMU machine` -> `OVMF firmware` -> `UEFI app (our Rust kernel artifact)` -> `serial diagnostics`

If this chain is healthy, M1 is healthy.
