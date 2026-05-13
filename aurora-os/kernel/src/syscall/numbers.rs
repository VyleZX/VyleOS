//! System Call Numbers
//!
//! These constants define the syscall numbers used by userspace programs.

/// Read from file descriptor
pub const SYS_READ: usize = 0;

/// Write to file descriptor
pub const SYS_WRITE: usize = 1;

/// Open a file
pub const SYS_OPEN: usize = 2;

/// Close a file descriptor
pub const SYS_CLOSE: usize = 3;

/// Exit current process
pub const SYS_EXIT: usize = 60;

/// Create a new process
pub const SYS_FORK: usize = 57;

/// Execute a program
pub const SYS_EXEC: usize = 59;

/// Wait for child process
pub const SYS_WAITPID: usize = 61;

/// Get process ID
pub const SYS_GETPID: usize = 39;

/// Memory map a file
pub const SYS_MMAP: usize = 9;

/// Unmap memory region
pub const SYS_MUNMAP: usize = 11;

/// Change data segment size
pub const SYS_BRK: usize = 12;

/// Device-specific operations
pub const SYS_IOCTL: usize = 16;

/// Get current working directory
pub const SYS_GETCWD: usize = 79;

/// Change directory
pub const SYS_CHDIR: usize = 80;

/// Get file status
pub const SYS_STAT: usize = 4;

/// Get file status by FD
pub const SYS_FSTAT: usize = 5;

/// Unlink (delete) a file
pub const SYS_UNLINK: usize = 87;

/// Create a directory
pub const SYS_MKDIR: usize = 83;

/// Remove a directory
pub const SYS_RMDIR: usize = 84;

/// Get directory entries
pub const SYS_GETDENTS: usize = 78;

// Socket syscalls
pub const SYS_SOCKET: usize = 41;
pub const SYS_CONNECT: usize = 42;
pub const SYS_SEND: usize = 44;
pub const SYS_RECV: usize = 45;
pub const SYS_BIND: usize = 49;
pub const SYS_LISTEN: usize = 50;
pub const SYS_ACCEPT: usize = 43;
