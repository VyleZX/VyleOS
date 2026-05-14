//! Driver registry

use spin::Mutex;
use super::{DriverInfo, device};

/// Maximum number of registered drivers
const MAX_DRIVERS: usize = 64;

/// Registered drivers
static DRIVERS: Mutex<[Option<DriverInfo>; MAX_DRIVERS]> = 
    Mutex::new([None; MAX_DRIVERS]);

/// Initialize driver registry
pub fn init() {
    // Registry is initialized as empty
}

/// Register a driver
pub fn register_driver(info: DriverInfo) -> Result<(), &'static str> {
    let mut drivers = DRIVERS.lock();
    
    for slot in drivers.iter_mut() {
        if slot.is_none() {
            *slot = Some(info);
            return Ok(());
        }
    }
    
    Err("Driver registry full")
}

/// Find a driver for a device
pub fn find_driver(device: &device::Device) -> Option<&'static DriverInfo> {
    let drivers = DRIVERS.lock();
    
    for driver in drivers.iter() {
        if let Some(d) = driver {
            // Match based on device type and vendor/device IDs
            if let (Some(vid), Some(did)) = (device.vendor_id, device.device_id) {
                // Would check against driver's supported devices
                let _ = (vid, did);
            }
        }
    }
    
    None
}

/// Unregister a driver
pub fn unregister_driver(name: &str) -> Result<(), &'static str> {
    let mut drivers = DRIVERS.lock();
    
    for slot in drivers.iter_mut() {
        if let Some(d) = slot {
            if d.name == name {
                *slot = None;
                return Ok(());
            }
        }
    }
    
    Err("Driver not found")
}

/// List all registered drivers
pub fn list_drivers() -> Vec<&'static str> {
    let drivers = DRIVERS.lock();
    let mut names = Vec::new();
    
    for driver in drivers.iter() {
        if let Some(d) = driver {
            names.push(d.name);
        }
    }
    
    names
}
