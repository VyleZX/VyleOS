//! System Call Interface
//!
//! Provides the system call interface between userspace and kernel.

use x86_64::structures::idt::InterruptStackFrame;

pub mod numbers;

/// System call handler function type
type SyscallHandler = fn(&mut SyscallArgs) -> i64;

/// System call arguments
#[repr(C)]
pub struct SyscallArgs {
    pub arg0: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

impl SyscallArgs {
    pub const fn new() -> Self {
        Self {
            arg0: 0, arg1: 0, arg2: 0, arg3: 0, arg4: 0, arg5: 0,
        }
    }
}

/// System call table
pub struct SyscallTable {
    handlers: [Option<SyscallHandler>; 256],
}

impl SyscallTable {
    pub const fn new() -> Self {
        Self {
            handlers: [None; 256],
        }
    }
    
    pub fn register(&mut self, num: usize, handler: SyscallHandler) {
        if num < 256 {
            self.handlers[num] = Some(handler);
        }
    }
    
    pub fn call(&self, num: usize, args: &mut SyscallArgs) -> i64 {
        if num < 256 {
            if let Some(handler) = self.handlers[num] {
                return handler(args);
            }
        }
        -1 // Invalid syscall number
    }
}

/// Handle a system call from userspace
pub fn handle_syscall(num: usize, args: &mut SyscallArgs) -> i64 {
    static TABLE: spin::Once<SyscallTable> = spin::Once::new();
    
    let table = TABLE.call_once(|| {
        let mut t = SyscallTable::new();
        init_syscalls(&mut t);
        t
    });
    
    table.call(num, args)
}

/// Initialize the syscall table with all handlers
fn init_syscalls(table: &mut SyscallTable) {
    use self::numbers::*;
    
    table.register(SYS_READ, sys_read);
    table.register(SYS_WRITE, sys_write);
    table.register(SYS_OPEN, sys_open);
    table.register(SYS_CLOSE, sys_close);
    table.register(SYS_EXIT, sys_exit);
    table.register(SYS_FORK, sys_fork);
    table.register(SYS_EXEC, sys_exec);
    table.register(SYS_WAITPID, sys_waitpid);
    table.register(SYS_GETPID, sys_getpid);
    table.register(SYS_MMAP, sys_mmap);
    table.register(SYS_MUNMAP, sys_munmap);
    table.register(SYS_BRK, sys_brk);
    table.register(SYS_IOCTL, sys_ioctl);
    table.register(SYS_GETCWD, sys_getcwd);
    table.register(SYS_CHDIR, sys_chdir);
    table.register(SYS_STAT, sys_stat);
    table.register(SYS_FSTAT, sys_fstat);
    table.register(SYS_UNLINK, sys_unlink);
    table.register(SYS_MKDIR, sys_mkdir);
    table.register(SYS_RMDIR, sys_rmdir);
    table.register(SYS_GETDENTS, sys_getdents);
    table.register(SYS_SOCKET, sys_socket);
    table.register(SYS_CONNECT, sys_connect);
    table.register(SYS_SEND, sys_send);
    table.register(SYS_RECV, sys_recv);
    table.register(SYS_BIND, sys_bind);
    table.register(SYS_LISTEN, sys_listen);
    table.register(SYS_ACCEPT, sys_accept);
}

// System call implementations (stubs for now)

fn sys_read(args: &mut SyscallArgs) -> i64 {
    let _fd = args.arg0 as usize;
    let _buf = args.arg1 as *mut u8;
    let _count = args.arg2 as usize;
    // Would implement actual read
    -1
}

fn sys_write(args: &mut SyscallArgs) -> i64 {
    let _fd = args.arg0 as usize;
    let _buf = args.arg1 as *const u8;
    let _count = args.arg2 as usize;
    // Would implement actual write
    -1
}

fn sys_open(args: &mut SyscallArgs) -> i64 {
    let _path = args.arg0 as *const u8;
    let _flags = args.arg1 as i32;
    -1
}

fn sys_close(args: &mut SyscallArgs) -> i64 {
    let _fd = args.arg0 as usize;
    0
}

fn sys_exit(_args: &mut SyscallArgs) -> i64 {
    // Would terminate current process
    0
}

fn sys_fork(_args: &mut SyscallArgs) -> i64 {
    -1 // Not implemented
}

fn sys_exec(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_waitpid(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_getpid(_args: &mut SyscallArgs) -> i64 {
    crate::sched::current_process_id().map(|p| p as i64).unwrap_or(-1)
}

fn sys_mmap(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_munmap(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_brk(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_ioctl(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_getcwd(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_chdir(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_stat(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_fstat(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_unlink(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_mkdir(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_rmdir(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_getdents(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_socket(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_connect(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_send(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_recv(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_bind(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_listen(_args: &mut SyscallArgs) -> i64 {
    -1
}

fn sys_accept(_args: &mut SyscallArgs) -> i64 {
    -1
}
