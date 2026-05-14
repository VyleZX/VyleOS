//! PCI/PCIe device enumeration and management

use x86_64::instructions::port::Port;
use spin::Mutex;
use crate::println;

/// PCI configuration space access
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

/// Maximum number of PCI devices we track
const MAX_PCI_DEVICES: usize = 256;

/// List of discovered PCI devices
static PCI_DEVICES: Mutex<[Option<PciDevice>; MAX_PCI_DEVICES]> = 
    Mutex::new([None; MAX_PCI_DEVICES]);

/// Initialize PCI subsystem
pub fn init() {
    println!("Enumerating PCI devices...");
    
    let mut device_count = 0;
    
    // Scan all buses, devices, and functions
    for bus in 0..256 {
        for device in 0..32 {
            for function in 0..8 {
                if let Some(dev) = scan_device(bus, device, function) {
                    if device_count < MAX_PCI_DEVICES {
                        PCI_DEVICES.lock()[device_count] = Some(dev);
                        device_count += 1;
                        
                        println!("  Found PCI device: {:04x}:{:04x}", 
                            PCI_DEVICES.lock()[device_count - 1].unwrap().vendor_id,
                            PCI_DEVICES.lock()[device_count - 1].unwrap().device_id);
                    }
                }
            }
        }
    }
    
    println!("Found {} PCI devices", device_count);
}

/// Scan a single PCI device
fn scan_device(bus: u8, device: u8, function: u8) -> Option<PciDevice> {
    let vendor_id = read_config_u16(bus, device, function, 0);
    
    // 0xFFFF means no device present
    if vendor_id == 0xFFFF {
        return None;
    }
    
    let device_id = read_config_u16(bus, device, function, 2);
    let class_code = read_config_u8(bus, device, function, 0xB);
    let subclass = read_config_u8(bus, device, function, 0xA);
    let prog_if = read_config_u8(bus, device, function, 0x9);
    let revision = read_config_u8(bus, device, function, 0x8);
    let header_type = read_config_u8(bus, device, function, 0xE);
    
    Some(PciDevice {
        bus,
        device,
        function,
        vendor_id,
        device_id,
        class_code,
        subclass,
        prog_if,
        revision,
        header_type,
        bar: [0; 6],
        interrupt_line: 0,
        interrupt_pin: 0,
    })
}

/// Read a 32-bit value from PCI config space
fn read_config_u32(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    let address = ((bus as u32) << 16) | 
                  ((device as u32) << 11) | 
                  ((function as u32) << 8) | 
                  ((offset as u32) & 0xFC) | 
                  0x80000000;
    
    unsafe {
        Port::new(PCI_CONFIG_ADDRESS).write(address);
        Port::new(PCI_CONFIG_DATA).read()
    }
}

/// Read a 16-bit value from PCI config space
fn read_config_u16(bus: u8, device: u8, function: u8, offset: u8) -> u16 {
    let address = ((bus as u32) << 16) | 
                  ((device as u32) << 11) | 
                  ((function as u32) << 8) | 
                  (offset as u32) | 
                  0x80000000;
    
    unsafe {
        Port::new(PCI_CONFIG_ADDRESS).write(address);
        (Port::new(PCI_CONFIG_DATA).read() >> ((offset & 2) * 8)) as u16
    }
}

/// Read an 8-bit value from PCI config space
fn read_config_u8(bus: u8, device: u8, function: u8, offset: u8) -> u8 {
    let address = ((bus as u32) << 16) | 
                  ((device as u32) << 11) | 
                  ((function as u32) << 8) | 
                  (offset as u32) | 
                  0x80000000;
    
    unsafe {
        Port::new(PCI_CONFIG_ADDRESS).write(address);
        (Port::new(PCI_CONFIG_DATA).read() >> ((offset & 3) * 8)) as u8
    }
}

/// Write a 32-bit value to PCI config space
#[allow(dead_code)]
fn write_config_u32(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    let address = ((bus as u32) << 16) | 
                  ((device as u32) << 11) | 
                  ((function as u32) << 8) | 
                  ((offset as u32) & 0xFC) | 
                  0x80000000;
    
    unsafe {
        Port::new(PCI_CONFIG_ADDRESS).write(address);
        Port::new(PCI_CONFIG_DATA).write(value);
    }
}

/// PCI Device representation
#[derive(Debug, Clone, Copy)]
pub struct PciDevice {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
    pub header_type: u8,
    pub bar: [u32; 6],  // Base Address Registers
    pub interrupt_line: u8,
    pub interrupt_pin: u8,
}

impl PciDevice {
    /// Get the IRQ for this device
    pub fn irq(&self) -> u8 {
        self.interrupt_line
    }
    
    /// Check if device is a bridge
    pub fn is_bridge(&self) -> bool {
        self.class_code == 0x06 && self.subclass == 0x04
    }
    
    /// Check if device is a VGA controller
    pub fn is_vga(&self) -> bool {
        self.class_code == 0x03
    }
    
    /// Check if device is a storage controller
    pub fn is_storage(&self) -> bool {
        self.class_code == 0x01
    }
    
    /// Check if device is a network controller
    pub fn is_network(&self) -> bool {
        self.class_code == 0x02
    }
}

/// Iterate over all PCI devices
pub fn iter_devices() -> PciDeviceIter {
    PciDeviceIter {
        index: 0,
    }
}

/// Iterator over PCI devices
pub struct PciDeviceIter {
    index: usize,
}

impl Iterator for PciDeviceIter {
    type Item = PciDevice;
    
    fn next(&mut self) -> Option<Self::Item> {
        while self.index < MAX_PCI_DEVICES {
            if let Some(dev) = PCI_DEVICES.lock()[self.index] {
                self.index += 1;
                return Some(dev);
            }
            self.index += 1;
        }
        None
    }
}
