//! Security Subsystem
//!
//! Provides OS security features:
//! - Capability-based access control
//! - Sandboxing
//! - Secure boot verification
//! - Memory protection enforcement

use spin::RwLock;

pub mod capabilities;
pub mod sandbox;

pub use self::capabilities::{Capabilities, Capability};
pub use self::sandbox::Sandbox;

/// Initialize the security subsystem
pub fn init() {
    log::info!("Security subsystem initialized");
}

/// Security context for a process
pub struct SecurityContext {
    /// Process capabilities
    pub capabilities: Capabilities,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Supplementary groups
    pub groups: alloc::vec::Vec<u32>,
    /// SELinux-like security label
    pub label: Option<alloc::string::String>,
}

impl SecurityContext {
    pub fn new(uid: u32, gid: u32) -> Self {
        Self {
            capabilities: Capabilities::default(),
            uid,
            gid,
            groups: alloc::vec::Vec::new(),
            label: None,
        }
    }
    
    pub fn root() -> Self {
        Self {
            capabilities: Capabilities::all(),
            uid: 0,
            gid: 0,
            groups: alloc::vec::Vec::new(),
            label: Some("system_u:system_r:kernel_t:s0".into()),
        }
    }
    
    /// Check if the context has a specific capability
    pub fn has_capability(&self, cap: Capability) -> bool {
        self.capabilities.has(cap)
    }
    
    /// Check if the context is root (uid 0)
    pub fn is_root(&self) -> bool {
        self.uid == 0
    }
}
