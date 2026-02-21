# M2b Design — Timer + IRQ Controller Integration

- Milestone: `M2b-timer-irq-controller-integration`
- Status: Draft
- Linked PRD: `docs/specs/M2b-timer-irq-controller-integration/prd.md`

## 1. Context and Goals

M2b closes the gap between foundation-only interrupt readiness and explicit timer IRQ controller contracts required by subsequent preemption-facing work.

Goals:

- explicit timer source setup/frequency control;
- explicit IRQ routing + acknowledge path;
- deterministic periodic tick evidence for smoke/review.

## 2. Interfaces and Contracts

- `configure_periodic_timer(TimerSourceConfig { hz }) -> Result<(), TimerIntegrationError>`
  - `hz == 0` => `InvalidFrequency`
- `route_timer_irq(vector: u8) -> Result<(), TimerIntegrationError>`
  - `vector == 0` normalized to default `32`
- `bootstrap_periodic_timer(hz, vector)`
  - convenience setup wrapper
- `handle_timer_irq() -> Result<u64, TimerIntegrationError>`
  - requires routed vector
  - increments tick and acknowledge counters
- `snapshot() -> TickSnapshot`
  - exposes `total_ticks`, `ack_count`, `configured_hz`, `timer_irq_vector`

Error surface:

- `FoundationNotReady`
- `InvalidFrequency`
- `TimerIrqNotRouted`

## 3. State Model

States:

- `Cold` (foundation not ready)
- `FoundationReady`
- `TimerConfigured`
- `IrqRouted`
- `Ticking`

Transitions:

1. `init_foundation`: `Cold -> FoundationReady`
2. `configure_periodic_timer`: `FoundationReady -> TimerConfigured`
3. `route_timer_irq`: `FoundationReady/TimerConfigured -> IrqRouted`
4. `handle_timer_irq`: `IrqRouted -> Ticking` (repeatable)

Invariant:

- In normal timer IRQ path, `ack_count` grows with `total_ticks` (same step).

## 4. Failure Modes and Recovery

- setup before foundation:
  - detect: `FoundationNotReady`
  - recovery: call `init_foundation` first
- zero frequency:
  - detect: `InvalidFrequency`
  - recovery: provide non-zero hz
- IRQ handling before routing:
  - detect: `TimerIrqNotRouted`
  - recovery: call `route_timer_irq`

## 5. Rollback Plan

If boot integration regresses M1/M2 behavior:

1. keep interrupt module API/tests;
2. temporarily remove M2b boot wiring in `kernel/src/main.rs`;
3. restore previous success path and reintroduce wiring incrementally.

## 6. Compatibility

- No syscall ABI changes.
- Keeps UEFI boot diagnostics and M1 memory summary.
- M2 foundation API remains valid; M2b adds explicit controller contracts.

## 7. Architect Commit Phases (Implementation Split)

1. **Phase 1 — Docs/Spec**
   - add M2b PRD + design
2. **Phase 2 — Interrupt API Extension**
   - implement timer config/routing/ack/snapshot/error contracts
3. **Phase 3 — Boot Integration Evidence**
   - wire M2b setup + deterministic tick marker in boot path
4. **Phase 4 — Tests + Reviewer Evidence**
   - unit tests, fundamentals note, review report, roadmap status update

## 8. Test Strategy

- Layer A: fmt/check/clippy on workspace
- Layer B: `cargo test -p stardust-kernel` + build smoke
- Layer C: reviewer baseline run reproducing `m2b:timer_ticks ...` marker with stable counters
