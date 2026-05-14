//! Paging and virtual memory management

use x86_64::structures::paging::{
    Page, PageTable, PageTableFlags, Size4KiB, FrameAllocator, Mapper, OffsetPageTable, PhysFrame,
};
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use spin::Once;

/// Kernel page table mapper
static mut MAPPER: Once<OffsetPageTable<'static>> = Once::new();

/// Initialize paging subsystem
pub fn init() {
    // Paging is typically initialized by the bootloader
    // This function sets up our internal structures
    
    unsafe {
        let level_4_table = active_level_4_table();
        MAPPER.call_once(|| {
            OffsetPageTable::new(level_4_table, VirtAddr::new(0))
        });
    }
}

/// Get a mutable reference to the active level 4 page table
unsafe fn active_level_4_table() -> &'static mut PageTable {
    use x86_64::registers::control::Cr3;
    
    let (level_4_frame, _) = Cr3::read();
    let phys = level_4_frame.start_address();
    let virt = phys;
    let page_table_ptr = virt.as_mut_ptr();
    
    &mut *page_table_ptr
}

/// Get the kernel mapper
pub fn mapper() -> &'static mut OffsetPageTable<'static> {
    unsafe { MAPPER.get_mut().unwrap() }
}

/// Identity map a physical frame
pub fn identity_map(frame: PhysFrame<Size4KiB>, flags: PageTableFlags) -> Result<(), &'static str> {
    let mapper = mapper();
    let page = Page::containing_address(VirtAddr::new(frame.start_address().as_u64()));
    
    unsafe {
        mapper
            .map_to(page, frame, flags, &mut BootFrameAllocator)
            .map_err(|_| "Failed to identity map frame")?
            .flush();
    }
    
    Ok(())
}

/// Map a virtual page to a physical frame
pub fn map_page(
    page: Page<Size4KiB>,
    frame: PhysFrame<Size4KiB>,
    flags: PageTableFlags,
) -> Result<(), &'static str> {
    let mapper = mapper();
    
    unsafe {
        mapper
            .map_to(page, frame, flags, &mut BootFrameAllocator)
            .map_err(|_| "Failed to map page")?
            .flush();
    }
    
    Ok(())
}

/// Translate a virtual address to physical
pub fn translate(virt: VirtAddr) -> Option<PhysAddr> {
    let mapper = mapper();
    mapper.translate_addr(virt)
}

/// Boot time frame allocator (used during initialization)
struct BootFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        // During boot, we can't allocate new frames
        // This is only used for mapping pages that are already allocated
        None
    }
}

/// Page table flags for common mappings
pub mod flags {
    use super::PageTableFlags;
    
    /// Present flag
    pub const PRESENT: PageTableFlags = PageTableFlags::PRESENT;
    
    /// Writable flag
    pub const WRITABLE: PageTableFlags = PageTableFlags::WRITABLE;
    
    /// User accessible flag
    pub const USER_ACCESSIBLE: PageTableFlags = PageTableFlags::USER_ACCESSIBLE;
    
    /// No execute flag
    pub const NO_EXECUTE: PageTableFlags = PageTableFlags::NO_EXECUTE;
    
    /// Write through flag
    pub const WRITE_THROUGH: PageTableFlags = PageTableFlags::WRITE_THROUGH;
    
    /// Cache disabled flag
    pub const CACHE_DISABLED: PageTableFlags = PageTableFlags::NO_CACHE;
    
    /// Kernel code flags (present, no write, no execute)
    pub const KERNEL_CODE: PageTableFlags = PageTableFlags::PRESENT;
    
    /// Kernel data flags (present, writable)
    pub const KERNEL_DATA: PageTableFlags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    
    /// User code flags (present, user accessible, no execute)
    pub const USER_CODE: PageTableFlags = 
        PageTableFlags::PRESENT | PageTableFlags::USER_ACCESSIBLE | PageTableFlags::NO_EXECUTE;
    
    /// User data flags (present, writable, user accessible)
    pub const USER_DATA: PageTableFlags = 
        PageTableFlags::PRESENT | PageTableFlags::WRITABLE | PageTableFlags::USER_ACCESSIBLE;
}
