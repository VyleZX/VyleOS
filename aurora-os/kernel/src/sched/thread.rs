//! Thread Management
//!
//! A thread is the basic unit of CPU execution.

use alloc::sync::Arc;
use spin::RwLock;
use x86_64::{VirtAddr, structures::paging::Page};

/// Thread ID type
pub type Tid = usize;

/// Thread state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThreadState {
    /// Thread is runnable or running
    Runnable,
    /// Thread is blocked waiting for something
    Blocked,
    /// Thread is sleeping
    Sleeping,
    /// Thread has terminated
    Terminated,
}

/// Thread priority (0 = lowest, 99 = highest real-time)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(u8);

impl Priority {
    pub const MIN: Self = Priority(0);
    pub const NORMAL: Self = Priority(50);
    pub const MAX: Self = Priority(99);
    
    pub fn new(value: u8) -> Self {
        Priority(value.min(99))
    }
    
    pub fn value(&self) -> u8 {
        self.0
    }
}

/// CPU context saved during context switch
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct CpuContext {
    // General purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // Instruction pointer and stack
    pub rip: u64,
    pub rsp: u64,
    // Flags register
    pub rflags: u64,
}

impl CpuContext {
    pub const fn new() -> Self {
        Self {
            rax: 0, rbx: 0, rcx: 0, rdx: 0,
            rsi: 0, rdi: 0, rbp: 0,
            r8: 0, r9: 0, r10: 0, r11: 0,
            r12: 0, r13: 0, r14: 0, r15: 0,
            rip: 0, rsp: 0, rflags: 0,
        }
    }
}

/// Thread information
pub struct Thread {
    /// Unique thread identifier
    pub tid: Tid,
    /// Parent process ID
    pub pid: usize,
    /// Thread state
    pub state: RwLock<ThreadState>,
    /// Thread priority
    pub priority: RwLock<Priority>,
    /// Saved CPU context
    pub context: RwLock<CpuContext>,
    /// Kernel stack pointer
    pub kernel_stack: VirtAddr,
    /// User stack pointer (if applicable)
    pub user_stack: Option<VirtAddr>,
    /// Thread name
    pub name: RwLock<alloc::string::String>,
    /// Time spent running (in ticks)
    pub runtime: u64,
    /// Sleep wake-up time (in ticks since boot)
    pub wake_time: u64,
    /// CPU affinity mask
    pub cpu_affinity: u64,
}

impl Thread {
    /// Create a new kernel thread
    pub fn new_kernel(tid: Tid, pid: usize, entry: extern "C" fn() -> !) -> Arc<Self> {
        // Allocate kernel stack (typically 16KB)
        let stack_size = 16 * 1024;
        let stack_ptr = crate::mm::heap::allocate_stack(stack_size);
        
        let mut thread = Self {
            tid,
            pid,
            state: RwLock::new(ThreadState::Runnable),
            priority: RwLock::new(Priority::NORMAL),
            context: RwLock::new(CpuContext::new()),
            kernel_stack: stack_ptr,
            user_stack: None,
            name: RwLock::new(format!("thread-{}", tid)),
            runtime: 0,
            wake_time: 0,
            cpu_affinity: u64::MAX, // Can run on any CPU
        };
        
        // Set up initial context
        thread.context.write().rsp = stack_ptr.as_u64();
        thread.context.write().rip = entry as u64;
        thread.context.write().rflags = 0x202; // Interrupts enabled
        
        Arc::new(thread)
    }
    
    /// Create a new user thread
    pub fn new_user(tid: Tid, pid: usize, entry: VirtAddr, user_stack: VirtAddr) -> Arc<Self> {
        let stack_size = 16 * 1024;
        let kernel_stack = crate::mm::heap::allocate_stack(stack_size);
        
        let mut thread = Self {
            tid,
            pid,
            state: RwLock::new(ThreadState::Runnable),
            priority: RwLock::new(Priority::NORMAL),
            context: RwLock::new(CpuContext::new()),
            kernel_stack,
            user_stack: Some(user_stack),
            name: RwLock::new(format!("user-thread-{}", tid)),
            runtime: 0,
            wake_time: 0,
            cpu_affinity: u64::MAX,
        };
        
        // Set up context for user mode entry
        let ctx = &mut *thread.context.write();
        ctx.rsp = user_stack.as_u64();
        ctx.rip = entry.as_u64();
        ctx.rflags = 0x202;
        
        Arc::new(thread)
    }
    
    /// Set thread state
    pub fn set_state(&self, state: ThreadState) {
        *self.state.write() = state;
    }
    
    /// Get thread state
    pub fn get_state(&self) -> ThreadState {
        *self.state.read()
    }
    
    /// Set thread priority
    pub fn set_priority(&self, priority: Priority) {
        *self.priority.write() = priority;
    }
    
    /// Get thread priority
    pub fn get_priority(&self) -> Priority {
        *self.priority.read()
    }
    
    /// Check if thread is runnable
    pub fn is_runnable(&self) -> bool {
        matches!(*self.state.read(), ThreadState::Runnable)
    }
}

impl core::fmt::Debug for Thread {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Thread")
            .field("tid", &self.tid)
            .field("pid", &self.pid)
            .field("state", &*self.state.read())
            .field("priority", &*self.priority.read())
            .field("name", &*self.name.read())
            .finish()
    }
}
