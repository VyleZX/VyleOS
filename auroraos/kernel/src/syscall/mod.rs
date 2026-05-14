//! System call interface

use x86_64::instructions::interrupts;

/// Initialize syscall subsystem
pub fn init() {
    // Enable syscall/sysret instructions on x86_64
    enable_syscall();
}

/// Enable SYSCALL instruction
fn enable_syscall() {
    use x86_64::registers::{control, model_specific};
    
    // Set up STAR (System Call Target Address Register)
    // This defines the CS/SS selectors and RIP for syscall
    
    let kernel_cs = crate::arch::x86_64::gdt::kernel_code_selector().0;
    let user_cs = crate::arch::x86_64::gdt::user_code_selector().0;
    
    // Calculate values for STAR
    let star_value = ((kernel_cs as u64) << 32) | ((user_cs as u64) << 48);
    
    unsafe {
        model_specific::Star::write_raw(star_value);
    }
    
    // Set syscall entry point (LSTAR)
    unsafe {
        model_specific::LStar::write(syscall_entry as usize as u64);
    }
    
    // Set FMask (flags to clear on syscall)
    unsafe {
        model_specific::EFER::update(|efer| {
            *efer |= model_specific::EferFlags::SYSTEM_CALL_EXTENSIONS;
        });
    }
}

/// Syscall entry point (assembly stub would be here)
extern "C" fn syscall_entry() {
    // Save registers
    // Get syscall number from RAX
    // Dispatch to handler
    // Restore registers and return via SYSRET
}

/// Handle a system call
pub fn handle_syscall() {
    // Read syscall number from register
    // Dispatch to appropriate handler
    // Return result
    
    interrupts::enable();
}

/// System call numbers
pub mod numbers {
    /// Read from file descriptor
    pub const READ: u64 = 0;
    /// Write to file descriptor
    pub const WRITE: u64 = 1;
    /// Open a file
    pub const OPEN: u64 = 2;
    /// Close a file descriptor
    pub const CLOSE: u64 = 3;
    /// Get file status
    pub const STAT: u64 = 4;
    /// Map files or devices into memory
    pub const MMAP: u64 = 5;
    /// Unmap memory region
    pub const MUNMAP: u64 = 6;
    /// Create a process
    pub const FORK: u64 = 7;
    /// Execute a program
    pub const EXEC: u64 = 8;
    /// Exit current process
    pub const EXIT: u64 = 9;
    /// Wait for child process
    pub const WAIT: u64 = 10;
    /// Get process ID
    pub const GETPID: u64 = 11;
    /// Send signal to process
    pub const KILL: u64 = 12;
    /// Create socket
    pub const SOCKET: u64 = 13;
    /// Connect socket
    pub const CONNECT: u64 = 14;
    /// Bind socket to address
    pub const BIND: u64 = 15;
    /// Listen on socket
    pub const LISTEN: u64 = 16;
    /// Accept connection
    pub const ACCEPT: u64 = 17;
    /// Send data on socket
    pub const SEND: u64 = 18;
    /// Receive data from socket
    pub const RECV: u64 = 19;
    /// Memory allocation (custom)
    pub const BRK: u64 = 20;
    /// IOCTL device control
    pub const IOCTL: u64 = 21;
    /// Get time
    pub const GETTIME: u64 = 22;
    /// Yield CPU
    pub const YIELD: u64 = 23;
}

/// Syscall handler functions
pub mod handlers {
    use super::numbers;
    
    /// Handle read syscall
    pub fn sys_read(fd: u64, buf: u64, count: u64) -> i64 {
        let _ = (fd, buf, count);
        -1 // Not implemented
    }
    
    /// Handle write syscall
    pub fn sys_write(fd: u64, buf: u64, count: u64) -> i64 {
        let _ = (fd, buf, count);
        -1 // Not implemented
    }
    
    /// Handle exit syscall
    pub fn sys_exit(code: u64) -> ! {
        crate::scheduler::exit(code as i32)
    }
    
    /// Handle get PID syscall
    pub fn sys_getpid() -> u64 {
        1 // Init process
    }
}
