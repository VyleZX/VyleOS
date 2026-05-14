//! Device abstraction layer

use bitflags::bitflags;

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Storage,
    Network,
    Display,
    Input,
    Audio,
    Usb,
    Serial,
    Other,
}

/// Device state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceState {
    Uninitialized,
    Ready,
    Active,
    Error,
    Removed,
}

bitflags! {
    /// Device capability flags
    #[derive(Debug, Clone, Copy)]
    pub struct DeviceCaps: u32 {
        const DMA = 1 << 0;
        const INTERRUPT = 1 << 1;
        const HOTPLUG = 1 << 2;
        const POWER_MANAGEMENT = 1 << 3;
    }
}

/// Resource types that devices can have
#[derive(Debug, Clone)]
pub enum Resource {
    Memory { start: u64, end: u64 },
    Io { port: u16, size: u16 },
    Interrupt { irq: u8 },
    Dma { channel: u8 },
}

/// Abstract device representation
pub struct Device {
    pub id: u32,
    pub name: [u8; 32],
    pub device_type: DeviceType,
    pub state: DeviceState,
    pub capabilities: DeviceCaps,
    pub resources: Vec<Resource>,
    pub driver_name: Option<[u8; 32]>,
    pub parent_id: Option<u32>,
    pub children: Vec<u32>,
    
    // PCI-specific info (if applicable)
    pub pci_bus: Option<u8>,
    pub pci_device: Option<u8>,
    pub pci_function: Option<u8>,
    pub vendor_id: Option<u16>,
    pub device_id: Option<u16>,
}

impl Device {
    /// Create a new device
    pub fn new(id: u32, name: &str, device_type: DeviceType) -> Self {
        let mut name_bytes = [0u8; 32];
        let name_slice = name.as_bytes();
        let copy_len = name_slice.len().min(32);
        name_bytes[..copy_len].copy_from_slice(&name_slice[..copy_len]);
        
        Self {
            id,
            name: name_bytes,
            device_type,
            state: DeviceState::Uninitialized,
            capabilities: DeviceCaps::empty(),
            resources: Vec::new(),
            driver_name: None,
            parent_id: None,
            children: Vec::new(),
            pci_bus: None,
            pci_device: None,
            pci_function: None,
            vendor_id: None,
            device_id: None,
        }
    }
    
    /// Add a resource to the device
    pub fn add_resource(&mut self, resource: Resource) {
        self.resources.push(resource);
    }
    
    /// Check if device has a specific capability
    pub fn has_capability(&self, cap: DeviceCaps) -> bool {
        self.capabilities.contains(cap)
    }
    
    /// Set device as ready
    pub fn set_ready(&mut self) {
        self.state = DeviceState::Ready;
    }
    
    /// Set device as active
    pub fn set_active(&mut self) {
        self.state = DeviceState::Active;
    }
    
    /// Set device error state
    pub fn set_error(&mut self) {
        self.state = DeviceState::Error;
    }
}

/// Device handle for userspace
#[derive(Debug, Clone, Copy)]
pub struct DeviceHandle {
    pub device_id: u32,
    pub token: u64,  // Security token
}

/// Open a device
pub fn open_device(device_id: u32) -> Option<DeviceHandle> {
    // Implementation would check permissions and return handle
    let _ = device_id;
    None
}

/// Close a device handle
pub fn close_device(handle: DeviceHandle) {
    let _ = handle;
}

/// Read from a device
pub fn device_read(handle: DeviceHandle, buffer: &mut [u8]) -> Result<usize, &'static str> {
    let _ = (handle, buffer);
    Err("Not implemented")
}

/// Write to a device
pub fn device_write(handle: DeviceHandle, buffer: &[u8]) -> Result<usize, &'static str> {
    let _ = (handle, buffer);
    Err("Not implemented")
}

/// IOCTL for device control
pub fn device_ioctl(handle: DeviceHandle, cmd: u32, arg: u64) -> Result<u64, &'static str> {
    let _ = (handle, cmd, arg);
    Err("Not implemented")
}
