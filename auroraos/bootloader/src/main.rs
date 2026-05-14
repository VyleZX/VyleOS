//! AuroraOS UEFI Bootloader
//!
//! This is the main entry point for the UEFI bootloader.

#![no_std]
#![no_main]

use uefi::prelude::*;
use uefi_services::println;

#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize utilities
    uefi_services::init(&mut system_table).expect("Failed to initialize utilities");
    
    println!("AuroraOS Bootloader Starting...");
    
    // Get boot information
    match aurora_bootloader::efi::prepare_boot_info(&mut system_table) {
        Some(boot_info) => {
            println!("Boot info prepared successfully");
            println!("  Framebuffer: {}x{}", boot_info.framebuffer_width, boot_info.framebuffer_height);
            println!("  Memory map at: 0x{:X}", boot_info.memory_map_addr);
            println!("  ACPI RSDP at: 0x{:X}", boot_info.acpi_rsdp_addr);
            
            // In a real implementation, we would load the kernel here
            // and pass the boot_info structure to it
            
            println!("Kernel loading not yet implemented [STUB]");
        }
        None => {
            println!("Failed to prepare boot info");
            return Status::ABORTED;
        }
    }
    
    // For now, just exit gracefully
    println!("Bootloader stub complete. System halted.");
    
    loop {
        core::hint::spin_loop();
    }
}

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC: {}", info);
    loop {
        core::hint::spin_loop();
    }
}
