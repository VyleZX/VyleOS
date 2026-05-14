//! Memory management subsystem

use spin::Mutex;
use crate::arch::x86_64::paging;
use x86_64::{PhysAddr, VirtAddr};
use x86_64::structures::paging::{Page, Size4KiB, PageTableFlags};

/// Physical memory allocator
static PHYS_ALLOCATOR: Mutex<Option<PhysicalMemoryAllocator>> = Mutex::new(None);

/// Kernel heap bounds
static mut HEAP_START: usize = 0;
static mut HEAP_END: usize = 0;

/// Initialize memory management
pub fn init(memory_map: &'static crate::boot::MemoryMap) {
    // Find usable memory regions from the memory map
    let mut allocator = PhysicalMemoryAllocator::new();
    
    for region in memory_map.iter() {
        if region.region_type == crate::boot::MemoryRegionType::USABLE {
            let start = region.physical_start as usize;
            let end = start + region.number_of_pages as usize * 4096;
            
            // Align to page boundaries
            let start = (start + 0xFFF) & !0xFFF;
            let end = end & !0xFFF;
            
            if end > start {
                allocator.add_region(start, end);
            }
        }
    }
    
    *PHYS_ALLOCATOR.lock() = Some(allocator);
    
    // Set up kernel heap bounds (from linker script symbols)
    unsafe {
        extern "C" {
            static _heap_start: u8;
            static _heap_end: u8;
        }
        HEAP_START = &_heap_start as *const u8 as usize;
        HEAP_END = &_heap_end as *const u8 as usize;
    }
}

/// Physical memory allocator using a bitmap
pub struct PhysicalMemoryAllocator {
    regions: [MemoryRegion; 16],
    region_count: usize,
}

#[derive(Clone, Copy)]
struct MemoryRegion {
    start: usize,
    end: usize,
    used_pages: usize,
    total_pages: usize,
}

impl PhysicalMemoryAllocator {
    /// Create a new physical memory allocator
    pub const fn new() -> Self {
        Self {
            regions: [MemoryRegion {
                start: 0,
                end: 0,
                used_pages: 0,
                total_pages: 0,
            }; 16],
            region_count: 0,
        }
    }
    
    /// Add a memory region to manage
    pub fn add_region(&mut self, start: usize, end: usize) {
        if self.region_count < self.regions.len() {
            let total_pages = (end - start) / 4096;
            self.regions[self.region_count] = MemoryRegion {
                start,
                end,
                used_pages: 0,
                total_pages,
            };
            self.region_count += 1;
        }
    }
    
    /// Allocate a physical frame
    pub fn allocate_frame(&mut self) -> Option<PhysAddr> {
        for region in &mut self.regions[..self.region_count] {
            if region.used_pages < region.total_pages {
                let page_offset = region.used_pages * 4096;
                region.used_pages += 1;
                return Some(PhysAddr::new(region.start as u64 + page_offset as u64));
            }
        }
        None
    }
    
    /// Free a physical frame
    pub fn deallocate_frame(&mut self, addr: PhysAddr) {
        let addr = addr.as_u64() as usize;
        for region in &mut self.regions[..self.region_count] {
            if addr >= region.start && addr < region.end {
                let page_offset = addr - region.start;
                let page_num = page_offset / 4096;
                if page_num < region.used_pages {
                    region.used_pages -= 1;
                }
                return;
            }
        }
    }
}

/// Allocate a physical frame
pub fn allocate_frame() -> Option<PhysAddr> {
    PHYS_ALLOCATOR.lock().as_mut().and_then(|a| a.allocate_frame())
}

/// Free a physical frame
pub fn deallocate_frame(addr: PhysAddr) {
    if let Some(ref mut allocator) = *PHYS_ALLOCATOR.lock() {
        allocator.deallocate_frame(addr);
    }
}

/// Map kernel memory
pub fn map_kernel_memory(virt: VirtAddr, phys: PhysAddr, flags: PageTableFlags) {
    let page = Page::containing_address(virt);
    let frame = x86_64::structures::paging::PhysFrame::containing_address(phys);
    
    paging::map_page(page, frame, flags).expect("Failed to map kernel memory");
}

/// Get kernel heap start address
pub fn heap_start() -> usize {
    unsafe { HEAP_START }
}

/// Get kernel heap end address
pub fn heap_end() -> usize {
    unsafe { HEAP_END }
}

/// Memory statistics
pub struct MemoryStats {
    pub total_physical: usize,
    pub used_physical: usize,
    pub free_physical: usize,
    pub total_kernel_heap: usize,
    pub used_kernel_heap: usize,
}

/// Get memory statistics
pub fn get_stats() -> MemoryStats {
    let mut total_physical = 0;
    let mut used_physical = 0;
    
    if let Some(ref allocator) = *PHYS_ALLOCATOR.lock() {
        for region in &allocator.regions[..allocator.region_count] {
            total_physical += region.total_pages * 4096;
            used_physical += region.used_pages * 4096;
        }
    }
    
    MemoryStats {
        total_physical,
        used_physical,
        free_physical: total_physical - used_physical,
        total_kernel_heap: unsafe { HEAP_END - HEAP_START },
        used_kernel_heap: 0, // Would need to track from allocator
    }
}
