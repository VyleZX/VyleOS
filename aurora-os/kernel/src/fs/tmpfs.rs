//! tmpfs - Temporary filesystem stored in memory

use alloc::sync::Arc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use spin::RwLock;
use crate::vfs::{File, FileMetadata, FileType, Permissions, VfsResult, VfsError, FsStats};

/// tmpfs inode types
enum InodeType {
    File(Vec<u8>),
    Directory(BTreeMap<String, Arc<Inode>>),
}

/// tmpfs inode
struct Inode {
    id: u64,
    node_type: RwLock<InodeType>,
    metadata: RwLock<FileMetadata>,
}

impl Inode {
    fn new_file(id: u64) -> Self {
        Self {
            id,
            node_type: RwLock::new(InodeType::File(Vec::new())),
            metadata: RwLock::new(FileMetadata {
                file_type: FileType::RegularFile,
                size: 0,
                permissions: Permissions::new(),
                inode: id,
                created_at: 0,
                modified_at: 0,
                accessed_at: 0,
                uid: 0,
                gid: 0,
                link_count: 1,
            }),
        }
    }
    
    fn new_directory(id: u64) -> Self {
        Self {
            id,
            node_type: RwLock::new(InodeType::Directory(BTreeMap::new())),
            metadata: RwLock::new(FileMetadata {
                file_type: FileType::Directory,
                size: 0,
                permissions: Permissions::new(),
                inode: id,
                created_at: 0,
                modified_at: 0,
                accessed_at: 0,
                uid: 0,
                gid: 0,
                link_count: 2, // . and parent
            }),
        }
    }
}

/// A file in tmpfs
struct TmpFile {
    inode: Arc<Inode>,
    name: String,
}

impl TmpFile {
    fn new(inode: Arc<Inode>, name: String) -> Self {
        Self { inode, name }
    }
}

impl File for TmpFile {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> VfsResult<usize> {
        let metadata = self.inode.metadata.read();
        
        if offset >= metadata.size {
            return Ok(0);
        }
        
        match &*self.inode.node_type.read() {
            InodeType::File(data) => {
                let start = offset as usize;
                let end = (offset as usize + buf.len()).min(data.len());
                if start < data.len() {
                    let bytes_to_read = end - start;
                    buf[..bytes_to_read].copy_from_slice(&data[start..end]);
                    Ok(bytes_to_read)
                } else {
                    Ok(0)
                }
            }
            _ => Err(VfsError::NotAFile),
        }
    }
    
    fn write_at(&self, buf: &[u8], offset: u64) -> VfsResult<usize> {
        match &mut *self.inode.node_type.write() {
            InodeType::File(data) => {
                let offset = offset as usize;
                
                // Extend the file if necessary
                if offset + buf.len() > data.len() {
                    data.resize(offset + buf.len(), 0);
                }
                
                data[offset..offset + buf.len()].copy_from_slice(buf);
                
                // Update metadata
                let mut meta = self.inode.metadata.write();
                meta.size = data.len() as u64;
                
                Ok(buf.len())
            }
            _ => Err(VfsError::NotAFile),
        }
    }
    
    fn metadata(&self) -> VfsResult<FileMetadata> {
        Ok((*self.inode.metadata.read()).clone())
    }
    
