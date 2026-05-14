//! UEFI boot services interface

use crate::boot::{BootInfo, MemoryMap, MemoryRegion, MemoryRegionType, FramebufferInfo};

/// Initialize UEFI boot services
pub fn init() -> BootInfo {
    // UEFI initialization is handled by the bootloader
    // This function would be called early in the boot process
    
    BootInfo {
        memory_map: &MemoryMap {
            regions: core::ptr::null(),
            count: 0,
        },
        kernel_offset: 0,
        rsdp_addr: None,
        framebuffer: None,
    }
}

/// UEFI memory type mapping
pub fn map_uefi_memory_type(uefi_type: u32) -> MemoryRegionType {
    match uefi_type {
        0 => MemoryRegionType::RESERVED,
        1 => MemoryRegionType::USABLE,
        2 | 5 => MemoryRegionType::RESERVED,
        3 | 4 => MemoryRegionType::ACPI_RECLAIMABLE,
        6 => MemoryRegionType::RESERVED,
        7 => MemoryRegionType::RESERVED,
        8..=10 => MemoryRegionType::LOADER_CODE,
        11..=19 => MemoryRegionType::LOADER_DATA,
        _ => MemoryRegionType::RESERVED,
    }
}

/// Exit UEFI boot services
pub fn exit_boot_services() {
    // Called by bootloader before transferring control to kernel
}
