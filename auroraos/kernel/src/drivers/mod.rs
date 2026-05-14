//! Driver framework for AuroraOS

pub mod pci;
pub mod device;
pub mod manager;
pub mod registry;

/// Initialize driver subsystem
pub fn init() {
    // Initialize PCI enumeration
    pci::init();
    
    // Initialize device manager
    manager::init();
    
    // Initialize driver registry
    registry::init();
}

/// Driver registration macro
#[macro_export]
macro_rules! register_driver {
    ($name:expr, $probe:expr, $remove:expr) => {
        $crate::drivers::registry::register_driver($crate::drivers::registry::DriverInfo {
            name: $name,
            probe: $probe,
            remove: $remove,
        })
    };
}

/// Device trait that all drivers must implement
pub trait Driver {
    /// Probe function - called when a matching device is found
    fn probe(&self, device: &device::Device) -> Result<(), &'static str>;
    
    /// Remove function - called when device is removed
    fn remove(&self, device: &device::Device);
}

/// Driver information structure
pub struct DriverInfo {
    pub name: &'static str,
    pub probe: fn(&device::Device) -> Result<(), &'static str>,
    pub remove: fn(&device::Device),
}
