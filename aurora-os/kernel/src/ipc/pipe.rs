//! Pipe IPC

use alloc::vec::Vec;
use spin::Mutex;
use super::{IpcResult, IpcError};

/// Maximum pipe buffer size
const PIPE_BUFFER_SIZE: usize = 65536; // 64KB

/// A unidirectional pipe for IPC
pub struct Pipe {
    /// Buffer for pipe data
    buffer: Mutex<Vec<u8>>,
    /// Number of readers
    readers: Mutex<u32>,
    /// Number of writers
    writers: Mutex<u32>,
}

impl Pipe {
    pub fn new() -> Self {
        Self {
            buffer: Mutex::new(Vec::with_capacity(PIPE_BUFFER_SIZE)),
            readers: Mutex::new(0),
            writers: Mutex::new(0),
        }
    }
    
    /// Add a reader to the pipe
    pub fn add_reader(&self) {
        *self.readers.lock() += 1;
    }
    
    /// Remove a reader from the pipe
    pub fn remove_reader(&self) {
        let mut readers = self.readers.lock();
        *readers -= 1;
    }
    
    /// Add a writer to the pipe
    pub fn add_writer(&self) {
        *self.writers.lock() += 1;
    }
    
    /// Remove a writer from the pipe
    pub fn remove_writer(&self) {
        let mut writers = self.writers.lock();
        *writers -= 1;
    }
    
    /// Read from the pipe
    pub fn read(&self, buf: &mut [u8]) -> IpcResult<usize> {
        let mut buffer = self.buffer.lock();
        
        if buffer.is_empty() {
            if *self.writers.lock() == 0 {
                return Ok(0); // EOF - no writers and empty
            }
            return Err(IpcError::QueueEmpty); // Would block in blocking impl
        }
        
        let bytes_to_read = buf.len().min(buffer.len());
        buf[..bytes_to_read].copy_from_slice(&buffer[..bytes_to_read]);
        buffer.drain(..bytes_to_read);
        
        Ok(bytes_to_read)
    }
    
    /// Write to the pipe
    pub fn write(&self, buf: &[u8]) -> IpcResult<usize> {
        let mut buffer = self.buffer.lock();
        
        if buffer.len() + buf.len() > PIPE_BUFFER_SIZE {
            return Err(IpcError::QueueFull);
        }
        
        buffer.extend_from_slice(buf);
        Ok(buf.len())
    }
    
    /// Check if there's data to read
    pub fn can_read(&self) -> bool {
        !self.buffer.lock().is_empty()
    }
    
    /// Check if we can write more data
    pub fn can_write(&self) -> bool {
        self.buffer.lock().len() < PIPE_BUFFER_SIZE
    }
}

impl Default for Pipe {
    fn default() -> Self {
        Self::new()
    }
}

/// Create a connected pair of pipe file descriptors
pub fn pipe() -> (PipeReadEnd, PipeWriteEnd) {
    let pipe = Arc::new(Pipe::new());
    pipe.add_reader();
    pipe.add_writer();
    
    (
        PipeReadEnd { pipe: pipe.clone() },
        PipeWriteEnd { pipe },
    )
}

use alloc::sync::Arc;

/// Read end of a pipe
pub struct PipeReadEnd {
    pipe: Arc<Pipe>,
}

impl PipeReadEnd {
    pub fn read(&self, buf: &mut [u8]) -> IpcResult<usize> {
        self.pipe.read(buf)
    }
}

impl Drop for PipeReadEnd {
    fn drop(&mut self) {
        self.pipe.remove_reader();
    }
}

/// Write end of a pipe
pub struct PipeWriteEnd {
    pipe: Arc<Pipe>,
}

impl PipeWriteEnd {
    pub fn write(&self, buf: &[u8]) -> IpcResult<usize> {
        self.pipe.write(buf)
    }
}

impl Drop for PipeWriteEnd {
    fn drop(&mut self) {
        self.pipe.remove_writer();
    }
}
