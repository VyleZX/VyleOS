//! Memory Management Subsystem
//!
//! This module handles all memory-related operations including:
//! - Physical memory management (frame allocation)
//! - Virtual memory management (page tables)
//! - Heap allocation
//! - Address space management
//! - Memory protection

use bootloader::BootInfo;
use x86_64::{
    structures::paging::{
        Page, PageTable, FrameAllocator, Mapper, OffsetPageTable, PhysFrame, Size4KiB,
    },
    VirtAddr, PhysAddr,
};
use spin::Mutex;
use core::ptr::NonNull;

pub mod allocator;
pub mod frame;
pub mod page_table;
pub mod heap;

use self::allocator::MemoryAllocator;
use self::frame::FrameAllocator as PhysicalFrameAllocator;

/// Global memory allocator
static MEMORY_ALLOCATOR: Mutex<Option<MemoryAllocator>> = Mutex::new(None);

/// Initialize the memory management subsystem
pub fn init(boot_info: &'static mut BootInfo) {
    log::info!("Initializing memory management subsystem...");
    
    // Initialize physical frame allocator
    frame::init(boot_info);
    
    // Create offset page table mapper
    let phys_memory_offset = boot_info.physical_memory_offset;
    let mut mapper = unsafe { init_mapper(phys_memory_offset) };
    
    // Initialize heap
    heap::init(&mut mapper, boot_info);
    
    // Initialize memory allocator with available memory regions
    let mut allocator = MemoryAllocator::new();
    for region in boot_info.memory_map.iter() {
        if region.region_type == bootloader::boot_info::MemoryRegionType::Usable {
            allocator.add_region(region.range.start_addr(), region.range.end_addr());
        }
    }
    
    *MEMORY_ALLOCATOR.lock() = Some(allocator);
    
    log::info!("Memory management initialized successfully");
}

/// Initialize the page table mapper
unsafe fn init_mapper(offset: u64) -> OffsetPageTable<'static> {
    let phys_to_virt = |frame: PhysFrame| -> *mut PageTable {
        let phys = frame.start_address().as_u64();
        let virt = offset + phys;
        (virt as *mut PageTable)
    };
    
    OffsetPageTable::new(phys_to_virt(PhysFrame::containing_address(PhysAddr::new(0))), VirtAddr::new(offset))
}

/// Get a reference to the global memory allocator
pub fn get_allocator() -> Option<impl core::ops::Deref<Target = MemoryAllocator>> {
    MEMORY_ALLOCATOR.lock()
}

/// Allocate a physical frame
pub fn allocate_frame() -> Option<PhysFrame> {
    frame::allocate_frame()
}

/// Deallocate a physical frame
pub fn deallocate_frame(frame: PhysFrame) {
    frame::deallocate_frame(frame);
}

/// Convert physical address to virtual address
pub fn phys_to_virt(addr: PhysAddr) -> VirtAddr {
    // This would use the actual mapping offset from boot info
    VirtAddr::new(addr.as_u64())
}

/// Convert virtual address to physical address
pub fn virt_to_phys(addr: VirtAddr) -> PhysAddr {
    PhysAddr::new(addr.as_u64())
}

/// Memory region types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    ACPI,
    MMIO,
    Kernel,
    User,
}

/// Represents a memory region
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub start: PhysAddr,
    pub end: PhysAddr,
    pub region_type: MemoryRegionType,
}

impl MemoryRegion {
    pub fn new(start: u64, end: u64, region_type: MemoryRegionType) -> Self {
        Self {
            start: PhysAddr::new(start),
            end: PhysAddr::new(end),
            region_type,
        }
    }
    
    pub fn size(&self) -> u64 {
        self.end.as_u64() - self.start.as_u64()
    }
}
