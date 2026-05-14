//! UEFI Boot implementation for AuroraOS

use core::ptr;
use uefi::prelude::*;
use uefi::proto::console::gop::GraphicsOutput;
use uefi::table::boot::{MemoryMap, MemoryType};
use crate::BootInfo;

/// Initialize UEFI services and prepare for kernel boot
pub fn init() -> Result<(), &'static str> {
    // UEFI initialization handled by uefi-services
    Ok(())
}

/// Get framebuffer information from GOP
pub fn get_framebuffer(system_table: &SystemTable<Boot>) -> Option<(u64, u32, u32, u32)> {
    let gop = system_table.get_protocol::<GraphicsOutput>().ok()?;
    let mode = gop.current_mode_info();
    let frame_buffer = gop.frame_buffer();
    
    Some((
        frame_buffer.as_mut_ptr() as u64,
        mode.info().resolution().0,
        mode.info().resolution().1,
        mode.info().stride(),
    ))
}

/// Create memory map for kernel
pub fn create_memory_map(system_table: &mut SystemTable<Boot>) -> Option<(u64, usize)> {
    let mut buffer = [0u8; 4096];
    let (key, desc_size, desc_version) = {
        let mmap = system_table.boot_services()
            .memory_map(&mut buffer)
            .ok()?;
        (mmap.map_key(), mmap.desc().size(), mmap.desc_version())
    };
    
    Some((buffer.as_ptr() as u64, buffer.len()))
}

/// Find ACPI RSDP table address
pub fn find_acpi_rsdp(system_table: &SystemTable<Boot>) -> Option<u64> {
    let config_table = system_table.config_table();
    
    // Search for ACPI 2.0 GUID
    const ACPI_20_GUID: uefi::Guid = uefi::guid!("8868e871-e4f1-11d3-bc22-0080c73c8881");
    
    for entry in config_table.iter() {
        if entry.guid == ACPI_20_GUID {
            return Some(entry.address as u64);
        }
    }
    
    // Fallback to ACPI 1.0
    const ACPI_10_GUID: uefi::Guid = uefi::guid!("eb9d2d30-2d88-11d3-9a16-0090273fc14d");
    
    for entry in config_table.iter() {
        if entry.guid == ACPI_10_GUID {
            return Some(entry.address as u64);
        }
    }
    
    None
}

/// Prepare boot info structure for kernel
pub fn prepare_boot_info(system_table: &mut SystemTable<Boot>) -> Option<BootInfo> {
    let mut info = BootInfo::new();
    
    // Get framebuffer info
    if let Some((addr, width, height, pitch)) = get_framebuffer(system_table) {
        info.framebuffer_addr = addr;
        info.framebuffer_width = width;
        info.framebuffer_height = height;
        info.framebuffer_pitch = pitch;
    }
    
    // Get memory map
    if let Some((addr, size)) = create_memory_map(system_table) {
        info.memory_map_addr = addr;
        info.memory_map_size = size;
    }
    
    // Get ACPI RSDP
    if let Some(addr) = find_acpi_rsdp(system_table) {
        info.acpi_rsdp_addr = addr;
    }
    
    Some(info)
}

/// Exit boot services and hand control to kernel
pub unsafe fn exit_boot_services(
    system_table: &mut SystemTable<Boot>,
    boot_info: &BootInfo,
) -> ! {
    let boot_services = system_table.boot_services();
    
    // Exit boot services
    let image_handle = system_table.image_handle();
    boot_services.exit_boot_services(image_handle, boot_info.memory_map_addr as usize);
    
    // Jump to kernel entry point (placeholder - actual implementation loads kernel)
    loop {
        core::hint::spin_loop();
    }
}
