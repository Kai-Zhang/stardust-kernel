#![no_main]
#![no_std]

extern crate alloc;

use stardust_kernel::interrupts;
use stardust_kernel::userspace;
use uefi::boot::{self, MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;
use uefi::println;

const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
const M2B_TIMER_HZ: u32 = 100;
const M2B_TIMER_VECTOR: u8 = 32;
const M2B_DEMO_TICKS: usize = 8;

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    println!("stardust-kernel {}", KERNEL_VERSION);

    let memory_status = match boot::memory_map(MemoryType::LOADER_DATA) {
        Ok(memory_map) => {
            let mut descriptors = 0usize;
            let mut total_pages = 0u64;
            let mut conventional_pages = 0u64;

            for descriptor in memory_map.entries() {
                descriptors += 1;
                total_pages += descriptor.page_count;
                if descriptor.ty == MemoryType::CONVENTIONAL {
                    conventional_pages += descriptor.page_count;
                }
            }

            println!(
                "memmap: descriptors={} total_pages={} conventional_pages={}",
                descriptors, total_pages, conventional_pages
            );
            Status::SUCCESS
        }
        Err(err) => {
            println!("memmap: unavailable status={:?}", err.status());
            Status::LOAD_ERROR
        }
    };

    if memory_status != Status::SUCCESS {
        return memory_status;
    }

    let summary = interrupts::init_foundation();
    println!(
        "m2b:init gdt_ready={} tss_ready={} idt_ready={}",
        summary.gdt_ready, summary.tss_ready, summary.idt_ready
    );

    if let Err(err) = interrupts::bootstrap_periodic_timer(M2B_TIMER_HZ, M2B_TIMER_VECTOR) {
        println!("m2b:timer_setup status=error reason={:?}", err);
        return Status::ABORTED;
    }

    for _ in 0..M2B_DEMO_TICKS {
        if let Err(err) = interrupts::handle_timer_irq() {
            println!("m2b:irq status=error reason={:?}", err);
            return Status::ABORTED;
        }
    }

    let snap = interrupts::snapshot();
    println!(
        "m2b:timer_ticks hz={} vector={} ticks={} acks={}",
        snap.configured_hz, snap.timer_irq_vector, snap.total_ticks, snap.ack_count
    );

    let user_report = userspace::run_m3_demo_payload();
    println!(
        "m3:demo ring0_to_ring3=true returned_to_ring0={} bytes_written={} exit_code={}",
        user_report.final_ring == userspace::CpuRing::Ring0,
        user_report.bytes_written,
        user_report.exit_code
    );

    Status::SUCCESS
}
