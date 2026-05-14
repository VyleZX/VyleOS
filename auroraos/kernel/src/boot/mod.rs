//! Boot information and memory map

use bitflags::bitflags;

/// Boot information provided by the bootloader
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct BootInfo {
    /// Memory map from bootloader
    pub memory_map: &'static MemoryMap,
    /// Kernel physical offset
    pub kernel_offset: u64,
    /// ACPI RSDP pointer
    pub rsdp_addr: Option<u64>,
    /// Framebuffer info (if available)
    pub framebuffer: Option<FramebufferInfo>,
}

/// Memory map containing all physical memory regions
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryMap {
    /// Pointer to memory regions
    pub regions: *const MemoryRegion,
    /// Number of regions
    pub count: usize,
}

impl MemoryMap {
    /// Iterate over memory regions
    pub fn iter(&self) -> MemoryMapIter {
        MemoryMapIter {
            regions: self.regions,
            count: self.count,
            index: 0,
        }
    }
}

/// Iterator over memory regions
pub struct MemoryMapIter {
    regions: *const MemoryRegion,
    count: usize,
    index: usize,
}

impl Iterator for MemoryMapIter {
    type Item = &'static MemoryRegion;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.count {
            return None;
        }
        
        let region = unsafe { &*self.regions.add(self.index) };
        self.index += 1;
        Some(region)
    }
}

/// A single memory region
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MemoryRegion {
    /// Physical start address
    pub physical_start: u64,
    /// Number of pages (4KB each)
    pub number_of_pages: u64,
    /// Region type
    pub region_type: MemoryRegionType,
}

/// Type of memory region
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    USABLE = 0,
    RESERVED = 1,
    ACPI_RECLAIMABLE = 2,
    ACPI_NVS = 3,
    BAD_MEMORY = 4,
    LOADER_CODE = 5,
    LOADER_DATA = 6,
    BOOT_LOADER = 7,
}

/// Framebuffer information
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FramebufferInfo {
    /// Framebuffer base address
    pub addr: u64,
    /// Width in pixels
    pub width: u32,
    /// Height in pixels
    pub height: u32,
    /// Bytes per pixel
    pub bpp: u8,
    /// Pitch (bytes per line)
    pub pitch: u32,
}

bitflags! {
    /// Memory region flags
    #[derive(Debug, Clone, Copy)]
    pub struct MemoryFlags: u32 {
        const PRESENT = 1 << 0;
        const WRITABLE = 1 << 1;
        const USER_ACCESSIBLE = 1 << 2;
        const NO_EXECUTE = 1 << 3;
    }
}
