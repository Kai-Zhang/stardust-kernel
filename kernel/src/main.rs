#![no_main]
#![no_std]

extern crate alloc;

use uefi::boot::{self, MemoryType};
use uefi::mem::memory_map::MemoryMap;
use uefi::prelude::*;
use uefi::println;

const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry]
fn efi_main() -> Status {
    uefi::helpers::init().unwrap();

    println!("stardust-kernel {}", KERNEL_VERSION);

    match boot::memory_map(MemoryType::LOADER_DATA) {
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
    }
}
