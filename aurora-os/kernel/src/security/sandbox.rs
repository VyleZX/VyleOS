//! Process Sandboxing

use super::capabilities::Capabilities;
use alloc::string::String;
use alloc::vec::Vec;

/// Sandbox configuration for process isolation
pub struct Sandbox {
    /// Namespace ID
    pub namespace_id: u32,
    /// Allowed filesystem paths
    pub allowed_paths: Vec<String>,
    /// Denied capabilities
    pub denied_caps: Capabilities,
    /// Network access allowed
    pub network_access: bool,
    /// Maximum memory in bytes
    pub max_memory: Option<u64>,
    /// Maximum CPU percentage (0-100)
    pub max_cpu: Option<u8>,
}

impl Sandbox {
    /// Create a new sandbox with default restrictive settings
    pub fn new(namespace_id: u32) -> Self {
        Self {
            namespace_id,
            allowed_paths: Vec::new(),
            denied_caps: Capabilities::empty(),
            network_access: false,
            max_memory: None,
            max_cpu: None,
        }
    }
    
    /// Create a permissive sandbox (for testing)
    pub fn permissive(namespace_id: u32) -> Self {
        Self {
            namespace_id,
            allowed_paths: vec!["/".into()],
            denied_caps: Capabilities::empty(),
            network_access: true,
            max_memory: None,
            max_cpu: None,
        }
    }
    
    /// Add an allowed path
    pub fn allow_path(&mut self, path: &str) {
        self.allowed_paths.push(path.into());
    }
    
    /// Deny a capability
    pub fn deny_capability(&mut self, cap: super::Capability) {
        self.denied_caps.effective.insert(cap);
        self.denied_caps.permitted.insert(cap);
    }
    
    /// Check if a path is allowed
    pub fn is_path_allowed(&self, path: &str) -> bool {
        if self.allowed_paths.is_empty() {
            return false;
        }
        
        for allowed in &self.allowed_paths {
            if path.starts_with(allowed) {
                return true;
            }
        }
        
        false
    }
    
    /// Check if a capability is denied
    pub fn is_capability_denied(&self, cap: super::Capability) -> bool {
        self.denied_caps.effective.contains(cap)
    }
}
