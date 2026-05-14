//! Interrupt Descriptor Table implementation

use x86_64::structures::idt::{InterruptDescriptorTable, Entry, HandlerFunc};
use x86_64::instructions::interrupts;
use spin::Once;
use crate::println;

static IDT: Once<InterruptDescriptorTable> = Once::new();

/// Initialize the IDT with all interrupt handlers
pub fn init() {
    let mut idt = InterruptDescriptorTable::new();
    
    // CPU exceptions (0-31)
    idt.divide_error.set_handler_fn(divide_error_handler);
    idt.debug.set_handler_fn(debug_handler);
    idt.non_maskable_interrupt.set_handler_fn(nmi_handler);
    idt.breakpoint.set_handler_fn(breakpoint_handler);
    idt.overflow.set_handler_fn(overflow_handler);
    idt.bound_range_exceeded.set_handler_fn(bound_handler);
    idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
    idt.device_not_available.set_handler_fn(device_not_available_handler);
    idt.double_fault.set_handler_fn(double_fault_handler);
    idt.invalid_tss.set_handler_fn(invalid_tss_handler);
    idt.segment_not_present.set_handler_fn(segment_not_present_handler);
    idt.stack_segment_fault.set_handler_fn(stack_fault_handler);
    idt.general_protection_fault.set_handler_fn(gpf_handler);
    idt.page_fault.set_handler_fn(page_fault_handler);
    idt.x87_floating_point.set_handler_fn(fp_error_handler);
    idt.alignment_check.set_handler_fn(alignment_check_handler);
    idt.machine_check.set_handler_fn(machine_check_handler);
    idt.simd_floating_point.set_handler_fn(simd_fp_handler);
    idt.virtualization.set_handler_fn(virtualization_handler);
    
    // Hardware interrupts (32-47) - IRQ 0-15
    idt[32].set_handler_fn(timer_handler);      // IRQ 0: Timer
    idt[33].set_handler_fn(keyboard_handler);   // IRQ 1: Keyboard
    idt[34].set_handler_fn(cascade_handler);    // IRQ 2: Cascade
    idt[35].set_handler_fn(com2_handler);       // IRQ 3: COM2
    idt[36].set_handler_fn(com1_handler);       // IRQ 4: COM1
    idt[37].set_handler_fn(lpt2_handler);       // IRQ 5: LPT2
    idt[38].set_handler_fn(floppy_handler);     // IRQ 6: Floppy
    idt[39].set_handler_fn(lpt1_handler);       // IRQ 7: LPT1/Spurious
    idt[40].set_handler_fn(rtc_handler);        // IRQ 8: RTC
    idt[41].set_handler_fn(pci1_handler);       // IRQ 9: PCI 1
    idt[42].set_handler_fn(pci2_handler);       // IRQ 10: PCI 2
    idt[43].set_handler_fn(pci3_handler);       // IRQ 11: PCI 3
    idt[44].set_handler_fn(mouse_handler);      // IRQ 12: PS/2 Mouse
    idt[45].set_handler_fn(fpu_handler);        // IRQ 13: FPU
    idt[46].set_handler_fn(ata1_handler);       // IRQ 14: ATA Primary
    idt[47].set_handler_fn(ata2_handler);       // IRQ 15: ATA Secondary
    
    // Software interrupt for syscalls
    idt[0x80].set_handler_fn(syscall_handler);
    
    IDT.call_once(|| idt).load();
}

// Exception handlers
extern "x86-interrupt" fn divide_error_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Divide by zero error");
}

extern "x86-interrupt" fn debug_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    println!("Debug exception");
}

extern "x86-interrupt" fn nmi_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Non-maskable interrupt");
}

extern "x86-interrupt" fn breakpoint_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    println!("Breakpoint");
}

extern "x86-interrupt" fn overflow_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Overflow");
}

extern "x86-interrupt" fn bound_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Bound range exceeded");
}

extern "x86-interrupt" fn invalid_opcode_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Invalid opcode");
}

extern "x86-interrupt" fn device_not_available_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Device not available");
}

extern "x86-interrupt" fn double_fault_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) -> ! {
    panic!("Double fault");
}

extern "x86-interrupt" fn invalid_tss_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Invalid TSS");
}

extern "x86-interrupt" fn segment_not_present_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Segment not present");
}

extern "x86-interrupt" fn stack_fault_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Stack segment fault");
}

extern "x86-interrupt" fn gpf_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("General protection fault");
}

extern "x86-interrupt" fn page_fault_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame, _error_code: x86_64::structures::idt::PageFaultErrorCode) {
    use x86_64::registers::control::Cr2;
    let addr = Cr2::read();
    panic!("Page fault at address {:p}", addr);
}

extern "x86-interrupt" fn fp_error_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("x87 floating point error");
}

extern "x86-interrupt" fn alignment_check_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Alignment check");
}

extern "x86-interrupt" fn machine_check_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) -> ! {
    panic!("Machine check");
}

extern "x86-interrupt" fn simd_fp_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("SIMD floating point error");
}

extern "x86-interrupt" fn virtualization_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    panic!("Virtualization exception");
}

// Hardware interrupt handlers
extern "x86-interrupt" fn timer_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    // Notify PIC that interrupt is handled
    unsafe {
        x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20);
    }
    // Schedule next tick
    crate::scheduler::tick();
}

extern "x86-interrupt" fn keyboard_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe {
        x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20);
        x86_64::instructions::port::Port::<u8>::new(0x60).read(); // Read and discard scancode
    }
}

extern "x86-interrupt" fn cascade_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn com2_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn com1_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn lpt2_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn floppy_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn lpt1_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn rtc_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn pci1_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn pci2_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn pci3_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn mouse_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn fpu_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn ata1_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

extern "x86-interrupt" fn ata2_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    unsafe { x86_64::instructions::port::Port::<u8>::new(0x20).write(0x20); }
}

// Syscall handler
extern "x86-interrupt" fn syscall_handler(_stack_frame: &mut x86_64::structures::idt::InterruptStackFrame) {
    // Handle system call
    crate::syscall::handle_syscall();
}
