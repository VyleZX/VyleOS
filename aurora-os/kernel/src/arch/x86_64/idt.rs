//! Interrupt Descriptor Table implementation

use x86_64::structures::idt::{InterruptDescriptorTable, InterruptHandler, ExceptionStackFrame};
use x86_64::instructions::segmentation::{Segment, CS};
use lazy_static::lazy_static;
use spin::Mutex;

/// Aurora OS IDT wrapper
pub struct IdtWrapper {
    table: InterruptDescriptorTable,
}

impl IdtWrapper {
    /// Create a new IDT with all handlers
    pub fn new() -> Self {
        let mut table = InterruptDescriptorTable::new();
        
        // CPU Exceptions
        table.division_error.set_handler_fn(division_error_handler);
        table.debug.set_handler_fn(debug_handler);
        table.non_maskable_interrupt.set_handler_fn(nmi_handler);
        table.breakpoint.set_handler_fn(breakpoint_handler);
        table.overflow.set_handler_fn(overflow_handler);
        table.bound_range_exceeded.set_handler_fn(bound_range_handler);
        table.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        table.device_not_available.set_handler_fn(device_not_available_handler);
        table.double_fault.set_handler_fn(double_fault_handler);
        table.invalid_tss.set_handler_fn(invalid_tss_handler);
        table.segment_not_present.set_handler_fn(segment_not_present_handler);
        table.stack_segment_fault.set_handler_fn(stack_segment_fault_handler);
        table.general_protection_fault.set_handler_fn(general_protection_handler);
        table.page_fault.set_handler_fn(page_fault_handler);
        table.x87_floating_point.set_handler_fn(fpu_handler);
        table.alignment_check.set_handler_fn(alignment_check_handler);
        table.machine_check.set_handler_fn(machine_check_handler);
        table.simd_floating_point.set_handler_fn(simd_fpu_handler);
        table.virtualization.set_handler_fn(virtualization_handler);
        table.security_exception.set_handler_fn(security_exception_handler);
        
        // Hardware interrupts (IRQ 0-15)
        table[32].set_handler_fn(timer_handler);
        table[33].set_handler_fn(keyboard_handler);
        table[34].set_handler_fn(cascade_handler);
        table[35].set_handler_fn(com2_handler);
        table[36].set_handler_fn(com1_handler);
        table[37].set_handler_fn(lpt2_handler);
        table[38].set_handler_fn(floppy_handler);
        table[39].set_handler_fn(lpt1_handler);
        table[40].set_handler_fn(rtc_handler);
        table[41].set_handler_fn(pci1_handler);
        table[42].set_handler_fn(pci2_handler);
        table[43].set_handler_fn(mouse_handler);
        table[44].set_handler_fn(coprocessor_handler);
        table[45].set_handler_fn(ide1_handler);
        table[46].set_handler_fn(ide2_handler);
        
        Self { table }
    }
    
    /// Load the IDT
    pub fn load(&'static self) {
        self.table.load();
    }
}

// Exception handlers
extern "x86-interrupt" fn division_error_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Division Error");
}

extern "x86-interrupt" fn debug_handler(_stack_frame: InterruptStackFrame) {
    log::debug!("EXCEPTION: Debug");
}

extern "x86-interrupt" fn nmi_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Non-Maskable Interrupt");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log::debug!("EXCEPTION: Breakpoint at {:#x}", stack_frame.instruction_pointer);
}

extern "x86-interrupt" fn overflow_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Overflow");
}

extern "x86-interrupt" fn bound_range_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Bound Range Exceeded");
}

extern "x86-interrupt" fn invalid_opcode_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Invalid Opcode at {:#x}", _stack_frame.instruction_pointer);
}

extern "x86-interrupt" fn device_not_available_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Device Not Available");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, 
    _error_code: u64
) -> ! {
    panic!("EXCEPTION: Double Fault\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn invalid_tss_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Invalid TSS, error code: {:#x}", error_code);
}

extern "x86-interrupt" fn segment_not_present_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Segment Not Present, error code: {:#x}", error_code);
}

extern "x86-interrupt" fn stack_segment_fault_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Stack-Segment Fault, error code: {:#x}", error_code);
}

extern "x86-interrupt" fn general_protection_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: General Protection Fault, error code: {:#x}", error_code);
}

extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame, 
    error_code: x86_64::structures::idt::PageFaultErrorCode
) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    panic!("EXCEPTION: Page Fault at {:#x}, error code: {:?}", addr, error_code);
}

extern "x86-interrupt" fn fpu_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: x87 Floating Point");
}

extern "x86-interrupt" fn alignment_check_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Alignment Check, error code: {:#x}", error_code);
}

extern "x86-interrupt" fn machine_check_handler(_stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: Machine Check");
}

extern "x86-interrupt" fn simd_fpu_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SIMD Floating Point");
}

extern "x86-interrupt" fn virtualization_handler(_stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: Virtualization");
}

extern "x86-interrupt" fn security_exception_handler(_stack_frame: InterruptStackFrame, error_code: u64) {
    panic!("EXCEPTION: Security Exception, error code: {:#x}", error_code);
}

// Hardware interrupt handlers
extern "x86-interrupt" fn timer_handler(_stack_frame: InterruptStackFrame) {
    // Handled by APIC or scheduler
    log::trace!("Timer interrupt");
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: InterruptStackFrame) {
    // Handled by input driver
    log::trace!("Keyboard interrupt");
}

extern "x86-interrupt" fn cascade_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("Cascade interrupt");
}

extern "x86-interrupt" fn com2_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("COM2 interrupt");
}

extern "x86-interrupt" fn com1_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("COM1 interrupt");
}

extern "x86-interrupt" fn lpt2_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("LPT2 interrupt");
}

extern "x86-interrupt" fn floppy_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("Floppy interrupt");
}

extern "x86-interrupt" fn lpt1_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("LPT1 interrupt");
}

extern "x86-interrupt" fn rtc_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("RTC interrupt");
}

extern "x86-interrupt" fn pci1_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("PCI1 interrupt");
}

extern "x86-interrupt" fn pci2_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("PCI2 interrupt");
}

extern "x86-interrupt" fn mouse_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("Mouse interrupt");
}

extern "x86-interrupt" fn coprocessor_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("Coprocessor interrupt");
}

extern "x86-interrupt" fn ide1_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("IDE1 interrupt");
}

extern "x86-interrupt" fn ide2_handler(_stack_frame: InterruptStackFrame) {
    log::trace!("IDE2 interrupt");
}

lazy_static! {
    static ref GLOBAL_IDT: Mutex<Option<&'static IdtWrapper>> = Mutex::new(None);
}

/// Initialize the global IDT
pub fn init() {
    let idt = Box::leak(Box::new(IdtWrapper::new()));
    idt.load();
    
    *GLOBAL_IDT.lock() = Some(idt);
}

/// Get the global IDT reference
pub fn get_idt() -> Option<&'static IdtWrapper> {
    *GLOBAL_IDT.lock()
}
