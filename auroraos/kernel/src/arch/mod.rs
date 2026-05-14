//! Architecture abstraction layer

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

/// Initialize architecture-specific components
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    x86_64::init();
    
    #[cfg(target_arch = "aarch64")]
    aarch64::init();
}

/// Halt the CPU
#[inline(always)]
pub fn halt() {
    #[cfg(target_arch = "x86_64")]
    x86_64::halt();
    
    #[cfg(target_arch = "aarch64")]
    aarch64::halt();
}

/// Reboot the system
pub fn reboot() -> ! {
    #[cfg(target_arch = "x86_64")]
    return x86_64::reboot();
    
    #[cfg(target_arch = "aarch64")]
    return aarch64::reboot();
}

/// Shutdown the system
pub fn shutdown() -> ! {
    #[cfg(target_arch = "x86_64")]
    return x86_64::shutdown();
    
    #[cfg(target_arch = "aarch64")]
    return aarch64::shutdown();
}
