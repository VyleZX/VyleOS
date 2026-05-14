//! Device abstraction module for AuroraOS driver framework

use alloc::string::String;
use crate::{DriverError, DriverStatus};

/// Device types supported by the driver framework
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    /// Storage device (HDD, SSD, NVMe)
    Storage,
    /// Input device (keyboard, mouse)
    Input,
    /// Display/GPU device
    Display,
    /// Network interface
    Network,
    /// USB controller or device
    Usb,
    /// Audio device
    Audio,
    /// Platform/device-specific
    Platform,
    /// Unknown device type
    Unknown,
}

/// PCI device information
#[derive(Debug, Clone)]
pub struct PciInfo {
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub vendor_id: u16,
    pub device_id: u16,
    pub class_code: u8,
    pub subclass: u8,
    pub prog_if: u8,
    pub revision: u8,
}

impl PciInfo {
    pub fn new(
        bus: u8,
        device: u8,
        function: u8,
        vendor_id: u16,
        device_id: u16,
        class_code: u8,
        subclass: u8,
        prog_if: u8,
        revision: u8,
    ) -> Self {
        Self {
            bus,
            device,
            function,
            vendor_id,
            device_id,
            class_code,
            subclass,
            prog_if,
            revision,
        }
    }
    
    /// Get device type from PCI class code
    pub fn device_type(&self) -> DeviceType {
        match self.class_code {
            0x01 => DeviceType::Storage,
            0x03 => DeviceType::Display,
            0x02 => DeviceType::Network,
            0x0C => DeviceType::Usb,
            0x04 => DeviceType::Audio,
            _ => DeviceType::Unknown,
        }
    }
}

/// Hardware resource descriptor
#[derive(Debug, Clone)]
pub enum Resource {
    /// Memory-mapped I/O region
    Mmio { base: u64, size: usize },
    /// Port-mapped I/O region
    Pio { base: u16, size: usize },
    /// Interrupt request line
    Irq { irq: u32 },
    /// Message Signaled Interrupt
    Msi { address: u64, data: u32 },
    /// DMA channel
    Dma { channel: u8 },
}

/// Abstract device representation
pub struct Device {
    /// Unique device identifier
    pub id: u64,
    /// Device name
    pub name: String,
    /// Device type
    pub device_type: DeviceType,
    /// PCI information (if applicable)
    pub pci_info: Option<PciInfo>,
    /// Hardware resources
    pub resources: alloc::vec::Vec<Resource>,
    /// Driver status
    pub status: DriverStatus,
    /// Parent device (for USB hierarchies, etc.)
    pub parent: Option<u64>,
}

impl Device {
    pub fn new(id: u64, name: String, device_type: DeviceType) -> Self {
        Self {
            id,
            name,
            device_type,
            pci_info: None,
            resources: alloc::vec::Vec::new(),
            status: DriverStatus::Unloaded,
            parent: None,
        }
    }
    
    pub fn from_pci(id: u64, pci_info: PciInfo) -> Self {
        let device_type = pci_info.device_type();
        let name = format!("PCI {:02x}:{:02x}.{:02x}", 
            pci_info.bus, pci_info.device, pci_info.function);
        
        Self {
            id,
            name,
            device_type,
            pci_info: Some(pci_info),
            resources: alloc::vec::Vec::new(),
            status: DriverStatus::Unloaded,
            parent: None,
        }
    }
    
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
}
