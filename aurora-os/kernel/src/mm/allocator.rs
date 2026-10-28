//! Memory Region Allocator

use x86_64::PhysAddr;
use spin::Mutex;

/// Memory region allocator for physical memory management
pub struct MemoryAllocator {
    regions: Mutex<alloc::vec::Vec<MemoryRegion>>,
}

#[derive(Debug, Clone)]
struct MemoryRegion {
    start: u64,
    end: u64,
    allocated: bool,
}

impl MemoryAllocator {
    pub const fn new() -> Self {
        Self {
            regions: Mutex::new(alloc::vec::Vec::new()),
        }
    }
    
    /// Add a memory region to the allocator
    pub fn add_region(&mut self, start: PhysAddr, end: PhysAddr) {
        let mut regions = self.regions.lock();
        regions.push(MemoryRegion {
            start: start.as_u64(),
            end: end.as_u64(),
            allocated: false,
        });
    }
    
    /// Allocate memory from the heap
    pub fn allocate(&self, size: usize) -> Option<PhysAddr> {
        let mut regions = self.regions.lock();
        
        for region in regions.iter_mut() {
            if !region.allocated && (region.end - region.start) >= size as u64 {
                let addr = PhysAddr::new(region.start);
                region.start += size as u64;
                if region.start >= region.end {
                    region.allocated = true;
                }
                return Some(addr);
            }
        }
        
        None
    }
}
