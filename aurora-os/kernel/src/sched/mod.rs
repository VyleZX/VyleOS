//! Process Scheduler
//!
//! Implements a preemptive multitasking scheduler with support for:
//! - Multiple scheduling policies (CFS-inspired, real-time)
//! - Per-CPU runqueues for SMP
//! - Thread priorities and nice values
//! - CPU affinity
//! - Sleep/wake functionality

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::{Mutex, RwLock};
use core::sync::atomic::{AtomicUsize, Ordering};

pub mod process;
pub mod thread;
pub mod scheduler;

pub use self::process::Process;
pub use self::thread::Thread;
pub use self::scheduler::Scheduler;

/// Maximum number of CPUs supported
const MAX_CPUS: usize = 256;

/// Global scheduler instance
static SCHEDULER: RwLock<Option<Scheduler>> = RwLock::new(None);

/// Process ID counter
static PID_COUNTER: AtomicUsize = AtomicUsize::new(1);

/// Initialize the scheduler subsystem
pub fn init() {
    let scheduler = Scheduler::new();
    *SCHEDULER.write() = Some(scheduler);
    log::info!("Scheduler initialized");
}

/// Get a reference to the global scheduler
pub fn get_scheduler() -> Option<impl core::ops::Deref<Target = Scheduler>> {
    SCHEDULER.read().map(|s| s)
}

/// Allocate a new process ID
pub fn allocate_pid() -> usize {
    PID_COUNTER.fetch_add(1, Ordering::SeqCst)
}

/// Spawn the initial userspace process (init)
pub fn spawn_init() {
    let scheduler = SCHEDULER.read();
    if let Some(ref sched) = *scheduler {
        // Create init process with PID 1
        let init = Process::new_init();
        sched.add_process(init);
        log::info!("Init process spawned");
    }
}

/// Yield the current CPU time slice
pub fn yield_now() {
    let scheduler = SCHEDULER.read();
    if let Some(ref sched) = *scheduler {
        sched.yield_current();
    }
}

/// Sleep for a specified duration
pub fn sleep_ms(ms: u64) {
    let scheduler = SCHEDULER.read();
    if let Some(ref sched) = *scheduler {
        sched.sleep_current(ms);
    }
}

/// Wake up sleeping threads
pub fn wake_sleeping_threads() {
    let scheduler = SCHEDULER.read();
    if let Some(ref sched) = *scheduler {
        sched.wake_sleeping();
    }
}

/// Get current thread info (would be thread-local in full implementation)
pub fn current_thread_id() -> Option<usize> {
    None // Would return thread-local storage value
}

/// Get current process info
pub fn current_process_id() -> Option<usize> {
    None // Would return thread-local storage value
}
