//! PCI enumeration and management module

use alloc::vec::Vec;
use crate::device::{Device, PciInfo, DeviceType};

/// PCI configuration space addresses
const PCI_CONFIG_ADDRESS: u16 = 0xCF8;
const PCI_CONFIG_DATA: u16 = 0xCFC;

/// PCI vendor IDs for known vendors
pub const VENDOR_INTEL: u16 = 0x8086;
pub const VENDOR_AMD: u16 = 0x1022;
pub const VENDOR_NVIDIA: u16 = 0x10DE;
pub const VENDOR_AMD_ATI: u16 = 0x1002;
pub const VENDOR_REALTEK: u16 = 0x10EC;
pub const VENDOR_INTEL_NETWORK: u16 = 0x8086;

/// PCI class codes
pub const CLASS_STORAGE: u8 = 0x01;
pub const CLASS_NETWORK: u8 = 0x02;
pub const CLASS_DISPLAY: u8 = 0x03;
pub const CLASS_AUDIO: u8 = 0x04;
pub const CLASS_USB: u8 = 0x0C;

/// PCI subclass codes for storage
pub const SUBCLASS_STORAGE_IDE: u8 = 0x01;
pub const SUBCLASS_STORAGE_AHCI: u8 = 0x06;
pub const SUBCLASS_STORAGE_NVME: u8 = 0x08;

/// PCI device header type
#[derive(Debug, Clone, Copy)]
pub enum HeaderType {
    EndPoint = 0x00,
    PciToPciBridge = 0x01,
    CardBusBridge = 0x02,
}

/// PCI BAR (Base Address Register) information
#[derive(Debug, Clone)]
pub struct BarInfo {
    pub bar_index: u8,
    pub address: u64,
    pub size: u64,
    pub is_mmio: bool,
    pub is_64bit: bool,
    pub prefetchable: bool,
}

/// PCI controller/bridge representation
pub struct PciController {
    pub domain: u16,
    pub start_bus: u8,
    pub end_bus: u8,
}

impl PciController {
    pub fn new(domain: u16, start_bus: u8, end_bus: u8) -> Self {
        Self { domain, start_bus, end_bus }
    }
}

/// Read a 32-bit value from PCI configuration space
pub fn config_read(bus: u8, device: u8, function: u8, offset: u8) -> u32 {
    #[cfg(feature = "hw_access")]
    {
        use x86_64::instructions::port::Port;
        
        let address = (1 << 31) 
            | ((bus as u32) << 16) 
            | ((device as u32) << 11) 
            | ((function as u32) << 8) 
            | ((offset & 0xFC) as u32);
        
        unsafe {
            let mut addr_port = Port::new(PCI_CONFIG_ADDRESS);
            let mut data_port = Port::new(PCI_CONFIG_DATA);
            
            addr_port.write(address);
            data_port.read::<u32>()
        }
    }
    
    #[cfg(not(feature = "hw_access"))]
    {
        // Stub implementation for non-hardware builds
        0xFFFFFFFF
    }
}

/// Write a 32-bit value to PCI configuration space
pub fn config_write(bus: u8, device: u8, function: u8, offset: u8, value: u32) {
    #[cfg(feature = "hw_access")]
    {
        use x86_64::instructions::port::Port;
        
        let address = (1 << 31) 
            | ((bus as u32) << 16) 
            | ((device as u32) << 11) 
            | ((function as u32) << 8) 
            | ((offset & 0xFC) as u32);
        
        unsafe {
            let mut addr_port = Port::new(PCI_CONFIG_ADDRESS);
            let mut data_port = Port::new(PCI_CONFIG_DATA);
            
            addr_port.write(address);
            data_port.write(value);
        }
    }
    
    #[cfg(not(feature = "hw_access"))]
    {
        // Stub implementation
        let _ = (bus, device, function, offset, value);
    }
}

/// Check if a PCI device exists at the given location
pub fn device_exists(bus: u8, device: u8, function: u8) -> bool {
    let vendor_id = config_read(bus, device, function, 0x00) as u16;
    vendor_id != 0xFFFF
}

/// Enumerate all PCI devices on a bus
pub fn enumerate_bus(bus: u8, devices: &mut Vec<Device>) {
    for device in 0..32 {
        // Check for multi-function devices
        let header_type = (config_read(bus, device, 0, 0x0E) >> 16) as u8 & 0x7F;
        let is_multifunction = (header_type & 0x80) != 0;
        
        if device_exists(bus, device, 0) {
            enumerate_function(bus, device, 0, devices);
        }
        
        if is_multifunction {
            for function in 1..8 {
                if device_exists(bus, device, function) {
                    enumerate_function(bus, device, function, devices);
                }
            }
        }
    }
}

/// Enumerate a single PCI function
fn enumerate_function(bus: u8, device: u8, function: u8, devices: &mut Vec<Device>) {
    let header = config_read(bus, device, function, 0x00);
    let vendor_id = (header & 0xFFFF) as u16;
    let device_id = (header >> 16) as u16;
    
    let header2 = config_read(bus, device, function, 0x08);
    let revision = (header2 & 0xFF) as u8;
    let prog_if = ((header2 >> 8) & 0xFF) as u8;
    let subclass = ((header2 >> 16) & 0xFF) as u8;
    let class_code = ((header2 >> 24) & 0xFF) as u8;
    
    let pci_info = PciInfo::new(
        bus, device, function,
        vendor_id, device_id,
        class_code, subclass, prog_if, revision,
    );
    
    // Generate a unique device ID
    let device_id_unique = ((bus as u64) << 24) 
        | ((device as u64) << 16) 
        | ((function as u64) << 8);
    
    let mut dev = Device::from_pci(device_id_unique, pci_info);
    
    // Read BARs
    for bar_index in 0..6 {
        let bar_offset = 0x10 + (bar_index * 4);
        let bar_value = config_read(bus, device, function, bar_offset);
        
        if bar_value != 0 && bar_value != 0xFFFFFFFF {
            // Determine BAR type and size (simplified)
            let is_mmio = (bar_value & 0x01) == 0;
            let is_64bit = is_mmio && ((bar_value & 0x06) == 0x04);
            let prefetchable = (bar_value & 0x08) != 0;
            
            // In a real implementation, we would determine the actual size
            // by writing all 1s and reading back
            let address = if is_mmio {
                (bar_value & !0xF) as u64
            } else {
                (bar_value & !0x3) as u64
            };
            
            dev.add_resource(crate::device::Resource::Mmio {
                base: address,
                size: 4096, // Placeholder size
            });
        }
    }
    
    devices.push(dev);
}

/// Full PCI enumeration across all buses
pub fn enumerate_all() -> Vec<Device> {
    let mut devices = Vec::new();
    
    // First, check if we have a PCI host bridge
    let header_type = config_read(0, 0, 0, 0x0E) as u8;
    
    if (header_type & 0x80) == 0 {
        // Single host bridge - enumerate bus 0 only
        enumerate_bus(0, &mut devices);
    } else {
        // Multiple host bridges - scan all 256 buses
        for bus in 0..256 {
            enumerate_bus(bus, &mut devices);
        }
    }
    
    devices
}

/// Initialize PCI subsystem
pub fn init() -> Vec<Device> {
    log::info!("Enumerating PCI devices...");
    let devices = enumerate_all();
    log::info!("Found {} PCI devices", devices.len());
    devices
}
