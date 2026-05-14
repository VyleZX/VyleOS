//! Driver manager module - handles driver loading and lifecycle

use alloc::vec::Vec;
use alloc::boxed::Box;
use crate::{Driver, DriverStatus, DriverError, device::Device};

/// Maximum number of drivers that can be registered
const MAX_DRIVERS: usize = 64;

/// Driver manager state
pub struct DriverManager {
    /// Registered drivers
    drivers: Vec<Box<dyn Driver>>,
    /// Discovered devices
    devices: Vec<Device>,
    /// Device-to-driver mappings
    bindings: Vec<(u64, usize)>, // (device_id, driver_index)
}

impl DriverManager {
    pub const fn new() -> Self {
        Self {
            drivers: Vec::new(),
            devices: Vec::new(),
            bindings: Vec::new(),
        }
    }
    
    /// Register a driver with the manager
    pub fn register_driver(&mut self, driver: Box<dyn Driver>) -> Result<(), DriverError> {
        if self.drivers.len() >= MAX_DRIVERS {
            return Err(DriverError::OutOfMemory);
        }
        
        log::info!("Registering driver: {}", driver.name());
        self.drivers.push(driver);
        Ok(())
    }
    
    /// Add a discovered device
    pub fn add_device(&mut self, device: Device) {
        log::debug!("Device discovered: {} (type: {:?})", device.name, device.device_type);
        self.devices.push(device);
    }
    
    /// Add multiple devices
    pub fn add_devices(&mut self, devices: Vec<Device>) {
        for device in devices {
            self.add_device(device);
        }
    }
    
    /// Match drivers to devices and bind them
    pub fn match_and_bind(&mut self) {
        log::info!("Matching drivers to devices...");
        
        for (device_idx, device) in self.devices.iter_mut().enumerate() {
            if device.status != DriverStatus::Unloaded {
                continue;
            }
            
            // Find a matching driver
            for (driver_idx, driver) in self.drivers.iter_mut().enumerate() {
                if driver.supports(device) {
                    log::info!("Binding driver '{}' to device '{}'", driver.name(), device.name);
                    
                    match driver.bind(device) {
                        Ok(()) => {
                            device.status = DriverStatus::Ready;
                            self.bindings.push((device.id, driver_idx));
                            break;
                        }
                        Err(e) => {
                            log::error!("Failed to bind driver to device: {:?}", e);
                            device.status = DriverStatus::Error(e);
                        }
                    }
                }
            }
            
            if device.status == DriverStatus::Unloaded {
                log::warn!("No driver found for device: {}", device.name);
            }
        }
    }
    
    /// Initialize all bound drivers
    pub fn initialize_drivers(&mut self) {
        log::info!("Initializing drivers...");
        
        for (driver_idx, driver) in self.drivers.iter_mut().enumerate() {
            // Check if this driver has any bound devices
            let has_devices = self.bindings.iter().any(|(_, idx)| *idx == driver_idx);
            
            if has_devices {
                match driver.init() {
                    Ok(()) => {
                        log::info!("Driver '{}' initialized successfully", driver.name());
                    }
                    Err(e) => {
                        log::error!("Failed to initialize driver '{}': {:?}", driver.name(), e);
                    }
                }
            }
        }
    }
    
    /// Get status of a specific device
    pub fn get_device_status(&self, device_id: u64) -> Option<DriverStatus> {
        self.devices.iter()
            .find(|d| d.id == device_id)
            .map(|d| d.status)
    }
    
    /// Get list of all devices
    pub fn get_devices(&self) -> &[Device] {
        &self.devices
    }
    
    /// Get list of all drivers
    pub fn get_drivers(&self) -> &[Box<dyn Driver>] {
        &self.drivers
    }
    
    /// Shutdown all drivers
    pub fn shutdown(&mut self) {
        log::info!("Shutting down all drivers...");
        
        for driver in self.drivers.iter_mut() {
            driver.shutdown();
        }
        
        self.bindings.clear();
        
        for device in self.devices.iter_mut() {
            device.status = DriverStatus::Unloaded;
        }
    }
}

/// Global driver manager instance (to be used by kernel)
static mut GLOBAL_MANAGER: Option<DriverManager> = None;

/// Initialize the global driver manager
pub fn init() {
    unsafe {
        GLOBAL_MANAGER = Some(DriverManager::new());
    }
}

/// Get reference to global driver manager
pub fn get_manager() -> Option<&'static mut DriverManager> {
    unsafe { GLOBAL_MANAGER.as_mut() }
}
