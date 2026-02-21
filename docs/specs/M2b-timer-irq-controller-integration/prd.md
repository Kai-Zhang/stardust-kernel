# M2b PRD — Timer + IRQ Controller Integration

- Milestone: `M2b-timer-irq-controller-integration`
- Status: Draft
- Owner: Orchestrator + Codex roles
- Depends on: `M2-memory-interrupt-foundation`

## 1. Problem Statement

M2 delivered interrupt foundation readiness and deterministic tick bookkeeping, but not explicit timer source configuration nor IRQ controller-level routing/ack flow contracts.

Without these contracts, periodic tick behavior remains underspecified for scheduler-prep milestones.

## 2. Scope

### In Scope

- Add programmable timer source setup with frequency input validation.
- Add explicit timer IRQ routing path and acknowledge bookkeeping.
- Expose deterministic integration snapshot for ticks/acks/frequency/vector.
- Wire M2b boot evidence marker showing repeated tick progress.

### Out of Scope (Non-Goals)

- Full APIC/PIC production hardware driver.
- SMP interrupt distribution.
- Scheduler and preemption policy.
- Userspace signal/timer ABI exposure.

## 3. MVP Definition

Minimal demoable result:

1. Boot path initializes interrupt foundation and configures periodic timer at a fixed demo frequency.
2. Timer IRQ handling path can be invoked repeatedly with ack accounting.
3. Boot log prints one deterministic `m2b:timer_ticks ...` marker with non-zero ticks/acks.

## 4. Functional Requirements

- FR-1: `configure_periodic_timer(hz)` rejects `hz == 0` and stores configured frequency.
- FR-2: `route_timer_irq(vector)` sets timer IRQ vector; default vector fallback is defined.
- FR-3: `handle_timer_irq()` increments tick counter and always records an acknowledge event.
- FR-4: `snapshot()` exposes `ticks/acks/hz/vector` for smoke evidence.
- FR-5: Boot integration emits M2b markers and aborts cleanly on setup/IRQ errors.

## 5. Acceptance Criteria (Testable)

- AC-1: Layer A checks pass for touched scope.
- AC-2: Unit tests cover frequency validation, unrouted IRQ failure, routed repeated IRQ progress.
- AC-3: Boot output includes marker:
  - `m2b:timer_ticks hz=... vector=... ticks=... acks=...`
- AC-4: Reviewer baseline (Layer C) reproduces marker and confirms `ticks == acks` growth behavior in smoke run.

## 6. Demo Plan

1. `cargo fmt --check`
2. `cargo check --workspace`
3. `cargo clippy --workspace --all-targets -- -D warnings`
4. `cargo test -p stardust-kernel`
5. `scripts/build.sh`
6. QEMU smoke run (`timeout 8s scripts/run-qemu.sh`) and capture `m2b:*` lines.

## 7. Risks & Open Questions

- QEMU/UEFI environment may not deliver real hardware periodic IRQ at this phase; implementation keeps deterministic controller contract via explicit path invocation.
- Real APIC/PIT backend can be introduced in a later milestone while preserving M2b API and marker compatibility.
