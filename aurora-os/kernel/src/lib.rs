#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(naked_functions)]
#![feature(asm_const)]
#![deny(warnings)]
#![deny(clippy::all)]

//! # Aurora OS Kernel
//! 
//! A modern, secure, and performant operating system kernel written in Rust.
//! 
//! ## Architecture
//! 
//! The kernel follows a hybrid microkernel design with:
//! - Minimal trusted computing base in kernel space
//! - Most drivers and services running in userspace
//! - High-performance IPC for communication
//! - Capability-based security model

extern crate alloc;

pub mod arch;
pub mod mm;
pub mod sched;
pub mod ipc;
pub mod syscall;
pub mod drivers;
pub mod fs;
pub mod vfs;
pub mod security;
pub mod panic;
pub mod acpi;

use core::panic::PanicInfo;

/// Kernel version information
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const KERNEL_NAME: &str = "Aurora OS Kernel";

/// Kernel entry point called by bootloader
#[no_mangle]
pub extern "C" fn kernel_main(boot_info: &'static mut bootloader::BootInfo) -> ! {
    // Initialize logging subsystem
    println!("[KERNEL] {} v{}", KERNEL_NAME, KERNEL_VERSION);
    
    // Initialize architecture-specific components
    arch::init();
    
    // Initialize memory management
    mm::init(boot_info);
    
    // Initialize ACPI for power management and hardware discovery
    acpi::init();
    
    // Initialize interrupt descriptor table
    arch::interrupts::init();
    
    // Initialize scheduler
    sched::init();
    
    // Initialize IPC subsystem
    ipc::init();
    
    // Initialize VFS layer
    vfs::init();
    
    // Initialize device drivers
    drivers::init();
    
    // Initialize security subsystem
    security::init();
    
    println!("[KERNEL] Initialization complete, starting userspace...");
    
    // Start the init process
    sched::spawn_init();
    
    // Enter idle loop - scheduler takes over
    loop {
        arch::halt();
    }
}

/// Panic handler - called when a panic occurs
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    panic::handle_panic(info);
}
