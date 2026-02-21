# M2 Design — Memory + Interrupt Foundation

- Milestone: `M2-memory-interrupt-foundation`
- Status: Draft
- Linked PRD: `docs/specs/M2-memory-interrupt-foundation/prd.md`

## 1. Context and Goals

M2 introduces foundation-only kernel subsystems required by later userspace/syscall milestones:

- physical frame tracking,
- explicit virtual map contract,
- interrupt descriptor setup skeleton with timer tick hook.

Design goals:

- keep state explicit and deterministic;
- keep API small and testable;
- maintain M1 boot flow and output markers.

## 2. Interfaces and Contracts

### 2.1 Physical Frames

- `FrameAllocator::from_memory_map(entries)`
- `allocate_frame() -> Option<PhysFrame>`
- `free_frame(frame) -> bool`
- `stats() -> FrameStats`

Contract:

- allocator ingests only conventional memory regions;
- allocation is deterministic FIFO-by-normalized-frame-order;
- double-free/out-of-pool free returns `false` (no panic in release path).

### 2.2 Virtual Mapping API (Foundation)

- `VirtualMapper::map(page, frame, flags) -> Result<(), MapError>`
- `VirtualMapper::unmap(page) -> Result<PhysFrame, MapError>`
- `VirtualMapper::translate(page) -> Option<MapEntry>`

Contract:

- one page maps to at most one frame;
- map conflict returns `MapError::AlreadyMapped`;
- unmap missing page returns `MapError::NotMapped`.

### 2.3 Interrupt Foundation

- `interrupts::init_foundation() -> InterruptInitSummary`
- `interrupts::record_timer_tick()`
- `interrupts::timer_ticks() -> u64`

Contract:

- GDT/TSS/IDT setup path is explicit and idempotent for single-core demo flow;
- timer tick counter is monotonic;
- boot path can emit deterministic readiness marker even when external periodic source is unavailable.

ABI implications:

- none yet for Linux syscall ABI surface.
- keeps groundwork for future user-mode interrupt/syscall entry contracts.

## 3. State Model

### 3.1 Frame Allocator

States:

- `Uninitialized` -> `Ready(pool)`

Invariants:

- `free + allocated == tracked_total`
- all tracked frames are unique and page-aligned

### 3.2 Virtual Mapper

States:

- `Empty` -> `MappedSet`

Transitions:

- `map` inserts unique page key
- `unmap` removes existing key

Invariant:

- page key uniqueness; no duplicate map.

### 3.3 Interrupt Foundation

States:

- `Cold` -> `TablesReady` -> `Ticking`

Transitions:

- `init_foundation` moves `Cold` to `TablesReady`
- `record_timer_tick` moves to/maintains `Ticking`

Invariant:

- `timer_ticks` monotonic non-decreasing.

## 4. Failure Modes

- Failure case: malformed memory map ranges (non-page aligned/empty)
  - Detection: normalization rejects invalid range
  - Recovery: skip invalid ranges; expose count in summary

- Failure case: overlapping virtual map request
  - Detection: existing entry on `map`
  - Recovery: return `MapError::AlreadyMapped`

- Failure case: interrupt setup invoked more than once
  - Detection: foundation state already initialized
  - Recovery: idempotent return with same-ready summary

## 5. Rollback Plan

Rollback trigger:

- Any boot regression that breaks M1 markers or causes fault before output.

Rollback steps:

1. disable M2 runtime wiring call sites from `efi_main`;
2. keep pure-library modules + tests behind no-runtime impact path;
3. restore previous boot-only log path;
4. reintroduce steps incrementally.

Data integrity considerations:

- no persistent on-disk state introduced in M2.

## 6. Compatibility

Backward compatibility impact:

- M1 outputs preserved; M2 adds new log markers only.

Linux ABI compatibility notes:

- no syscall ABI behavior changes;
- structure naming and contracts avoid conflicting future syscall/task-model assumptions.

## 7. Implementation Steps (Commit-Sized)

Architect-chosen commit phases:

1. **Phase 1 (docs/planning):** add M2 PRD/design + fundamentals index updates.
2. **Phase 2 (feat/memory):** implement frame allocator + virtual mapper modules with focused tests.
3. **Phase 3 (feat/interrupt-foundation):** add interrupt foundation state + timer tick accounting API.
4. **Phase 4 (feat/integration):** wire M2 foundation initialization and readiness logs into boot path.
5. **Phase 5 (review/docs):** add reviewer checklist report with Layer A/B/C evidence.

## 8. Test Strategy

Unit tests:

- frame allocator range normalization and allocate/free invariants.
- virtual mapper map/unmap/translate conflict cases.
- timer tick monotonic counter behavior.

Integration tests (local):

- compile checks for workspace + kernel target.
- smoke boot run with deterministic marker grep.

Demo validation:

- serial log contains M1 + M2 markers in one run.
- reviewer can re-run baseline commands and map evidence to ACs.
