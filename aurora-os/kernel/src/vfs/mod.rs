//! Virtual Filesystem (VFS) Layer
//!
//! Provides a unified interface for all filesystem operations.

use alloc::sync::Arc;
use alloc::string::String;
use spin::RwLock;

pub mod file;
pub mod path;
pub mod mount;

pub use self::file::{File, FileType, FileMetadata};
pub use self::path::Path;
pub use self::mount::MountPoint;

/// VFS error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VfsError {
    NotFound,
    PermissionDenied,
    AlreadyExists,
    InvalidPath,
    NotADirectory,
    NotAFile,
    IsADirectory,
    DirectoryNotEmpty,
    NoSpaceLeft,
    TooManyOpenFiles,
    InvalidSeek,
    IoError,
}

pub type VfsResult<T> = Result<T, VfsError>;

/// Inode number type
pub type InodeId = u64;

/// File descriptor type  
pub type Fd = usize;

/// Open file description
pub struct OpenFile {
    pub file: Arc<dyn File>,
    pub position: u64,
    pub flags: OpenFlags,
}

impl OpenFile {
    pub fn new(file: Arc<dyn File>, flags: OpenFlags) -> Self {
        Self {
            file,
            position: 0,
            flags,
        }
    }
}

/// File open flags
bitflags::bitflags! {
    pub struct OpenFlags: u32 {
        const READ = 0b0000_0001;
        const WRITE = 0b0000_0010;
        const APPEND = 0b0000_0100;
        const CREATE = 0b0001_0000;
        const TRUNCATE = 0b0010_0000;
        const EXCLUSIVE = 0b0100_0000;
    }
}

/// Main VFS structure
pub struct VirtualFilesystem {
    /// Root mount point
    root: RwLock<Option<Arc<MountPoint>>>,
    /// All mount points
    mounts: RwLock<alloc::vec::Vec<Arc<MountPoint>>>,
    /// Global file descriptor table (per-process in full impl)
    fd_table: RwLock<alloc::collections::BTreeMap<Fd, OpenFile>>,
    /// Next file descriptor counter
    next_fd: RwLock<Fd>,
}

impl VirtualFilesystem {
    pub const fn new() -> Self {
        Self {
            root: RwLock::new(None),
            mounts: RwLock::new(alloc::vec::Vec::new()),
            fd_table: RwLock::new(alloc::collections::BTreeMap::new()),
            next_fd: RwLock::new(0),
        }
    }
    
    /// Mount a filesystem at the specified path
    pub fn mount(&self, path: &str, filesystem: Arc<dyn crate::fs::Filesystem>) -> VfsResult<()> {
        let mount = MountPoint::new(path.into(), filesystem);
        let arc_mount = Arc::new(mount);
        
        if path == "/" {
            *self.root.write() = Some(arc_mount.clone());
        }
        
        self.mounts.write().push(arc_mount);
        Ok(())
    }
    
    /// Open a file at the specified path
    pub fn open(&self, path: &str, flags: OpenFlags) -> VfsResult<Fd> {
        let file = self.resolve_path(path)?;
        
        // Check permissions
        if flags.contains(OpenFlags::WRITE) && !file.metadata()?.permissions.writable() {
            return Err(VfsError::PermissionDenied);
        }
        
        let open_file = OpenFile::new(file, flags);
        
        let mut fd_table = self.fd_table.write();
        let fd = *self.next_fd.read();
        fd_table.insert(fd, open_file);
        *self.next_fd.write() += 1;
        
        Ok(fd)
    }
    
    /// Read from an open file descriptor
    pub fn read(&self, fd: Fd, buf: &mut [u8]) -> VfsResult<usize> {
        let mut fd_table = self.fd_table.write();
        let open_file = fd_table.get_mut(&fd).ok_or(VfsError::InvalidSeek)?;
        
        let bytes_read = open_file.file.read_at(buf, open_file.position)?;
        open_file.position += bytes_read as u64;
        
        Ok(bytes_read)
    }
    
    /// Write to an open file descriptor
    pub fn write(&self, fd: Fd, buf: &[u8]) -> VfsResult<usize> {
        let mut fd_table = self.fd_table.write();
        let open_file = fd_table.get_mut(&fd).ok_or(VfsError::InvalidSeek)?;
        
        let bytes_written = open_file.file.write_at(buf, open_file.position)?;
        open_file.position += bytes_written as u64;
        
        Ok(bytes_written)
    }
    
    /// Close a file descriptor
    pub fn close(&self, fd: Fd) -> VfsResult<()> {
        let mut fd_table = self.fd_table.write();
        fd_table.remove(&fd).ok_or(VfsError::InvalidSeek)?;
        Ok(())
    }
    
    /// Get metadata for a file
    pub fn stat(&self, path: &str) -> VfsResult<FileMetadata> {
        let file = self.resolve_path(path)?;
        file.metadata()
    }
    
    /// List directory contents
    pub fn readdir(&self, path: &str) -> VfsResult<alloc::vec::Vec<String>> {
        let file = self.resolve_path(path)?;
        
        if file.metadata()?.file_type != FileType::Directory {
            return Err(VfsError::NotADirectory);
        }
        
        file.readdir()
    }
    
    /// Create a directory
    pub fn mkdir(&self, path: &str) -> VfsResult<()> {
        // Find parent directory and create child
        let parent_path = Path::parent(path);
        let name = Path::basename(path);
        
        let parent = self.resolve_path(parent_path)?;
        parent.mkdir(name)
    }
    
    /// Remove a file or directory
    pub fn unlink(&self, path: &str) -> VfsResult<()> {
        let parent_path = Path::parent(path);
        let name = Path::basename(path);
        
        let parent = self.resolve_path(parent_path)?;
        parent.unlink(name)
    }
    
    /// Resolve a path to a file
    fn resolve_path(&self, path: &str) -> VfsResult<Arc<dyn File>> {
        let mounts = self.mounts.read();
        
        // Find the appropriate mount point
        let mut best_mount: Option<&Arc<MountPoint>> = None;
        let mut best_len = 0;
        
        for mount in mounts.iter() {
            if path.starts_with(&mount.path) && mount.path.len() > best_len {
                best_mount = Some(mount);
                best_len = mount.path.len();
            }
        }
        
        let mount = best_mount.ok_or(VfsError::NotFound)?;
        let relative_path = if best_len > 1 {
            &path[best_len..]
        } else {
            path
        };
        
        mount.filesystem.open(relative_path)
    }
}

/// Global VFS instance
static VFS: RwLock<Option<VirtualFilesystem>> = RwLock::new(None);

/// Initialize the VFS subsystem
pub fn init() {
    *VFS.write() = Some(VirtualFilesystem::new());
    log::info!("Virtual filesystem initialized");
}

/// Get the global VFS instance
pub fn get_vfs() -> Option<impl core::ops::Deref<Target = VirtualFilesystem>> {
    VFS.read().map(|v| v)
}
