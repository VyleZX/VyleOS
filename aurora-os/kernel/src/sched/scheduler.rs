//! Multi-queue Scheduler Implementation
//!
//! Implements a CFS-inspired scheduler with:
//! - Per-CPU runqueues for SMP scalability
//! - Multiple priority queues
//! - Fair scheduling with virtual runtime tracking
//! - Real-time priority support

use alloc::collections::VecDeque;
use alloc::sync::Arc;
use spin::{Mutex, RwLock};
use super::{Thread, ThreadState, Priority, Process};
use crate::arch;

/// Number of priority queues
const NUM_QUEUES: usize = 10;

/// Time slice in milliseconds
const TIME_SLICE_MS: u64 = 10;

/// Per-CPU runqueue
pub struct RunQueue {
    /// Priority queues (higher index = higher priority)
    queues: [VecDeque<Arc<Thread>>; NUM_QUEUES],
    /// Currently running thread
    current: Option<Arc<Thread>>,
    /// Number of threads in this runqueue
    thread_count: usize,
}

impl RunQueue {
    pub const fn new() -> Self {
        Self {
            queues: unsafe { core::mem::zeroed() },
            current: None,
            thread_count: 0,
        }
    }
    
    /// Add a thread to the runqueue
    pub fn enqueue(&mut self, thread: Arc<Thread>) {
        let priority = thread.get_priority().value() as usize;
        let queue_idx = priority * NUM_QUEUES / 100;
        self.queues[queue_idx].push_back(thread);
        self.thread_count += 1;
    }
    
    /// Get the next runnable thread
    pub fn dequeue(&mut self) -> Option<Arc<Thread>> {
        // Check queues from highest to lowest priority
        for queue in self.queues.iter_mut().rev() {
            if let Some(thread) = queue.pop_front() {
                self.thread_count -= 1;
                return Some(thread);
            }
        }
        None
    }
    
    /// Peek at the next thread without removing
    pub fn peek(&self) -> Option<&Arc<Thread>> {
        for queue in self.queues.iter().rev() {
            if let Some(thread) = queue.front() {
                return Some(thread);
            }
        }
        None
    }
    
    /// Set the currently running thread
    pub fn set_current(&mut self, thread: Option<Arc<Thread>>) {
        self.current = thread;
    }
    
    /// Get the currently running thread
    pub fn current(&self) -> Option<&Arc<Thread>> {
        self.current.as_ref()
    }
    
    /// Get thread count
    pub fn len(&self) -> usize {
        self.thread_count
    }
    
    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.thread_count == 0
    }
}

/// Main scheduler structure
pub struct Scheduler {
    /// Per-CPU runqueues
    runqueues: [Mutex<RunQueue>; super::MAX_CPUS],
    /// List of all processes
    processes: RwLock<alloc::collections::BTreeMap<usize, Arc<super::Process>>>,
    /// Sleep queue (threads waiting to be woken)
    sleep_queue: Mutex<VecDeque<(u64, Arc<Thread>)>>, // (wake_time, thread)
    /// Tick counter
    tick: u64,
}

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            runqueues: unsafe { core::mem::zeroed() },
            processes: RwLock::new(alloc::collections::BTreeMap::new()),
            sleep_queue: Mutex::new(VecDeque::new()),
            tick: 0,
        }
    }
    
    /// Add a process to the scheduler
    pub fn add_process(&self, process: Process) {
        let pid = process.pid;
        let arc_process = Arc::new(process);
        
        // Add main thread to runqueue
        if let Some(main_thread) = arc_process.main_thread() {
            let cpu_id = arch::cpu_id();
            self.runqueues[cpu_id].lock().enqueue(main_thread);
        }
        
        self.processes.write().insert(pid, arc_process);
    }
    
    /// Add a thread to the scheduler
    pub fn add_thread(&self, thread: Arc<Thread>) {
        let cpu_id = arch::cpu_id();
        self.runqueues[cpu_id].lock().enqueue(thread);
    }
    
    /// Yield the current CPU - reschedule
    pub fn yield_current(&self) {
        let cpu_id = arch::cpu_id();
        let mut rq = self.runqueues[cpu_id].lock();
        
        // Put current thread back in runqueue if still runnable
        if let Some(current) = rq.current.take() {
            if current.is_runnable() {
                rq.enqueue(current);
            }
        }
        
        // Get next thread
        if let Some(next) = rq.dequeue() {
            rq.set_current(Some(next));
            // Context switch would happen here via assembly
        }
    }
    
    /// Timer tick handler - called on each timer interrupt
    pub fn tick(&self) {
        self.tick += 1;
        
        // Wake up sleeping threads
        self.wake_sleeping();
        
        // Preempt current thread if time slice expired
        let cpu_id = arch::cpu_id();
        let rq = self.runqueues[cpu_id].lock();
        if let Some(current) = &rq.current {
            if current.runtime >= TIME_SLICE_MS {
                drop(rq);
                self.yield_current();
            }
        }
    }
    
    /// Put current thread to sleep for specified milliseconds
    pub fn sleep_current(&self, ms: u64) {
        let cpu_id = arch::cpu_id();
        let mut rq = self.runqueues[cpu_id].lock();
        
        if let Some(current) = rq.current.take() {
            current.set_state(ThreadState::Sleeping);
            current.wake_time = self.tick + ms;
            
            let mut sleep_q = self.sleep_queue.lock();
            sleep_q.push_back((current.wake_time, current));
            
            // Reschedule
            drop(rq);
            self.yield_current();
        }
    }
    
    /// Wake up threads whose sleep time has expired
    pub fn wake_sleeping(&self) {
        let mut sleep_q = self.sleep_queue.lock();
        let mut to_wake = Vec::new();
        
        let mut i = 0;
        while i < sleep_q.len() {
            let (wake_time, _) = sleep_q[i];
            if wake_time <= self.tick {
                to_wake.push(i);
            }
            i += 1;
        }
        
        // Remove in reverse order to maintain indices
        for i in to_wake.into_iter().rev() {
            if let Some((_, thread)) = sleep_q.remove(i) {
                thread.set_state(ThreadState::Runnable);
                
                // Add back to appropriate runqueue
                let cpu_id = arch::cpu_id();
                self.runqueues[cpu_id].lock().enqueue(thread);
            }
        }
    }
    
    /// Get scheduler statistics
    pub fn stats(&self) -> SchedulerStats {
        let mut total_threads = 0;
        let mut running_threads = 0;
        
        for rq in &self.runqueues {
            let rq = rq.lock();
            total_threads += rq.len();
            if rq.current.is_some() {
                running_threads += 1;
            }
        }
        
        SchedulerStats {
            total_threads,
            running_threads,
            sleeping_threads: self.sleep_queue.lock().len(),
            total_processes: self.processes.read().len(),
            tick: self.tick,
        }
    }
}

/// Scheduler statistics
#[derive(Debug, Clone)]
pub struct SchedulerStats {
    pub total_threads: usize,
    pub running_threads: usize,
    pub sleeping_threads: usize,
    pub total_processes: usize,
    pub tick: u64,
}

impl core::fmt::Display for SchedulerStats {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "Scheduler: {} threads ({} running, {} sleeping), {} processes, tick {}",
               self.total_threads, self.running_threads, self.sleeping_threads,
               self.total_processes, self.tick)
    }
}
