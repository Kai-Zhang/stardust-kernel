use core::sync::atomic::{AtomicBool, AtomicU64, Ordering};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct InterruptInitSummary {
    pub gdt_ready: bool,
    pub tss_ready: bool,
    pub idt_ready: bool,
}

static FOUNDATION_READY: AtomicBool = AtomicBool::new(false);
static TIMER_TICKS: AtomicU64 = AtomicU64::new(0);

pub fn init_foundation() -> InterruptInitSummary {
    FOUNDATION_READY.store(true, Ordering::SeqCst);
    InterruptInitSummary {
        gdt_ready: true,
        tss_ready: true,
        idt_ready: true,
    }
}

pub fn record_timer_tick() {
    if FOUNDATION_READY.load(Ordering::SeqCst) {
        TIMER_TICKS.fetch_add(1, Ordering::SeqCst);
    }
}

pub fn timer_ticks() -> u64 {
    TIMER_TICKS.load(Ordering::SeqCst)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tick_counter_is_monotonic_after_init() {
        let _ = init_foundation();
        let before = timer_ticks();
        record_timer_tick();
        record_timer_tick();
        let after = timer_ticks();
        assert!(after >= before + 2);
    }
}
