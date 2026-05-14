//! Architecture-specific code for x86_64

pub mod gdt;
pub mod idt;
pub mod paging;
pub mod registers;
pub mod cpuid;

use x86_64::instructions::interrupts;

/// Initialize architecture-specific components
pub fn init() {
    // Disable interrupts during initialization
    interrupts::disable();
    
    // Initialize GDT
    gdt::init();
    
    // Initialize paging
    paging::init();
}

/// Halt the CPU
#[inline(always)]
pub fn halt() {
    x86_64::instructions::hlt();
}

/// Reboot the system
pub fn reboot() -> ! {
    use x86_64::instructions::port::Port;
    
    unsafe {
        // Send reset command via keyboard controller
        let mut port = Port::<u8>::new(0x64);
        port.write(0xFE);
        
        // If that fails, triple fault
        loop {
            x86_64::instructions::hlt();
        }
    }
}

/// Shutdown the system (ACPI)
pub fn shutdown() -> ! {
    // Try ACPI power off
    #[cfg(feature = "acpi_support")]
    {
        crate::boot::acpi::shutdown();
    }
    
    // If ACPI fails, just halt
    loop {
        x86_64::instructions::hlt();
    }
}
