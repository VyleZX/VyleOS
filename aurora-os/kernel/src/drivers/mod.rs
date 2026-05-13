//! Device Driver Framework
//!
//! Provides a unified framework for device drivers:
//! - Storage drivers (SATA, AHCI, NVMe, USB)
//! - Graphics drivers (VGA, Framebuffer, GPU)
//! - Input drivers (USB HID, Keyboard, Mouse)
//! - Audio drivers (Intel HDA)
//! - Network drivers (Ethernet, Wi-Fi)
//! - USB subsystem
//! - Power management

use alloc::sync::Arc;
use spin::RwLock;

pub mod storage;
pub mod graphics;
pub mod input;
pub mod audio;
pub mod network;
pub mod usb;
pub mod power;

/// Initialize all device drivers
pub fn init() {
    log::info!("Initializing device drivers...");
    
    // Initialize USB subsystem first (needed for many devices)
    usb::init();
    
    // Initialize storage drivers
    storage::init();
    
    // Initialize graphics
    graphics::init();
    
    // Initialize input devices
    input::init();
    
    // Initialize audio
    audio::init();
    
    // Initialize network
    network::init();
    
    // Initialize power management
    power::init();
    
    log::info!("Device drivers initialized");
}

/// Device trait - base interface for all devices
pub trait Device: Send + Sync {
    /// Get device name
    fn name(&self) -> &str;
    
    /// Get device type
    fn device_type(&self) -> DeviceType;
    
    /// Initialize the device
    fn initialize(&self) -> DriverResult<()>;
    
    /// Check if device is present and working
    fn is_present(&self) -> bool;
}

/// Device type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
    Storage,
    Graphics,
    Input,
    Audio,
    Network,
    USB,
    Power,
    Other,
}

/// Driver error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    NotFound,
    NotSupported,
    IoError,
    Timeout,
    InvalidConfig,
    OutOfMemory,
    Busy,
}

pub type DriverResult<T> = Result<T, DriverError>;

/// Device registry for managing all discovered devices
pub struct DeviceRegistry {
    devices: RwLock<alloc::collections::BTreeMap<String, Arc<dyn Device>>>,
}

impl DeviceRegistry {
    pub const fn new() -> Self {
        Self {
            devices: RwLock::new(alloc::collections::BTreeMap::new()),
        }
    }
    
    pub fn register(&self, name: String, device: Arc<dyn Device>) {
        self.devices.write().insert(name, device);
    }
    
    pub fn get(&self, name: &str) -> Option<Arc<dyn Device>> {
        self.devices.read().get(name).cloned()
    }
    
    pub fn list(&self) -> alloc::vec::Vec<String> {
        self.devices.read().keys().cloned().collect()
    }
}

static DEVICE_REGISTRY: RwLock<Option<DeviceRegistry>> = RwLock::new(None);

/// Get the global device registry
pub fn get_registry() -> Option<impl core::ops::Deref<Target = DeviceRegistry>> {
    DEVICE_REGISTRY.read().map(|r| r)
}
