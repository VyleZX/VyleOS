//! Process Management
//!
//! A process is the basic unit of resource allocation in the OS.

use alloc::sync::Arc;
use alloc::vec::Vec;
use spin::RwLock;
use super::thread::Thread;
use crate::mm::page_table::PageTable;

/// Process ID type
pub type Pid = usize;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    /// Process is runnable or running
    Running,
    /// Process is waiting for I/O or other event
    Waiting,
    /// Process is stopped (e.g., by debugger)
    Stopped,
    /// Process has terminated but not yet reaped
    Zombie,
}

/// Process information
pub struct Process {
    /// Unique process identifier
    pub pid: Pid,
    /// Parent process ID
    pub ppid: Pid,
    /// Process state
    pub state: RwLock<ProcessState>,
    /// Threads belonging to this process
    pub threads: RwLock<Vec<Arc<Thread>>>,
    /// Process address space
    pub page_table: RwLock<Option<PageTable>>,
    /// Current working directory
    pub cwd: RwLock<alloc::string::String>,
    /// Environment variables
    pub env: RwLock<alloc::collections::BTreeMap<alloc::string::String, alloc::string::String>>,
    /// Open file descriptors
    pub fds: RwLock<Vec<Option<Arc<dyn crate::vfs::File>>>>,
    /// Signal handlers (placeholder)
    pub signals: RwLock<[SignalHandler; 64]>,
    /// User ID
    pub uid: u32,
    /// Group ID
    pub gid: u32,
    /// Process start time
    pub start_time: u64,
    /// CPU time used
    pub utime: u64,
    pub stime: u64,
}

/// Signal handler placeholder
#[derive(Clone, Copy)]
pub struct SignalHandler {
    pub handler: usize, // Function pointer or special value
    pub flags: u32,
}

impl Default for SignalHandler {
    fn default() -> Self {
        Self {
            handler: 0, // SIG_DFL
            flags: 0,
        }
    }
}

impl Process {
    /// Create a new process
    pub fn new(pid: Pid, ppid: Pid) -> Self {
        Self {
            pid,
            ppid,
            state: RwLock::new(ProcessState::Running),
            threads: RwLock::new(Vec::new()),
            page_table: RwLock::new(None),
            cwd: RwLock::new("/".into()),
            env: RwLock::new(alloc::collections::BTreeMap::new()),
            fds: RwLock::new(Vec::new()),
            signals: RwLock::new([SignalHandler::default(); 64]),
            uid: 0,
            gid: 0,
            start_time: 0,
            utime: 0,
            stime: 0,
        }
    }
    
    /// Create the init process (PID 1)
    pub fn new_init() -> Self {
        let mut proc = Self::new(1, 0);
        proc.uid = 0;
        proc.gid = 0;
        proc.cwd = RwLock::new("/".into());
        proc
    }
    
    /// Add a thread to the process
    pub fn add_thread(&self, thread: Arc<Thread>) {
        self.threads.write().push(thread);
    }
    
    /// Get the main thread
    pub fn main_thread(&self) -> Option<Arc<Thread>> {
        self.threads.read().first().cloned()
    }
    
    /// Set process state
    pub fn set_state(&self, state: ProcessState) {
        *self.state.write() = state;
    }
    
    /// Get process state
    pub fn get_state(&self) -> ProcessState {
        *self.state.read()
    }
    
    /// Check if process is alive
    pub fn is_alive(&self) -> bool {
        matches!(*self.state.read(), ProcessState::Running | ProcessState::Waiting)
    }
}

impl core::fmt::Debug for Process {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Process")
            .field("pid", &self.pid)
            .field("ppid", &self.ppid)
            .field("state", &*self.state.read())
            .field("threads", &self.threads.read().len())
            .finish()
    }
}
