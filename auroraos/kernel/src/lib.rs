//! AuroraOS Kernel Library
//!
//! This module exports kernel functionality for use by other kernel components.

#![no_std]
#![feature(naked_functions)]
#![feature(asm_const)]

extern crate alloc;

pub mod arch;
pub mod boot;
pub mod console;
pub mod drivers;
pub mod fs;
pub mod ipc;
pub mod loader;
pub mod memory;
pub mod net;
pub mod scheduler;
pub mod security;
pub mod smp;
pub mod syscall;

// Re-export important types
pub use arch::x86_64;
pub use memory::{FrameAllocator, PageTable};
pub use scheduler::{Process, Thread, ProcessId};
pub use fs::{VfsNode, FileType};
pub use ipc::{Message, Port};
pub use security::{Capabilities, Permissions};
