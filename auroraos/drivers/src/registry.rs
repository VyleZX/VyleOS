//! Driver registry module - maintains driver database

use alloc::string::String;
use alloc::vec::Vec;
use crate::device::DeviceType;

/// Driver metadata information
#[derive(Debug, Clone)]
pub struct DriverInfo {
    pub name: String,
    pub version: u32,
    pub description: String,
    pub author: String,
    pub supported_devices: Vec<DeviceType>,
    pub status: DriverLoadStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverLoadStatus {
    /// Driver available but not loaded
    Available,
    /// Driver currently loaded
    Loaded,
    /// Driver failed to load
    Failed,
    /// Driver incompatible with current system
    Incompatible,
}

/// Driver registry for managing available drivers
pub struct DriverRegistry {
    drivers: Vec<DriverInfo>,
}

impl DriverRegistry {
    pub const fn new() -> Self {
        Self {
            drivers: Vec::new(),
        }
    }
    
    /// Register a driver in the registry
    pub fn register(&mut self, info: DriverInfo) {
        log::info!("Registering driver: {} v{}.{}.{}", 
            info.name,
            (info.version >> 16) & 0xFF,
            (info.version >> 8) & 0xFF,
            info.version & 0xFF
        );
        self.drivers.push(info);
    }
    
    /// Find drivers that support a given device type
    pub fn find_for_device(&self, device_type: DeviceType) -> Vec<&DriverInfo> {
        self.drivers.iter()
            .filter(|d| d.supported_devices.contains(&device_type))
            .filter(|d| d.status == DriverLoadStatus::Available)
            .collect()
    }
    
    /// Get all registered drivers
    pub fn get_all(&self) -> &[DriverInfo] {
        &self.drivers
    }
    
    /// Update driver load status
    pub fn set_status(&mut self, name: &str, status: DriverLoadStatus) {
        if let Some(driver) = self.drivers.iter_mut().find(|d| d.name == name) {
            driver.status = status;
        }
    }
    
    /// Get count of registered drivers
    pub fn count(&self) -> usize {
        self.drivers.len()
    }
    
    /// Get count of loaded drivers
    pub fn loaded_count(&self) -> usize {
        self.drivers.iter()
            .filter(|d| d.status == DriverLoadStatus::Loaded)
            .count()
    }
}

/// Global driver registry
static mut REGISTRY: Option<DriverRegistry> = None;

/// Initialize the global driver registry
pub fn init() {
    unsafe {
        REGISTRY = Some(DriverRegistry::new());
    }
}

/// Get reference to global registry
pub fn get_registry() -> Option<&'static mut DriverRegistry> {
    unsafe { REGISTRY.as_mut() }
}
