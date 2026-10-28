//! Shared Memory IPC

use x86_64::{VirtAddr, PhysAddr};
use alloc::sync::Arc;
use spin::Mutex;

/// A shared memory region for IPC
pub struct SharedMemory {
    /// Unique identifier
    id: u32,
    /// Virtual address of the mapping
    addr: VirtAddr,
    /// Physical address (for kernel mappings)
    phys_addr: Option<PhysAddr>,
    /// Size in bytes
    size: usize,
    /// Reference count
    ref_count: Mutex<u32>,
}

impl SharedMemory {
    pub fn new(id: u32, addr: VirtAddr, size: usize) -> Self {
        Self {
            id,
            addr,
            phys_addr: None,
            size,
            ref_count: Mutex::new(1),
        }
    }
    
    pub fn with_phys(id: u32, addr: VirtAddr, phys: PhysAddr, size: usize) -> Self {
        Self {
            id,
            addr,
            phys_addr: Some(phys),
            size,
            ref_count: Mutex::new(1),
        }
    }
    
    pub fn id(&self) -> u32 {
        self.id
    }
    
    pub fn addr(&self) -> VirtAddr {
        self.addr
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
    
    pub fn add_ref(&self) {
        *self.ref_count.lock() += 1;
    }
    
    pub fn release(&self) -> u32 {
        let mut count = self.ref_count.lock();
        *count -= 1;
        *count
    }
}
