//! Global Descriptor Table implementation

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::PrivilegeLevel;
use lazy_static::lazy_static;
use spin::Mutex;

/// GDT wrapper with cached segment selectors
pub struct GdtWrapper {
    table: GlobalDescriptorTable,
    code_selector: SegmentSelector,
    data_selector: SegmentSelector,
}

impl GdtWrapper {
    /// Create a new GDT with kernel and user segments
    pub fn new() -> Self {
        let mut table = GlobalDescriptorTable::new();
        
        // Kernel code segment
        let code_selector = table.add_entry(Descriptor::kernel_code_segment());
        
        // Kernel data segment
        let data_selector = table.add_entry(Descriptor::kernel_data_segment());
        
        // User code segment
        let _user_code = table.add_entry(Descriptor::UserSegment(
            DescriptorFlags::USER_CODE64.bits(),
        ));
        
        // User data segment
        let _user_data = table.add_entry(Descriptor::UserSegment(
            DescriptorFlags::USER_DATA.bits(),
        ));
        
        // TSS will be added later by each CPU
        
        Self {
            table,
            code_selector,
            data_selector,
        }
    }
    
    /// Load the GDT
    pub fn load(&'static self) {
        self.table.load();
    }
    
    /// Get the kernel code segment selector
    pub fn kernel_code_selector(&self) -> SegmentSelector {
        self.code_selector
    }
    
    /// Get the kernel data segment selector
    pub fn kernel_data_selector(&self) -> SegmentSelector {
        self.data_selector
    }
}

use x86_64::structures::gdt::DescriptorFlags;

lazy_static! {
    static ref GLOBAL_GDT: Mutex<Option<&'static GdtWrapper>> = Mutex::new(None);
}

/// Initialize the global GDT
pub fn init() {
    let gdt = Box::leak(Box::new(GdtWrapper::new()));
    gdt.load();
    
    *GLOBAL_GDT.lock() = Some(gdt);
}

/// Get the global GDT reference
pub fn get_gdt() -> Option<&'static GdtWrapper> {
    *GLOBAL_GDT.lock()
}
