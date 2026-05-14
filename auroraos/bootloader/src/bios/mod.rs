//! BIOS Boot implementation for AuroraOS
//! 
//! This module provides legacy BIOS boot support using multiboot protocol.

#![no_std]

/// Multiboot magic number
pub const MULTIBOOT_MAGIC: u32 = 0x1BADB002;

/// Multiboot flags
pub const MULTIBOOT_FLAGS_ALIGN: u32 = 1 << 0;
pub const MULTIBOOT_FLAGS_MEMINFO: u32 = 1 << 1;
pub const MULTIBOOT_FLAGS_VIDEO: u32 = 1 << 2;

/// Multiboot header structure
#[repr(C, align(4))]
pub struct MultibootHeader {
    pub magic: u32,
    pub flags: u32,
    pub checksum: u32,
    pub header_addr: u32,
    pub load_addr: u32,
    pub load_end_addr: u32,
    pub bss_end_addr: u32,
    pub entry_addr: u32,
    pub mode_type: u32,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
}

impl MultibootHeader {
    pub const fn new() -> Self {
        let flags = MULTIBOOT_FLAGS_ALIGN | MULTIBOOT_FLAGS_MEMINFO | MULTIBOOT_FLAGS_VIDEO;
        Self {
            magic: MULTIBOOT_MAGIC,
            flags,
            checksum: !(MULTIBOOT_MAGIC.wrapping_add(flags)),
            header_addr: 0,
            load_addr: 0,
            load_end_addr: 0,
            bss_end_addr: 0,
            entry_addr: 0,
            mode_type: 1, // Linear graphics mode
            width: 1920,
            height: 1080,
            depth: 32,
        }
    }
}

/// Multiboot information structure (provided by bootloader)
#[repr(C)]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub syms: [u32; 4],
    pub mmap_length: u32,
    pub mmap_addr: u32,
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer_addr: u64,
    pub framebuffer_pitch: u32,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_bpp: u8,
    pub framebuffer_type: u8,
    pub reserved: [u8; 2],
}

/// Memory map entry from multiboot
#[repr(C, packed)]
pub struct MultibootMemoryMapEntry {
    pub size: u32,
    pub addr_low: u32,
    pub addr_high: u32,
    pub len_low: u32,
    pub len_high: u32,
    pub r#type: u32,
}

/// Initialize BIOS boot environment
pub fn init() -> Result<(), &'static str> {
    // BIOS initialization is minimal compared to UEFI
    Ok(())
}

/// Parse multiboot information
pub unsafe fn parse_multiboot_info(info_addr: u32) -> Option<&'static MultibootInfo> {
    if info_addr == 0 {
        return None;
    }
    
    Some(&*(info_addr as *const MultibootInfo))
}

/// Extract framebuffer info from multiboot
pub fn get_framebuffer(info: &MultibootInfo) -> Option<(u64, u32, u32, u32)> {
    if info.flags & (1 << 11) == 0 {
        return None;
    }
    
    Some((
        info.framebuffer_addr,
        info.framebuffer_width,
        info.framebuffer_height,
        info.framebuffer_pitch,
    ))
}
