//! Global Descriptor Table implementation

use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::instructions::segmentation::{CS, DS, ES, FS, GS, SS};
use x86_64::PrivilegeLevel;
use spin::Once;

static GDT: Once<GlobalDescriptorTable> = Once::new();

/// Segment selectors for kernel and user mode
pub struct Selectors {
    pub code_selector: SegmentSelector,
    pub data_selector: SegmentSelector,
    pub user_code_selector: SegmentSelector,
    pub user_data_selector: SegmentSelector,
}

static SELECTORS: Once<Selectors> = Once::new();

/// Initialize the GDT
pub fn init() {
    let mut gdt = GlobalDescriptorTable::new();
    
    // Kernel code segment (ring 0)
    let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
    
    // Kernel data segment (ring 0)
    let data_selector = gdt.add_entry(Descriptor::kernel_data_segment());
    
    // User code segment (ring 3)
    let user_code_selector = gdt.add_entry(Descriptor::user_code_segment());
    
    // User data segment (ring 3)
    let user_data_selector = gdt.add_entry(Descriptor::user_data_segment());
    
    // TSS selector will be added later when we implement task switching
    
    let selectors = Selectors {
        code_selector,
        data_selector,
        user_code_selector,
        user_data_selector,
    };
    
    SELECTORS.call_once(|| selectors);
    let gdt_ref = GDT.call_once(|| gdt);
    gdt_ref.load();
    
    // Load segment registers
    unsafe {
        CS::set_reg(code_selector);
        DS::set_reg(data_selector);
        ES::set_reg(data_selector);
        SS::set_reg(data_selector);
        // FS and GS are set to null initially
    }
}

/// Get the kernel code selector
pub fn kernel_code_selector() -> SegmentSelector {
    SELECTORS.get().unwrap().code_selector
}

/// Get the kernel data selector
pub fn kernel_data_selector() -> SegmentSelector {
    SELECTORS.get().unwrap().data_selector
}

/// Get the user code selector
pub fn user_code_selector() -> SegmentSelector {
    SELECTORS.get().unwrap().user_code_selector
}

/// Get the user data selector
pub fn user_data_selector() -> SegmentSelector {
    SELECTORS.get().unwrap().user_data_selector
}
