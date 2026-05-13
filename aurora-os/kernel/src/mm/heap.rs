//! Kernel Heap Allocator

use x86_64::{
    structures::paging::{Mapper, Page, Size4KiB},
    VirtAddr,
};
use bootloader::BootInfo;
use linked_list_allocator::LockedHeap;

/// Global heap allocator
#[global_allocator]
static HEAP_ALLOCATOR: LockedHeap = LockedHeap::empty();

/// Heap bounds
static mut HEAP_START: usize = 0;
static mut HEAP_SIZE: usize = 0;

/// Initialize the kernel heap
pub fn init(mapper: &mut impl Mapper<Size4KiB>, boot_info: &'static BootInfo) {
    use x86_64::structures::paging::FrameAllocator;
    
    // Get heap location from boot info or use default
    let heap_start = boot_info.physical_memory_offset + 0x10000000; // Example location
    let heap_size = 1024 * 1024 * 16; // 16 MB initial heap
    
    unsafe {
        HEAP_START = heap_start as usize;
        HEAP_SIZE = heap_size;
        
        let start_page = Page::containing_address(VirtAddr::new(heap_start));
        let end_page = Page::containing_address(VirtAddr::new(heap_start + heap_size as u64 - 1));
        
        for page in Page::range_inclusive(start_page, end_page) {
            let frame = x86_64::structures::paging::PhysFrame::containing_address(
                x86_64::PhysAddr::new(page.start_address().as_u64() - boot_info.physical_memory_offset)
            );
            mapper
                .translate_page(page)
                .unwrap_or_else(|_| {
                    mapper.map_to(page, frame, x86_64::PageTableFlags::PRESENT | x86_64::PageTableFlags::WRITABLE)
                        .unwrap()
                        .flush();
                });
        }
        
        HEAP_ALLOCATOR.lock().init(heap_start as *mut u8, heap_size);
    }
    
    log::info!("Heap initialized at {:#x}, size: {} bytes", heap_start, heap_size);
}

/// Allocate a stack for a thread
pub fn allocate_stack(size: usize) -> VirtAddr {
    use alloc::alloc::{alloc, Layout};
    
    let layout = Layout::from_size_align(size, 16).unwrap();
    let ptr = unsafe { alloc(layout) };
    
    if ptr.is_null() {
        panic!("Stack allocation failed");
    }
    
    VirtAddr::new(ptr as u64 + size as u64) // Return top of stack
}

/// Get heap statistics
pub fn get_stats() -> HeapStats {
    let allocator = HEAP_ALLOCATOR.lock();
    let (used, free) = allocator.used_free();
    
    HeapStats {
        used,
        free,
        total: used + free,
    }
}

/// Heap statistics
#[derive(Debug, Clone)]
pub struct HeapStats {
    pub used: usize,
    pub free: usize,
    pub total: usize,
}

impl core::fmt::Display for HeapStats {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Heap: {} KB used, {} KB free, {} KB total",
               self.used / 1024, self.free / 1024, self.total / 1024)
    }
}
