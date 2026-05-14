//! AuroraOS Driver Framework
//!
//! This crate provides the unified driver framework for AuroraOS,
//! including PCI enumeration, device abstraction, and driver management.

#![no_std]

extern crate alloc;

pub mod manager;
pub mod device;
pub mod registry;
pub mod pci;

// Driver implementations
pub mod storage;
pub mod input;
pub mod display;
pub mod network;
pub mod usb;
pub mod audio;
pub mod platform;

use core::fmt;
use alloc::string::String;

/// Driver initialization status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    /// Driver is not loaded
    Unloaded,
    /// Driver is loading
    Loading,
    /// Driver is ready
    Ready,
    /// Driver encountered an error
    Error(DriverError),
}

/// Driver error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverError {
    /// Device not found
    NotFound,
    /// Unsupported device
    Unsupported,
    /// Initialization failed
    InitFailed,
    /// I/O error
    IoError,
    /// Memory allocation failed
    OutOfMemory,
}

impl fmt::Display for DriverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriverError::NotFound => write!(f, "Device not found"),
            DriverError::Unsupported => write!(f, "Unsupported device"),
            DriverError::InitFailed => write!(f, "Initialization failed"),
            DriverError::IoError => write!(f, "I/O error"),
            DriverError::OutOfMemory => write!(f, "Out of memory"),
        }
    }
}

/// Driver trait that all drivers must implement
pub trait Driver: Send + Sync {
    /// Get the driver name
    fn name(&self) -> &'static str;
    
    /// Get the driver version
    fn version(&self) -> u32;
    
    /// Initialize the driver
    fn init(&mut self) -> Result<(), DriverError>;
    
    /// Shutdown the driver
    fn shutdown(&mut self);
    
    /// Check if the driver supports a device
    fn supports(&self, device: &device::Device) -> bool;
    
    /// Bind to a device
    fn bind(&mut self, device: &device::Device) -> Result<(), DriverError>;
    
    /// Unbind from a device
    fn unbind(&mut self, device: &device::Device);
}

/// Marker for production-ready drivers
pub const DRIVER_STATUS_READY: &str = "[READY]";
/// Marker for work-in-progress drivers
pub const DRIVER_STATUS_WIP: &str = "[WIP]";
/// Marker for stub drivers
pub const DRIVER_STATUS_STUB: &str = "[STUB]";
