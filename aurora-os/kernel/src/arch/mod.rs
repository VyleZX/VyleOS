//! Architecture-specific code
//! 
//! This module contains architecture-specific implementations.
//! Currently supports x86_64, with ARM64 support planned.

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

#[cfg(target_arch = "aarch64")]
pub mod arm64;

#[cfg(target_arch = "aarch64")]
pub use arm64::*;

use core::sync::atomic::{AtomicBool, Ordering};

/// Global flag indicating if architecture initialization is complete
static ARCH_INITIALIZED: AtomicBool = AtomicBool::new(false);

/// Initialize architecture-specific components
pub fn init() {
    #[cfg(target_arch = "x86_64")]
    x86_64::init();
    
    #[cfg(target_arch = "aarch64")]
    arm64::init();
    
    ARCH_INITIALIZED.store(true, Ordering::SeqCst);
}

/// Halt the CPU - used in idle loops
#[inline]
pub fn halt() {
    #[cfg(target_arch = "x86_64")]
    x86_64::halt();
    
    #[cfg(target_arch = "aarch64")]
    arm64::halt();
}

/// Enable interrupts
#[inline]
pub fn enable_interrupts() {
    #[cfg(target_arch = "x86_64")]
    x86_64::enable_interrupts();
    
    #[cfg(target_arch = "aarch64")]
    arm64::enable_interrupts();
}

/// Disable interrupts
#[inline]
pub fn disable_interrupts() {
    #[cfg(target_arch = "x86_64")]
    x86_64::disable_interrupts();
    
    #[cfg(target_arch = "aarch64")]
    arm64::disable_interrupts();
}

/// Check if interrupts are enabled
#[inline]
pub fn interrupts_enabled() -> bool {
    #[cfg(target_arch = "x86_64")]
    return x86_64::interrupts_enabled();
    
    #[cfg(target_arch = "aarch64")]
    return arm64::interrupts_enabled();
}

/// Get the current CPU ID (for SMP)
#[inline]
pub fn cpu_id() -> usize {
    #[cfg(target_arch = "x86_64")]
    return x86_64::cpu_id();
    
    #[cfg(target_arch = "aarch64")]
    return arm64::cpu_id();
}

/// Memory barrier - ensures ordering of memory operations
#[inline]
pub fn memory_barrier() {
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
}
