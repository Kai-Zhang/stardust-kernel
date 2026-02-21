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

- `read` (0): support console/serial input in polling mode
- `brk` (12): basic heap boundary management
- `mmap` (9): anonymous private mappings only (limited flags)
- `munmap` (11): unmap valid anonymous mappings

### Phase C: Filesystem-Independent Introspection

- `uname` (63): return fixed kernel identity fields
- `getpid` (39): return stable per-task identifier

## Explicitly Unsupported (Initial)

- `fork`, `clone`, `execve`
- Signals
- Sockets
- Full VFS and path-based file APIs

## Compatibility Notes

- Behavior aims to be Linux-like where practical, but this is a teaching kernel.
- Any intentional deviation must be documented here before implementation lands.
