# M2b Theory Note: Timer + IRQ Controller Integration

This note explains how M2b extends M2 interrupt foundations into explicit timer-source and IRQ-controller contracts.

Related artifacts:

- PRD: `docs/specs/M2b-timer-irq-controller-integration/prd.md`
- Design: `docs/specs/M2b-timer-irq-controller-integration/design.md`
- Review: `docs/reviews/M2b-timer-irq-controller-integration/review.md`

## 1) What M2b Proves

M2b proves three integration contracts are explicit and testable:

1. Timer frequency setup is configurable and validated.
2. Timer IRQ routing and acknowledge path are observable.
3. Repeated periodic tick handling shows deterministic counter progress.

Boot evidence markers:

- `m2b:init gdt_ready=... tss_ready=... idt_ready=...`
- `m2b:timer_ticks hz=... vector=... ticks=... acks=...`

## 2) Timer Source Setup

`kernel/src/interrupts.rs` exposes `configure_periodic_timer` with a strict rule:

- `hz` must be non-zero.

This keeps frequency control explicit and avoids hidden defaults that complicate later scheduler math.

## 3) IRQ Routing + Acknowledge

`route_timer_irq` stores timer vector selection (default vector fallback allowed).

`handle_timer_irq` enforces a routed-IRQ precondition, then:

1. increments tick counter,
2. records acknowledge event.

This models the essential controller contract needed before introducing real APIC/PIT backend details.

## 4) Observable Tick Behavior

M2b boot path configures timer + routing, simulates repeated timer IRQ handling, and prints one deterministic snapshot marker.

Teaching model:

- if setup succeeds, both `ticks` and `acks` advance together;
- if setup/routing fails, boot emits explicit error marker and aborts cleanly.

## 5) Why This Scope Is Minimal

M2b is still foundation/integration work. It intentionally does **not** claim production hardware controller completeness.

The goal is stable contracts and evidence that unblock subsequent preemption-facing milestones.
