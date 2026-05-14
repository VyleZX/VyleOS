//! CPU register access utilities

use x86_64::registers::{control, rflags};

/// Read the current stack pointer
#[inline]
pub fn rsp() -> u64 {
    let rsp: u64;
    unsafe {
        core::arch::asm!(
            "mov {}, rsp",
            out(reg) rsp,
            options(nomem, nostack, preserves_flags),
        );
    }
    rsp
}

/// Read the current instruction pointer
#[inline]
pub fn rip() -> u64 {
    // RIP cannot be read directly, but we can get it via call
    let rip: u64;
    unsafe {
        core::arch::asm!(
            "call 0f",
            "0:",
            out("rax") rip,
            options(nostack),
        );
    }
    rip
}

/// Read CR0 control register
#[inline]
pub fn cr0() -> u64 {
    control::Cr0::read_raw()
}

/// Read CR2 control register (page fault address)
#[inline]
pub fn cr2() -> u64 {
    control::Cr2::read().as_u64()
}

/// Read CR3 control register (page table base)
#[inline]
pub fn cr3() -> u64 {
    control::Cr3::read().0.start_address().as_u64()
}

/// Read CR4 control register
#[inline]
pub fn cr4() -> u64 {
    control::Cr4::read_raw()
}

/// Read EFLAGS/RFLAGS register
#[inline]
pub fn rflags() -> u64 {
    rflags::read().bits()
}

/// Check if interrupts are enabled
#[inline]
pub fn interrupts_enabled() -> bool {
    rflags::read().contains(rflags::RFlags::INTERRUPT_FLAG)
}

/// Read FS base register
#[inline]
pub fn fs_base() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, fsbase",
                out(reg) val,
                options(nomem, nostack, preserves_flags),
            );
        }
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

/// Read GS base register
#[inline]
pub fn gs_base() -> u64 {
    #[cfg(target_arch = "x86_64")]
    {
        let val: u64;
        unsafe {
            core::arch::asm!(
                "mov {}, gsbase",
                out(reg) val,
                options(nomem, nostack, preserves_flags),
            );
        }
        val
    }
    #[cfg(not(target_arch = "x86_64"))]
    {
        0
    }
}

/// Write FS base register
#[inline]
pub fn write_fs_base(val: u64) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "wrfsbase {}",
            in(reg) val,
            options(nostack, preserves_flags),
        );
    }
}

/// Write GS base register
#[inline]
pub fn write_gs_base(val: u64) {
    #[cfg(target_arch = "x86_64")]
    unsafe {
        core::arch::asm!(
            "wrgsbase {}",
            in(reg) val,
            options(nostack, preserves_flags),
        );
    }
}

/// Save all general purpose registers to a struct
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct RegisterState {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rbp: u64,
    pub rsp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}

impl RegisterState {
    /// Capture current register state
    pub fn capture() -> Self {
        Self {
            rax: 0, // Cannot capture RAX from within this function
            rbx: 0,
            rcx: 0,
            rdx: 0,
            rsi: 0,
            rdi: 0,
            rbp: rbp(),
            rsp: rsp(),
            r8: 0,
            r9: 0,
            r10: 0,
            r11: 0,
            r12: 0,
            r13: 0,
            r14: 0,
            r15: 0,
            rip: 0,
            rflags: rflags(),
        }
    }
}

/// Read RBP register
#[inline]
pub fn rbp() -> u64 {
    let rbp: u64;
    unsafe {
        core::arch::asm!(
            "mov {}, rbp",
            out(reg) rbp,
            options(nomem, nostack, preserves_flags),
        );
    }
    rbp
}
