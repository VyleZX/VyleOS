//! Interrupt management for x86_64

use super::idt;
use x86_64::instructions::interrupts;

/// Initialize interrupt handling
pub fn init() {
    // Initialize IDT first
    idt::init();
    
    // Remap PIC interrupts to 0x20-0x2F and 0x28-0x2F
    unsafe {
        remap_pic();
    }
    
    log::info!("Interrupt subsystem initialized");
}

/// Remap the PIC to avoid conflicts with CPU exceptions
unsafe fn remap_pic() {
    use super::ports::Port;
    
    const PIC1_COMMAND: u16 = 0x20;
    const PIC1_DATA: u16 = 0x21;
    const PIC2_COMMAND: u16 = 0xA0;
    const PIC2_DATA: u16 = 0xA1;
    
    const ICW1_INIT: u8 = 0x11;
    const ICW1_ICW4: u8 = 0x01;
    const ICW4_8086: u8 = 0x01;
    const ICW4_AUTO: u8 = 0x02;
    
    let mut pic1_cmd = Port::new(PIC1_COMMAND);
    let mut pic1_data = Port::new(PIC1_DATA);
    let mut pic2_cmd = Port::new(PIC2_COMMAND);
    let mut pic2_data = Port::new(PIC2_DATA);
    
    // Start initialization sequence
    pic1_cmd.write(ICW1_INIT | ICW1_ICW4);
    pic2_cmd.write(ICW1_INIT | ICW1_ICW4);
    
    // Set vector offsets
    pic1_data.write(0x20); // IRQ 0-7 -> interrupts 32-39
    pic2_data.write(0x28); // IRQ 8-15 -> interrupts 40-47
    
    // Configure cascading
    pic1_data.write(4); // Tell master about slave at IRQ2
    pic2_data.write(2); // Tell slave its cascade identity
    
    // Set 8086 mode
    pic1_data.write(ICW4_8086);
    pic2_data.write(ICW4_8086);
    
    // Mask all interrupts initially
    pic1_data.write(0xff);
    pic2_data.write(0xff);
}

/// Enable a specific IRQ line
pub fn enable_irq(irq: u8) {
    use super::ports::Port;
    
    if irq < 8 {
        let mut port = Port::new(0x21);
        unsafe {
            let value = port.read();
            port.write(value & !(1 << irq));
        }
    } else if irq < 16 {
        let mut port = Port::new(0xA1);
        unsafe {
            let value = port.read();
            port.write(value & !(1 << (irq - 8)));
        }
    }
}

/// Disable a specific IRQ line
pub fn disable_irq(irq: u8) {
    use super::ports::Port;
    
    if irq < 8 {
        let mut port = Port::new(0x21);
        unsafe {
            let value = port.read();
            port.write(value | (1 << irq));
        }
    } else if irq < 16 {
        let mut port = Port::new(0xA1);
        unsafe {
            let value = port.read();
            port.write(value | (1 << (irq - 8)));
        }
    }
}

/// Send End of Interrupt signal to PIC
pub fn end_of_interrupt(irq: u8) {
    use super::ports::Port;
    
    if irq >= 8 {
        unsafe { Port::new(0xA0).write(0x20); }
    }
    unsafe { Port::new(0x20).write(0x20); }
}

/// Check if we're in an interrupt context (thread-local, managed by scheduler)
#[inline]
pub fn in_interrupt() -> bool {
    // This would be thread-local state managed by the scheduler
    false
}
