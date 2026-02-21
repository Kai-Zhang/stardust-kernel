#![no_std]

extern crate alloc;

pub mod interrupts;
pub mod memory;
pub mod userspace;

/// Kernel entrypoint wiring and subsystem initialization will be added
/// incrementally following the milestone roadmap.
pub fn kernel_placeholder() {}