    fn truncate(&self, size: u64) -> VfsResult<()> {
        match &mut *self.inode.node_type.write() {
            InodeType::File(data) => {
                data.truncate(size as usize);
                self.inode.metadata.write().size = size;
                Ok(())
            }
            _ => Err(VfsError::NotAFile),
        }
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// A directory in tmpfs
struct TmpDir {
    inode: Arc<Inode>,
    name: String,
}

impl TmpDir {
    fn new(inode: Arc<Inode>, name: String) -> Self {
        Self { inode, name }
    }
}

impl File for TmpDir {
    fn read_at(&self, _buf: &mut [u8], _offset: u64) -> VfsResult<usize> {
        Err(VfsError::IsADirectory)
    }
    
    fn write_at(&self, _buf: &[u8], _offset: u64) -> VfsResult<usize> {
        Err(VfsError::IsADirectory)
    }
    
    fn metadata(&self) -> VfsResult<FileMetadata> {
        Ok((*self.inode.metadata.read()).clone())
    }
    
    fn readdir(&self) -> VfsResult<Vec<String>> {
        match &*self.inode.node_type.read() {
            InodeType::Directory(entries) => {
                let mut names = Vec::new();
                names.push(".".to_string());
                names.push("..".to_string());
                for name in entries.keys() {
                    names.push(name.clone());
                }
                Ok(names)
            }
            _ => Err(VfsError::NotADirectory),
        }
    }
    
    fn mkdir(&self, name: &str) -> VfsResult<()> {
        match &mut *self.inode.node_type.write() {
            InodeType::Directory(entries) => {
                if entries.contains_key(name) {
                    return Err(VfsError::AlreadyExists);
                }
                
                static NEXT_INODE: core::sync::atomic::AtomicU64 = 
                    core::sync::atomic::AtomicU64::new(2);
                let id = NEXT_INODE.fetch_add(1, core::sync::atomic::Ordering::SeqCst);
                
                let new_dir = Arc::new(Inode::new_directory(id));
                entries.insert(name.to_string(), new_dir);
                
                Ok(())
            }
            _ => Err(VfsError::NotADirectory),
        }
    }
    
    fn unlink(&self, name: &str) -> VfsResult<()> {
        match &mut *self.inode.node_type.write() {
            InodeType::Directory(entries) => {
                entries.remove(name).ok_or(VfsError::NotFound)?;
                Ok(())
            }
            _ => Err(VfsError::NotADirectory),
        }
    }
    
    fn open_child(&self, name: &str) -> VfsResult<Arc<dyn File>> {
        match &*self.inode.node_type.read() {
            InodeType::Directory(entries) => {
                if let Some(inode) = entries.get(name) {
                    match &*inode.node_type.read() {
                        InodeType::File(_) => {
                            Ok(Arc::new(TmpFile::new(inode.clone(), name.to_string())))
                        }
                        InodeType::Directory(_) => {
                            Ok(Arc::new(TmpDir::new(inode.clone(), name.to_string())))
                        }
                    }
                } else {
                    Err(VfsError::NotFound)
                }
            }
            _ => Err(VfsError::NotADirectory),
        }
    }
    
    fn name(&self) -> String {
        self.name.clone()
    }
}

/// tmpfs filesystem implementation
pub struct TmpFs {
    root: Arc<Inode>,
}

impl TmpFs {
    pub fn new() -> Self {
        Self {
            root: Arc::new(Inode::new_directory(1)),
        }
    }
}

impl Default for TmpFs {
    fn default() -> Self {
        Self::new()
    }
}

impl super::Filesystem for TmpFs {
    fn open(&self, path: &str) -> VfsResult<Arc<dyn File>> {
        let path = path.trim_matches('/');
        
        if path.is_empty() {
            return Ok(Arc::new(TmpDir::new(self.root.clone(), "/".to_string())));
        }
        
        // Navigate to the parent directory
        let components: Vec<&str> = path.split('/').collect();
        let filename = components.last().ok_or(VfsError::InvalidPath)?;
        let parent_path = if components.len() > 1 {
            components[..components.len() - 1].join("/")
        } else {
            String::new()
        };
        
        // Find parent directory
        let mut current = self.root.clone();
        for component in &parent_path.split('/').collect::<Vec<_>>() {
            if component.is_empty() {
                continue;
            }
            match &*current.node_type.read() {
                InodeType::Directory(entries) => {
                    if let Some(child) = entries.get(*component) {
                        current = child.clone();
                    } else {
                        return Err(VfsError::NotFound);
                    }
                }
                _ => return Err(VfsError::NotADirectory),
            }
        }
        
        // Open the target
        current.open_child(filename)
    }
    
    fn statfs(&self) -> VfsResult<FsStats> {
        Ok(FsStats {
            block_size: 4096,
            total_blocks: 1024 * 1024, // 4GB virtual
            free_blocks: 1024 * 1024,
            total_inodes: 1000000,
            free_inodes: 1000000,
            fs_type: "tmpfs",
        })
    }
    
    fn sync(&self) -> VfsResult<()> {
        Ok(()) // Already in memory
    }
    
    fn fs_type(&self) -> &'static str {
        "tmpfs"
    }
}
