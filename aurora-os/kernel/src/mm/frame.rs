//! Physical Frame Allocator
//!
//! Manages physical memory frames (4KB pages) using a bitmap allocator.

use x86_64::{
    structures::paging::{FrameAllocator, PhysFrame, Size4KiB},
    PhysAddr,
};
use bootloader::BootInfo;
use spin::Mutex;
use bitflags::bitflags;
use core::mem;

/// Maximum number of frames we can track (adjustable based on max RAM)
const MAX_FRAMES: usize = 1024 * 1024; // Support up to 4TB of RAM

/// Bitmap-based frame allocator
pub struct FrameAllocator {
    /// Bitmap tracking allocated frames (1 = allocated, 0 = free)
    bitmap: [u64; MAX_FRAMES / 64],
    /// Total number of frames
    total_frames: usize,
    /// Number of allocated frames
    allocated_frames: usize,
}

impl FrameAllocator {
    /// Create a new frame allocator
    pub const fn new() -> Self {
        Self {
            bitmap: [0; MAX_FRAMES / 64],
            total_frames: 0,
            allocated_frames: 0,
        }
    }
    
    /// Initialize the frame allocator with memory map from bootloader
    pub fn init_from_bootinfo(&mut self, boot_info: &'static BootInfo) {
        let mut frame_count = 0;
        
        // Count usable frames
        for region in boot_info.memory_map.iter() {
            if region.region_type == bootloader::boot_info::MemoryRegionType::Usable {
                let start_frame = region.range.start_addr().as_u64() / Size4KiB::SIZE;
                let end_frame = region.range.end_addr().as_u64() / Size4KiB::SIZE;
                frame_count += (end_frame - start_frame) as usize;
            }
        }
        
        self.total_frames = frame_count.min(MAX_FRAMES);
        
        log::info!("Physical frame allocator initialized with {} frames ({:.2} MB)", 
                   self.total_frames, 
                   (self.total_frames * Size4KiB::SIZE as usize) as f64 / (1024.0 * 1024.0));
    }
    
    /// Mark a frame as allocated
    pub fn allocate(&mut self) -> Option<PhysFrame> {
        for (word_idx, word) in self.bitmap.iter_mut().enumerate() {
            if *word != u64::MAX {
                for bit_idx in 0..64 {
                    if (*word & (1 << bit_idx)) == 0 {
                        *word |= 1 << bit_idx;
                        let frame_num = word_idx * 64 + bit_idx;
                        if frame_num < self.total_frames {
                            self.allocated_frames += 1;
                            let addr = PhysAddr::new((frame_num as u64) * Size4KiB::SIZE);
                            return Some(PhysFrame::from_start_address(addr).unwrap());
                        }
                    }
                }
            }
        }
        None
    }
    
    /// Deallocate a frame
    pub fn deallocate(&mut self, frame: PhysFrame) {
        let frame_num = (frame.start_address().as_u64() / Size4KiB::SIZE) as usize;
        if frame_num < MAX_FRAMES {
            let word_idx = frame_num / 64;
            let bit_idx = frame_num % 64;
            self.bitmap[word_idx] &= !(1 << bit_idx);
            self.allocated_frames -= 1;
        }
    }
    
    /// Get statistics about frame usage
    pub fn stats(&self) -> FrameStats {
        FrameStats {
            total_frames: self.total_frames,
            allocated_frames: self.allocated_frames,
            free_frames: self.total_frames - self.allocated_frames,
            total_memory: self.total_frames as u64 * Size4KiB::SIZE,
            allocated_memory: self.allocated_frames as u64 * Size4KiB::SIZE,
        }
    }
}

/// Statistics about frame allocation
#[derive(Debug, Clone)]
pub struct FrameStats {
    pub total_frames: usize,
    pub allocated_frames: usize,
    pub free_frames: usize,
    pub total_memory: u64,
    pub allocated_memory: u64,
}

impl core::fmt::Display for FrameStats {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Frames: {}/{} allocated, Memory: {:.2} MB / {:.2} MB",
               self.allocated_frames, self.total_frames,
               self.allocated_memory as f64 / (1024.0 * 1024.0),
               self.total_memory as f64 / (1024.0 * 1024.0))
    }
}

/// Global frame allocator instance
static FRAME_ALLOCATOR: Mutex<Option<FrameAllocator>> = Mutex::new(None);

/// Initialize the global frame allocator
pub fn init(boot_info: &'static BootInfo) {
    let mut allocator = FrameAllocator::new();
    allocator.init_from_bootinfo(boot_info);
    *FRAME_ALLOCATOR.lock() = Some(allocator);
}

/// Allocate a physical frame
pub fn allocate_frame() -> Option<PhysFrame> {
    let mut allocator = FRAME_ALLOCATOR.lock();
    allocator.as_mut().and_then(|a| a.allocate())
}

/// Deallocate a physical frame
pub fn deallocate_frame(frame: PhysFrame) {
    let mut allocator = FRAME_ALLOCATOR.lock();
    if let Some(ref mut alloc) = *allocator {
        alloc.deallocate(frame);
    }
}

/// Get frame allocator statistics
pub fn get_stats() -> Option<FrameStats> {
    let allocator = FRAME_ALLOCATOR.lock();
    allocator.as_ref().map(|a| a.stats())
}

// Implement FrameAllocator trait for integration with x86_64 crate
unsafe impl FrameAllocator<Size4KiB> for FrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        self.allocate()
    }
}
