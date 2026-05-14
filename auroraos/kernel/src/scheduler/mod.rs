//! Process scheduler module

use spin::Mutex;
use alloc::collections::VecDeque;
use alloc::sync::Arc;
use crate::arch::x86_64::registers::RegisterState;

/// Scheduler instance
static SCHEDULER: Mutex<Option<Scheduler>> = Mutex::new(None);

/// Initialize the scheduler
pub fn init() {
    *SCHEDULER.lock() = Some(Scheduler::new());
}

/// Get scheduler tick (called from timer interrupt)
pub fn tick() {
    if let Some(ref mut sched) = *SCHEDULER.lock() {
        sched.tick();
    }
}

/// Run the scheduler (starts the first process)
pub fn run() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Process ID type
pub type ProcessId = u32;

/// Thread ID type
pub type ThreadId = u32;

/// Process state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcessState {
    Running,
    Ready,
    Blocked,
    Terminated,
}

/// Process structure
pub struct Process {
    pub pid: ProcessId,
    pub name: [u8; 16],
    pub state: ProcessState,
    pub threads: Vec<Thread>,
    pub page_table_root: u64,
    pub parent_pid: Option<ProcessId>,
}

impl Process {
    /// Create a new process
    pub fn new(pid: ProcessId, name: &str) -> Self {
        let mut name_bytes = [0u8; 16];
        let name_slice = name.as_bytes();
        let copy_len = name_slice.len().min(16);
        name_bytes[..copy_len].copy_from_slice(&name_slice[..copy_len]);
        
        Self {
            pid,
            name: name_bytes,
            state: ProcessState::Ready,
            threads: Vec::new(),
            page_table_root: 0,
            parent_pid: None,
        }
    }
}

/// Thread structure
pub struct Thread {
    pub tid: ThreadId,
    pub state: ProcessState,
    pub registers: RegisterState,
    pub stack_pointer: u64,
    pub priority: i32,
}

impl Thread {
    /// Create a new thread
    pub fn new(tid: ThreadId) -> Self {
        Self {
            tid,
            state: ProcessState::Ready,
            registers: RegisterState::default(),
            stack_pointer: 0,
            priority: 0,
        }
    }
}

/// Completely Fair Scheduler implementation
pub struct Scheduler {
    ready_queue: VecDeque<Arc<Process>>,
    current_pid: Option<ProcessId>,
    next_pid: ProcessId,
    tick_count: u64,
}

impl Scheduler {
    /// Create a new scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            current_pid: None,
            next_pid: 1,
            tick_count: 0,
        }
    }
    
    /// Handle timer tick
    pub fn tick(&mut self) {
        self.tick_count += 1;
        
        // Simple round-robin for now, CFS would be implemented here
        if self.tick_count % 10 == 0 {
            self.schedule();
        }
    }
    
    /// Perform scheduling decision
    fn schedule(&mut self) {
        // Simple round-robin scheduling
        if let Some(process) = self.ready_queue.pop_front() {
            self.ready_queue.push_back(process);
        }
    }
    
    /// Add a process to the ready queue
    pub fn add_process(&mut self, process: Arc<Process>) {
        self.ready_queue.push_back(process);
    }
    
    /// Get the next process to run
    pub fn get_next_process(&mut self) -> Option<Arc<Process>> {
        self.ready_queue.front().cloned()
    }
}

/// Create a new process
pub fn create_process(name: &str) -> ProcessId {
    if let Some(ref mut sched) = *SCHEDULER.lock() {
        let pid = sched.next_pid;
        sched.next_pid += 1;
        
        let process = Arc::new(Process::new(pid, name));
        sched.add_process(process);
        
        pid
    } else {
        panic!("Scheduler not initialized");
    }
}

/// Block the current process
pub fn block_current() {
    // Implementation would save context and switch to another process
}

/// Wake up a blocked process
pub fn wake_up(pid: ProcessId) {
    // Implementation would move process from blocked to ready queue
}

/// Exit the current process
pub fn exit(code: i32) {
    // Implementation would clean up process resources
    let _ = code;
}
