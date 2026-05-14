//! AuroraOS Kernel - Main Entry Point
//!
//! This is the main entry point for the AuroraOS kernel.
//! The kernel is responsible for:
//! - Hardware initialization
//! - Memory management
//! - Process scheduling
//! - System calls
//! - Device management

#![no_std]
#![no_main]
#![feature(naked_functions)]
#![feature(asm_const)]
#![feature(abi_x86_64_sysv)]

extern crate alloc;

mod arch;
mod boot;
mod console;
mod drivers;
mod fs;
mod ipc;
mod loader;
mod memory;
mod net;
mod panic;
mod scheduler;
mod security;
mod smp;
mod syscall;

use core::panic::PanicInfo;
use log::{info, error};

/// Kernel version information
pub const KERNEL_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const KERNEL_NAME: &str = "AuroraOS Kernel";

/// Kernel entry point called by bootloader
#[no_mangle]
pub extern "C" fn _start(boot_info: &'static boot::BootInfo) -> ! {
    // Initialize console early for debug output
    console::init();
    
    info!("{} v{}", KERNEL_NAME, KERNEL_VERSION);
    info!("Booting AuroraOS...");
    
    // Initialize architecture-specific components
    arch::init();
    
    // Initialize memory management
    memory::init(boot_info.memory_map);
    info!("Memory management initialized");
    
    // Initialize interrupt descriptor table
    arch::x86_64::idt::init();
    info!("Interrupt descriptor table initialized");
    
    // Initialize ACPI and hardware discovery
    boot::acpi::init();
    info!("ACPI initialized");
    
    // Initialize PCI and device enumeration
    drivers::pci::init();
    info!("PCI enumeration complete");
    
    // Initialize scheduler
    scheduler::init();
    info!("Scheduler initialized");
    
    // Initialize filesystem subsystem
    fs::init();
    info!("Filesystem subsystem initialized");
    
    // Initialize network subsystem
    net::init();
    info!("Network subsystem initialized");
    
    // Initialize IPC system
    ipc::init();
    info!("IPC system initialized");
    
    // Initialize security subsystem
    security::init();
    info!("Security subsystem initialized");
    
    // Load and start init process
    loader::load_init();
    
    // Start scheduler (never returns)
    scheduler::run();
}

/// Kernel panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("Kernel panic: {}", info);
    panic::handle_panic(info);
}
