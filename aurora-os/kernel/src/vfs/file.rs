//! File trait and related types

use alloc::sync::Arc;
use alloc::string::String;
use alloc::vec::Vec;

/// File type enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    RegularFile,
    Directory,
    Symlink,
    BlockDevice,
    CharacterDevice,
    Fifo,
    Socket,
}

/// File permissions (Unix-style)
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_exec: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_exec: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_exec: bool,
}

impl Permissions {
    pub const fn new() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_exec: false,
            group_read: true,
            group_write: false,
            group_exec: false,
            other_read: true,
            other_write: false,
            other_exec: false,
        }
    }
    
    pub fn writable(&self) -> bool {
        self.owner_write || self.group_write || self.other_write
    }
    
    pub fn readable(&self) -> bool {
        self.owner_read || self.group_read || self.other_read
    }
}

impl Default for Permissions {
    fn default() -> Self {
        Self::new()
    }
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub file_type: FileType,
    pub size: u64,
    pub permissions: Permissions,
    pub inode: u64,
    pub created_at: u64,
    pub modified_at: u64,
    pub accessed_at: u64,
    pub uid: u32,
    pub gid: u32,
    pub link_count: u32,
}

impl FileMetadata {
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::RegularFile
    }
    
    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }
    
    pub fn is_symlink(&self) -> bool {
        self.file_type == FileType::Symlink
    }
}

/// File trait - implemented by all file types
pub trait File: Send + Sync {
    /// Read data from the file at the specified offset
    fn read_at(&self, buf: &mut [u8], offset: u64) -> super::VfsResult<usize>;
    
    /// Write data to the file at the specified offset
    fn write_at(&self, buf: &[u8], offset: u64) -> super::VfsResult<usize>;
    
    /// Get file metadata
    fn metadata(&self) -> super::VfsResult<FileMetadata>;
    
    /// Set file metadata (partial updates)
    fn set_metadata(&self, metadata: &FileMetadata) -> super::VfsResult<()> {
        let _ = metadata;
        Err(super::VfsError::IoError)
    }
    
    /// Flush pending writes
    fn flush(&self) -> super::VfsResult<()> {
        Ok(())
    }
    
    /// Truncate file to specified size
    fn truncate(&self, size: u64) -> super::VfsResult<()> {
        let _ = size;
        Err(super::VfsError::IoError)
    }
    
    /// List directory contents (only for directories)
    fn readdir(&self) -> super::VfsResult<Vec<String>> {
        Err(super::VfsError::NotADirectory)
    }
    
    /// Create a subdirectory (only for directories)
    fn mkdir(&self, name: &str) -> super::VfsResult<()> {
        let _ = name;
        Err(super::VfsError::NotADirectory)
    }
    
    /// Remove a child entry (only for directories)
    fn unlink(&self, name: &str) -> super::VfsResult<()> {
        let _ = name;
        Err(super::VfsError::NotADirectory)
    }
    
    /// Open a child entry (only for directories)
    fn open_child(&self, name: &str) -> super::VfsResult<Arc<dyn File>> {
        let _ = name;
        Err(super::VfsError::NotADirectory)
    }
    
    /// Get the file name
    fn name(&self) -> String;
    
    /// Check if this is a directory
    fn is_dir(&self) -> bool {
        matches!(self.metadata().map(|m| m.file_type), Ok(FileType::Directory))
    }
}
