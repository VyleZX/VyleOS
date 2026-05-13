//! Kernel Panic Handler
//!
//! Handles kernel panics with detailed error information and debugging output.

use core::panic::PanicInfo;
use crate::arch;

/// Handle a kernel panic
pub fn handle_panic(info: &PanicInfo) -> ! {
    // Disable interrupts to prevent further issues
    arch::disable_interrupts();
    
    // Print panic message
    println!("\n");
    println!("=================================================");
    println!("           AURORA OS KERNEL PANIC                ");
    println!("=================================================");
    println!("\n{}", info);
    
    // Print CPU registers if available
    print_registers();
    
    // Print stack trace
    print_stack_trace();
    
    // Halt the system
    loop {
        arch::halt();
    }
}

/// Print CPU register state (placeholder)
fn print_registers() {
    println!("\nCPU Registers:");
    println!("  This would show register dump in full implementation");
}

/// Print stack trace (placeholder)
fn print_stack_trace() {
    println!("\nStack Trace:");
    println!("  This would show stack backtrace in full implementation");
}

/// Panic hook for better error messages
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    handle_panic(info)
}
