# Linux Syscall Subset (x86_64)

This document defines the initial Linux ABI subset the teaching kernel aims to support for simple user-space programs.

## ABI Baseline

- Architecture: `x86_64`
- Calling convention: Linux syscall convention (`rax` syscall number, args in `rdi`, `rsi`, `rdx`, `r10`, `r8`, `r9`)
- Return value: non-negative success in `rax`, negative errno mapped to `-errno`
- Initial execution model: single process, single thread (milestones may expand)

## Phase Targets

### Phase A: Minimal IO + Process Exit

- `write` (1): support `fd=1` and `fd=2` for console output
- `exit` (60): terminate current task
- `exit_group` (231): alias to `exit` in single-thread model

### Phase B: Memory and Read Path

- `read` (0): support console/serial input in polling mode (`fd=0` only)
- `brk` (12): basic heap boundary management in fixed `[brk_min, brk_max]` window
- `mmap` (9): anonymous private mappings only (`MAP_ANONYMOUS | MAP_PRIVATE`)
- `munmap` (11): unmap previously returned anonymous mapping pairs `(addr, len)`

### Phase C: Filesystem-Independent Introspection

- `uname` (63): return success for non-null output pointer
- `getpid` (39): return stable per-task identifier

## Implemented Semantics and Error Behavior

- `read(fd, buf, count)`
  - success: returns `min(count, available_input)` for `fd=0`
  - errors: `-EBADF` for non-stdin fd, `-EFAULT` for null buffer
- `write(fd, buf, count)`
  - success: returns `count` for `fd=1/2`
  - errors: `-EBADF` for unsupported fd, `-EFAULT` for null buffer
- `brk(addr)`
  - `addr=0`: return current break
  - in-range addr: set + return new break
  - out-of-range addr: keep current break and return unchanged break
- `mmap(addr, len, prot, flags, fd, off)`
  - success: returns synthetic mapped address for anonymous private mappings
  - errors: `-EINVAL` for unsupported flags/length or exhausted mapping slots
- `munmap(addr, len)`
  - success: `0` only when `(addr, len)` exactly matches a live mapping
  - errors: `-EINVAL` on mismatch or invalid inputs
- `uname(buf)`
  - success: `0` for non-null pointer
  - errors: `-EFAULT` for null pointer
- `getpid()`
  - success: stable fixed pid in single-task model

## Known Deviations from Linux Behavior (Teaching Kernel)

- Ring transition and syscall handling are modeled in Rust control flow, not hardware `syscall/sysret`.
- `read` does not access a real TTY buffer; it reports synthetic available input.
- `brk` uses a fixed sandboxed range, no real VMA merge/split logic.
- `mmap` ignores `addr/prot/fd/off` details and only accepts anonymous private mappings.
- `munmap` requires exact `(addr, len)` match and does not support partial unmap.
- `uname` currently validates pointer shape but does not populate Linux-identical utsname bytes.
- `getpid` returns a fixed stable pid (`42`) in single-task runtime.

## Explicitly Unsupported (Initial)

- `fork`, `clone`, `execve`
- Signals
- Sockets
- Full VFS and path-based file APIs

## Compatibility Notes

- Behavior aims to be Linux-like where practical, but this is a teaching kernel.
- Any intentional deviation must be documented here before implementation lands.
