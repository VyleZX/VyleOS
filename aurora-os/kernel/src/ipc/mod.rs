//! Inter-Process Communication (IPC) Subsystem
//!
//! Provides high-performance IPC mechanisms:
//! - Message passing
//! - Shared memory
//! - Pipes and FIFOs
//! - Signals

use alloc::sync::Arc;
use spin::{Mutex, RwLock};

pub mod message;
pub mod shared_memory;
pub mod pipe;

pub use self::message::{Message, MessageQueue};
pub use self::shared_memory::SharedMemory;
pub use self::pipe::Pipe;

/// Initialize the IPC subsystem
pub fn init() {
    log::info!("IPC subsystem initialized");
}

/// IPC error types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IpcError {
    QueueFull,
    QueueEmpty,
    InvalidPort,
    PermissionDenied,
    ConnectionRefused,
    ConnectionReset,
    Timeout,
}

pub type IpcResult<T> = Result<T, IpcError>;
