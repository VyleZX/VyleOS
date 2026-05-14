//! AuroraOS Bootloader Library
//! 
//! This library provides common bootloader functionality for both UEFI and BIOS boot paths.

#![no_std]

pub mod efi;
pub mod bios;

/// Boot status codes
#[repr(u32)]
pub enum BootStatus {
    Success = 0,
    Failed = 1,
    Unsupported = 2,
}

/// Boot information passed from bootloader to kernel
#[repr(C)]
pub struct BootInfo {
    pub magic: u32,
    pub memory_map_addr: u64,
    pub memory_map_size: usize,
    pub framebuffer_addr: u64,
    pub framebuffer_width: u32,
    pub framebuffer_height: u32,
    pub framebuffer_pitch: u32,
    pub acpi_rsdp_addr: u64,
    pub reserved: [u64; 8],
}

impl BootInfo {
    pub const MAGIC: u32 = 0xAUR0RA0B;
    
    pub fn new() -> Self {
        Self {
            magic: Self::MAGIC,
            memory_map_addr: 0,
            memory_map_size: 0,
            framebuffer_addr: 0,
            framebuffer_width: 0,
            framebuffer_height: 0,
            framebuffer_pitch: 0,
            acpi_rsdp_addr: 0,
            reserved: [0; 8],
        }
    }
}
