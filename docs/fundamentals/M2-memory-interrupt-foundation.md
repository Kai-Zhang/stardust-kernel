# M2 Theory Note: Memory + Interrupt Foundation

This note explains what M2 adds beyond M1 and why these pieces are intentionally minimal.

Related artifacts:

- PRD: `docs/specs/M2-memory-interrupt-foundation/prd.md`
- Design: `docs/specs/M2-memory-interrupt-foundation/design.md`
- Review: `docs/reviews/M2-memory-interrupt-foundation/review.md`

## 1) What M2 Proves

M2 proves that the kernel has the smallest useful contracts for three core areas:

1. Physical frame tracking from boot memory map data.
2. Virtual map API with explicit map/unmap/translate behavior.
3. Interrupt foundation readiness with deterministic timer tick accounting.

In this repository, these are surfaced by boot markers:

- `m2:frames ...`
- `m2:vmap ...`
- `m2:interrupts ... timer_ticks=...`

## 2) Physical Frames in M2

`kernel/src/memory.rs` converts memory-map regions into page-sized frames and tracks allocation state.

Teaching model:

- only conventional memory is allocatable,
- all frames are page-granular,
- allocator state is explicit (`free`, `allocated`, `tracked_total`).

This is not a full production allocator yet; it is a deterministic foundation for later paging/process work.

## 3) Virtual Mapping Contract in M2

`VirtualMapper` in `kernel/src/memory.rs` provides a minimal contract:

- map page -> frame (+ flags)
- reject duplicate map on same page
- unmap returns mapped frame
- translate queries current mapping

Why this matters now:

- later milestones need stable map semantics before introducing architecture-specific page-table complexity.

## 4) Interrupt Foundation in M2

`kernel/src/interrupts.rs` provides a foundation API for interrupt table readiness and timer tick accounting:

- `init_foundation()` marks GDT/TSS/IDT readiness for this milestone scope,
- `record_timer_tick()` increments a monotonic counter,
- `timer_ticks()` exposes the current counter.

For M2, the emphasis is deterministic kernel-side interrupt state plumbing, not full controller-level production IRQ integration.

## 5) Runtime Walkthrough

At boot (`kernel/src/main.rs`):

1. M1 banner + memory summary are printed.
2. Memory regions are converted for frame allocator initialization.
3. A one-page map/unmap smoke path validates mapping API behavior.
4. Interrupt foundation is initialized and timer ticks are recorded.
5. M2 markers are printed for smoke/reviewer verification.

## 6) Mental Model

For M2, think in layers:

- **Boot evidence** (visible markers)
- **Deterministic state contracts** (allocator/mapper/interrupt foundation)
- **Future extensibility** (paging/IRQ/user-mode milestones)

M2 is complete when these contracts are stable, testable, and demonstrably wired in boot output.
