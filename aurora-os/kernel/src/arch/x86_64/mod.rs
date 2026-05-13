//! x86_64 architecture implementation

pub mod gdt;
pub mod idt;
pub mod interrupts;
pub mod ports;
pub mod serial;

use x86_64::instructions::interrupts;
use x86_64::instructions::hlt;
use x86_64::registers::control::{Cr0, Cr0Flags};

/// Initialize x86_64 specific components
pub fn init() {
    // Initialize GDT
    gdt::init();
    
    // Initialize serial port for early logging
    serial::init();
    
    // Enable write protect in CR0
    unsafe {
        Cr0::update(|flags| flags.insert(Cr0Flags::WRITE_PROTECT));
    }
}

/// Halt the CPU until next interrupt
#[inline]
pub fn halt() {
    hlt();
}

/// Enable interrupts
#[inline]
pub fn enable_interrupts() {
    interrupts::enable();
}

/// Disable interrupts
#[inline]
pub fn disable_interrupts() {
    interrupts::disable();
}

/// Check if interrupts are enabled
#[inline]
pub fn interrupts_enabled() -> bool {
    interrupts::are_enabled()
}

/// Get the current CPU ID (APIC ID)
#[inline]
pub fn cpu_id() -> usize {
    #[cfg(feature = "apic")]
    {
        use x86_64::registers::model_specific::ApicBase;
        ApicBase::get().base_address() as usize
    }
    #[cfg(not(feature = "apic"))]
    {
        0
    }
}

/// Get CPU features via CPUID
pub fn get_cpu_features() -> CpuFeatures {
    use core::arch::x86_64::__cpuid;
    
    let cpuid1 = unsafe { __cpuid(1) };
    let cpuid7 = unsafe { __cpuid(7) };
    
    CpuFeatures {
        sse: (cpuid1.edx & (1 << 25)) != 0,
        sse2: (cpuid1.edx & (1 << 26)) != 0,
        sse3: (cpuid1.ecx & (1 << 0)) != 0,
        ssse3: (cpuid1.ecx & (1 << 9)) != 0,
        sse4_1: (cpuid1.ecx & (1 << 19)) != 0,
        sse4_2: (cpuid1.ecx & (1 << 20)) != 0,
        avx: (cpuid1.ecx & (1 << 28)) != 0,
        avx2: (cpuid7.ebx & (1 << 5)) != 0,
        fma: (cpuid1.ecx & (1 << 12)) != 0,
        bmi1: (cpuid7.ebx & (1 << 3)) != 0,
        bmi2: (cpuid7.ebx & (1 << 8)) != 0,
        apic: (cpuid1.edx & (1 << 9)) != 0,
        x2apic: (cpuid1.ecx & (1 << 21)) != 0,
        tsc: (cpuid1.edx & (1 << 4)) != 0,
        tsc_deadline: (cpuid1.ecx & (1 << 24)) != 0,
        lahf_sahf: (cpuid1.ecx & (1 << 0)) != 0,
        nx: (cpuid7.edx & (1 << 20)) != 0,
    }
}

/// CPU feature flags
#[derive(Debug, Clone, Copy)]
pub struct CpuFeatures {
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub fma: bool,
    pub bmi1: bool,
    pub bmi2: bool,
    pub apic: bool,
    pub x2apic: bool,
    pub tsc: bool,
    pub tsc_deadline: bool,
    pub lahf_sahf: bool,
    pub nx: bool,
}

impl CpuFeatures {
    /// Check if NX (No-eXecute) is supported
    pub fn has_nx(&self) -> bool {
        self.nx
    }
    
    /// Check if SSE is supported
    pub fn has_sse(&self) -> bool {
        self.sse && self.sse2
    }
    
    /// Check if AVX is supported
    pub fn has_avx(&self) -> bool {
        self.avx && self.avx2
    }
}
