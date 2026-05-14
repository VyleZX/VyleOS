//! Kernel panic handler

use core::panic::PanicInfo;
use crate::console;
use crate::arch;

/// Handle kernel panic
pub fn handle_panic(info: &PanicInfo) -> ! {
    // Disable interrupts
    x86_64::instructions::interrupts::disable();
    
    // Print panic message
    console::_print(format_args!("\n"));
    console::_print(format_args!("========================================\n"));
    console::_print(format_args!("           KERNEL PANIC\n"));
    console::_print(format_args!("========================================\n"));
    console::_print(format_args!("{}\n", info));
    console::_print(format_args!("========================================\n"));
    
    // Print register state
    print_registers();
    
    // Halt or reboot based on configuration
    #[cfg(feature = "panic_halt")]
    {
        loop {
            arch::halt();
        }
    }
    
    #[cfg(not(feature = "panic_halt"))]
    {
        arch::reboot();
    }
}

/// Print CPU register state
fn print_registers() {
    use crate::arch::x86_64::registers;
    
    console::_print(format_args!("\nRegister State:\n"));
    console::_print(format_args!("  RAX: {:016x}  RBX: {:016x}\n", 
        registers::rsp(), registers::rsp()));
    console::_print(format_args!("  RCX: {:016x}  RDX: {:016x}\n", 
        registers::rsp(), registers::rsp()));
    console::_print(format_args!("  RSP: {:016x}  RBP: {:016x}\n", 
        registers::rsp(), registers::rbp()));
    console::_print(format_args!("  RFLAGS: {:016x}\n", registers::rflags()));
}
