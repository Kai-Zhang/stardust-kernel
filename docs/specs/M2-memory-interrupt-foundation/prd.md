# M2 PRD — Memory + Interrupt Foundation

- Milestone: `M2-memory-interrupt-foundation`
- Status: Draft
- Owner: Orchestrator + Codex roles
- Depends on: `M1-boot-to-rust`

## 1. Problem Statement

M1 proves boot-to-Rust and early diagnostics, but the kernel still lacks core runtime foundations:

- a managed source of physical frames,
- a minimal virtual mapping contract,
- interrupt descriptor/table setup with timer wiring.

Without these, later milestones (userspace, syscalls, process model) cannot be built in a controlled way.

## 2. Scope

### In Scope

- Introduce a physical frame allocator abstraction initialized from boot memory map data.
- Add a basic virtual memory mapping API with explicit map/unmap/query semantics.
- Add interrupt foundation setup types for GDT/TSS/IDT and a minimal timer-interrupt wiring path.
- Emit deterministic runtime evidence markers for memory + interrupt foundation readiness.
- Add focused tests/smoke checks aligned to Layer A/B/C model.

### Out of Scope (Non-Goals)

- Full demand-paged virtual memory implementation.
- Userspace ring transition and syscall dispatch.
- Complete APIC/PIC production-grade interrupt controller stack.
- Scheduler and preemption policy.

## 3. MVP Definition

Minimal runnable/demoable result:

1. Kernel boot initializes M2 foundation modules without faulting.
2. Boot output includes:
   - frame allocator summary,
   - virtual mapping smoke summary,
   - timer wiring readiness markers.
3. Host-side checks and deterministic local smoke flow pass.

Explicit constraints:

- Keep implementation small, readable, and teaching-oriented.
- Preserve Linux ABI roadmap direction; avoid speculative interfaces.
- Keep M1 boot path intact.

## 4. Functional Requirements

- FR-1: Build a frame allocator state from boot memory map conventional ranges and support deterministic frame allocation/free APIs.
- FR-2: Provide a basic virtual mapping table API supporting map, unmap, and translate/query with conflict detection.
- FR-3: Provide interrupt foundation setup entrypoints for GDT/TSS/IDT and timer tick accounting hook.
- FR-4: On boot, print M2 readiness markers with counts suitable for smoke verification.

## 5. Acceptance Criteria (Testable)

- AC-1: `cargo check --workspace`, `cargo fmt --check`, and `cargo clippy --workspace --all-targets` pass (Layer A).
- AC-2: Kernel runtime boot log shows M2 markers:
  - `m2:frames ...`
  - `m2:vmap ...`
  - `m2:interrupts timer_ticks=...`
- AC-3: Mapping API tests cover success + overlap/release behavior and pass (Layer B).
- AC-4: Reviewer baseline run can reproduce markers and map each to acceptance evidence (Layer C).

## 6. Demo Plan

Demo steps:

1. `cargo check --workspace`
2. `scripts/build.sh`
3. Run kernel smoke and capture logs:
   - `timeout 8s scripts/run-qemu.sh` (or equivalent host timeout wrapper)

Expected visible behavior:

- M1 boot markers remain present.
- M2 readiness markers appear in serial output.
- No crash/fault marker before timeout boundary.

## 7. Risks & Open Questions

Risk:

- Real hardware timer IRQ delivery inside the current UEFI-app-shaped boot stage may vary by firmware/emulator policy.

Question:

- For M2 scope closure, should timer wiring evidence be “interrupt subsystem initialized + deterministic tick accounting path active” rather than hard dependency on production periodic IRQ source at this stage? (Current proposal: yes, as foundation milestone)
