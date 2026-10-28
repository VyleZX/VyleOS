//! Page Table Management

use x86_64::structures::paging::{PageTable, PageTableFlags};

/// Wrapper around x86_64 page table with additional functionality
pub struct PageTable {
    inner: &'static mut PageTable,
}

impl PageTable {
    pub fn new(table: &'static mut PageTable) -> Self {
        Self { inner: table }
    }
    
    /// Get a reference to the underlying page table
    pub fn as_mut(&mut self) -> &mut PageTable {
        self.inner
    }
    
    /// Map a page with specified flags
    pub fn map(&mut self, page_idx: usize, frame_addr: u64, flags: PageTableFlags) {
        let entry = &mut self.inner[page_idx];
        entry.set_addr(x86_64::PhysAddr::new(frame_addr), flags);
    }
    
    /// Unmap a page
    pub fn unmap(&mut self, page_idx: usize) {
        let entry = &mut self.inner[page_idx];
        entry.set_flags(PageTableFlags::empty());
    }
    
    /// Check if a page is mapped
    pub fn is_mapped(&self, page_idx: usize) -> bool {
        self.inner[page_idx].flags().contains(PageTableFlags::PRESENT)
    }
    
    /// Get the physical address for a page
    pub fn get_frame(&self, page_idx: usize) -> Option<u64> {
        if self.is_mapped(page_idx) {
            Some(self.inner[page_idx].addr().as_u64())
        } else {
            None
        }
    }
}
