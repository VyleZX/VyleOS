//! Filesystem Implementations
//!
//! This module contains implementations for various filesystem types.

use alloc::sync::Arc;
use crate::vfs::{File, VfsResult};

pub mod tmpfs;

/// Filesystem trait - implemented by all filesystem types
pub trait Filesystem: Send + Sync {
    /// Open a file or directory at the specified path
    fn open(&self, path: &str) -> VfsResult<Arc<dyn File>>;
    
    /// Get filesystem statistics
    fn statfs(&self) -> VfsResult<FsStats>;
    
    /// Sync all pending writes
    fn sync(&self) -> VfsResult<()>;
    
    /// Get the filesystem type name
    fn fs_type(&self) -> &'static str;
}

/// Filesystem statistics
#[derive(Debug, Clone)]
pub struct FsStats {
    pub block_size: u64,
    pub total_blocks: u64,
    pub free_blocks: u64,
    pub total_inodes: u64,
    pub free_inodes: u64,
    pub fs_type: &'static str,
}

impl core::fmt::Display for FsStats {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Filesystem: {}\n", self.fs_type)?;
        write!(f, "Block size: {} bytes\n", self.block_size)?;
        write!(f, "Total blocks: {}, Free: {}\n", self.total_blocks, self.free_blocks)?;
        write!(f, "Total inodes: {}, Free: {}", self.total_inodes, self.free_inodes)
    }
}
