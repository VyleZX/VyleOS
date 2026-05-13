//! Driver module placeholder

use super::{Device, DeviceType, DriverResult};

/// Initialize this driver subsystem
pub fn init() {
    log::info!(concat!("Driver subsystem initialized: ", stringify!($mod)));
}

/// Placeholder device structure
pub struct PlaceholderDevice {
    name: alloc::string::String,
}

impl PlaceholderDevice {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.into(),
        }
    }
}

impl Device for PlaceholderDevice {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn device_type(&self) -> DeviceType {
        DeviceType::Other
    }
    
    fn initialize(&self) -> DriverResult<()> {
        Ok(())
    }
    
    fn is_present(&self) -> bool {
        true
    }
}
