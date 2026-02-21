use core::sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterruptInitSummary {
    pub gdt_ready: bool,
    pub tss_ready: bool,
    pub idt_ready: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimerSourceConfig {
    pub hz: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TickSnapshot {
    pub total_ticks: u64,
    pub ack_count: u64,
    pub configured_hz: u32,
    pub timer_irq_vector: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimerIntegrationError {
    FoundationNotReady,
    InvalidFrequency,
    TimerIrqNotRouted,
}

const DEFAULT_TIMER_IRQ_VECTOR: u8 = 32;

static FOUNDATION_READY: AtomicBool = AtomicBool::new(false);
static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);
static TIMER_ACKS: AtomicU64 = AtomicU64::new(0);
static TIMER_HZ: AtomicU32 = AtomicU32::new(0);
static TIMER_IRQ_VECTOR: AtomicU32 = AtomicU32::new(0);

pub fn init_foundation() -> InterruptInitSummary {
    FOUNDATION_READY.store(true, Ordering::SeqCst);
    InterruptInitSummary {
        gdt_ready: true,
        tss_ready: true,
        idt_ready: true,
    }
}

pub fn timer_ticks() -> u64 {
    TIMER_TICKS.load(Ordering::SeqCst)
}

pub fn configure_periodic_timer(config: TimerSourceConfig) -> Result<(), TimerIntegrationError> {
    if !FOUNDATION_READY.load(Ordering::SeqCst) {
        return Err(TimerIntegrationError::FoundationNotReady);
    }
    if config.hz == 0 {
        return Err(TimerIntegrationError::InvalidFrequency);
    }

    TIMER_HZ.store(config.hz, Ordering::SeqCst);
    Ok(())
}

pub fn route_timer_irq(vector: u8) -> Result<(), TimerIntegrationError> {
    if !FOUNDATION_READY.load(Ordering::SeqCst) {
        return Err(TimerIntegrationError::FoundationNotReady);
    }

    let normalized = if vector == 0 {
        DEFAULT_TIMER_IRQ_VECTOR
    } else {
        vector
    };

    TIMER_IRQ_VECTOR.store(normalized as u32, Ordering::SeqCst);
    Ok(())
}

pub fn handle_timer_irq() -> Result<u64, TimerIntegrationError> {
    if TIMER_IRQ_VECTOR.load(Ordering::SeqCst) == 0 {
        return Err(TimerIntegrationError::TimerIrqNotRouted);
    }

    let tick = TIMER_TICKS.fetch_add(1, Ordering::SeqCst) + 1;
    acknowledge_timer_irq();
    Ok(tick)
}

pub fn acknowledge_timer_irq() {
    TIMER_ACKS.fetch_add(1, Ordering::SeqCst);
}

pub fn snapshot() -> TickSnapshot {
    TickSnapshot {
        total_ticks: TIMER_TICKS.load(Ordering::SeqCst),
        ack_count: TIMER_ACKS.load(Ordering::SeqCst),
        configured_hz: TIMER_HZ.load(Ordering::SeqCst),
        timer_irq_vector: TIMER_IRQ_VECTOR.load(Ordering::SeqCst) as u8,
    }
}

pub fn bootstrap_periodic_timer(hz: u32, vector: u8) -> Result<(), TimerIntegrationError> {
    configure_periodic_timer(TimerSourceConfig { hz })?;
    route_timer_irq(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_LOCK: AtomicBool = AtomicBool::new(false);

    fn with_test_lock<F: FnOnce()>(f: F) {
        while TEST_LOCK
            .compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
            .is_err()
        {
            core::hint::spin_loop();
        }
        f();
        TEST_LOCK.store(false, Ordering::SeqCst);
    }

    fn reset_state() {
        FOUNDATION_READY.store(false, Ordering::SeqCst);
        TIMER_TICKS.store(0, Ordering::SeqCst);
        TIMER_ACKS.store(0, Ordering::SeqCst);
        TIMER_HZ.store(0, Ordering::SeqCst);
        TIMER_IRQ_VECTOR.store(0, Ordering::SeqCst);
    }

    #[test]
    fn timer_requires_foundation_and_valid_frequency() {
        with_test_lock(|| {
            reset_state();
            assert_eq!(
                configure_periodic_timer(TimerSourceConfig { hz: 100 }),
                Err(TimerIntegrationError::FoundationNotReady)
            );

            let _ = init_foundation();
            assert_eq!(
                configure_periodic_timer(TimerSourceConfig { hz: 0 }),
                Err(TimerIntegrationError::InvalidFrequency)
            );
        });
    }

    #[test]
    fn routed_timer_irq_produces_tick_and_ack_progress() {
        with_test_lock(|| {
            reset_state();
            let _ = init_foundation();
            let start = snapshot();

            bootstrap_periodic_timer(100, 32).expect("timer setup must succeed");
            let _ = handle_timer_irq().expect("timer irq should be routable");
            let _ = handle_timer_irq().expect("timer irq should be repeatable");

            let end = snapshot();
            assert_eq!(end.configured_hz, 100);
            assert_eq!(end.timer_irq_vector, 32);
            assert!(end.total_ticks >= start.total_ticks + 2);
            assert!(end.ack_count >= start.ack_count + 2);
        });
    }

    #[test]
    fn handle_fails_without_irq_routing() {
        with_test_lock(|| {
            reset_state();
            FOUNDATION_READY.store(true, Ordering::SeqCst);
            TIMER_IRQ_VECTOR.store(0, Ordering::SeqCst);
            assert_eq!(
                handle_timer_irq(),
                Err(TimerIntegrationError::TimerIrqNotRouted)
            );
        });
    }
}
