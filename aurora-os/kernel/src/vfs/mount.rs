//! Mount point management

use alloc::sync::Arc;
use alloc::string::String;
use super::{File, VfsResult};

/// A mount point represents a filesystem mounted at a specific path
pub struct MountPoint {
    /// The path where this filesystem is mounted
    pub path: String,
    /// The filesystem implementation
    pub filesystem: Arc<dyn crate::fs::Filesystem>,
}

impl MountPoint {
    /// Create a new mount point
    pub fn new(path: String, filesystem: Arc<dyn crate::fs::Filesystem>) -> Self {
        Self { path, filesystem }
    }
    
    /// Get the root directory of the mounted filesystem
    pub fn root(&self) -> VfsResult<Arc<dyn File>> {
        self.filesystem.open("/")
    }
}

impl core::fmt::Debug for MountPoint {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MountPoint")
            .field("path", &self.path)
            .finish()
    }
}
