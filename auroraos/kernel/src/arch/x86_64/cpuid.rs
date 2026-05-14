//! CPUID instruction wrapper for CPU feature detection

use core::arch::x86_64::__cpuid;
use bitflags::bitflags;

/// CPU feature flags detected via CPUID
#[derive(Debug, Clone, Copy)]
pub struct CpuFeatures {
    pub fpu: bool,
    pub mmx: bool,
    pub sse: bool,
    pub sse2: bool,
    pub sse3: bool,
    pub ssse3: bool,
    pub sse4_1: bool,
    pub sse4_2: bool,
    pub avx: bool,
    pub avx2: bool,
    pub apic: bool,
    pub x2apic: bool,
    pub hypervisor: bool,
    pub smep: bool,
    pub smap: bool,
    pub nx: bool,
}

impl CpuFeatures {
    /// Detect CPU features using CPUID
    pub fn detect() -> Self {
        let cpuid_0000_0001 = __cpuid(0x0000_0001);
        let cpuid_0000_0007 = __cpuid(0x0000_0007);
        let cpuid_0000_0000 = __cpuid(0x0000_0000);
        
        let has_hypervisor = cpuid_0000_0001.ecx & (1 << 31) != 0;
        
        Self {
            fpu: cpuid_0000_0001.edx & (1 << 0) != 0,
            mmx: cpuid_0000_0001.edx & (1 << 23) != 0,
            sse: cpuid_0000_0001.edx & (1 << 25) != 0,
            sse2: cpuid_0000_0001.edx & (1 << 26) != 0,
            sse3: cpuid_0000_0001.ecx & (1 << 0) != 0,
            ssse3: cpuid_0000_0001.ecx & (1 << 9) != 0,
            sse4_1: cpuid_0000_0001.ecx & (1 << 19) != 0,
            sse4_2: cpuid_0000_0001.ecx & (1 << 20) != 0,
            avx: cpuid_0000_0001.ecx & (1 << 28) != 0,
            avx2: cpuid_0000_0007.ebx & (1 << 5) != 0,
            apic: cpuid_0000_0001.edx & (1 << 9) != 0,
            x2apic: cpuid_0000_0001.ecx & (1 << 21) != 0,
            hypervisor: has_hypervisor,
            smep: cpuid_0000_0007.ebx & (1 << 7) != 0,
            smap: cpuid_0000_0007.ebx & (1 << 20) != 0,
            nx: cpuid_0000_8000_0001().edx & (1 << 20) != 0,
        }
    }
    
    /// Check if all required features are present
    pub fn verify_requirements(&self) -> Result<(), &'static str> {
        if !self.sse2 {
            return Err("SSE2 is required but not supported");
        }
        if !self.nx {
            return Err("NX (No-Execute) is required but not supported");
        }
        Ok(())
    }
}

fn cpuid_0000_8000_0001() -> core::arch::x86_64::CpuidResult {
    __cpuid(0x8000_0001)
}

/// Get CPU vendor string
pub fn get_vendor() -> &'static str {
    let cpuid = __cpuid(0x0000_0000);
    
    let mut vendor = [0u8; 12];
    vendor[0..4].copy_from_slice(&cpuid.ebx.to_le_bytes());
    vendor[4..8].copy_from_slice(&cpuid.edx.to_le_bytes());
    vendor[8..12].copy_from_slice(&cpuid.ecx.to_le_bytes());
    
    match &vendor[..] {
        b"GenuineIntel" => "Intel",
        b"AuthenticAMD" => "AMD",
        b"CentaurHauls" => "Centaur",
        _ => "Unknown",
    }
}

/// Get CPU brand string
pub fn get_brand() -> [u8; 48] {
    let mut brand = [0u8; 48];
    
    for i in 0..3 {
        let cpuid = __cpuid(0x8000_0002 + i);
        brand[i * 16..(i + 1) * 16].copy_from_slice(&cpuid.eax.to_le_bytes());
        brand[i * 16 + 4..(i + 1) * 16].copy_from_slice(&cpuid.ebx.to_le_bytes());
        brand[i * 16 + 8..(i + 1) * 16].copy_from_slice(&cpuid.ecx.to_le_bytes());
        brand[i * 16 + 12..(i + 1) * 16].copy_from_slice(&cpuid.edx.to_le_bytes());
    }
    
    brand
}

/// Get number of logical processors
pub fn get_logical_processor_count() -> u32 {
    let cpuid = __cpuid(0x0000_0001);
    ((cpuid.ebx >> 16) & 0xFF) as u32
}

/// Get APIC ID
pub fn get_apic_id() -> u32 {
    let cpuid = __cpuid(0x0000_0001);
    (cpuid.ebx >> 24) as u32
}

bitflags! {
    /// Extended feature flags from CPUID leaf 7
    #[derive(Debug, Clone, Copy)]
    pub struct ExtendedFeatures: u32 {
        const FSGSBASE = 1 << 0;
        const TSC_ADJUST = 1 << 1;
        const BMI1 = 1 << 3;
        const SMEP = 1 << 7;
        const BMI2 = 1 << 8;
        const REP_MOVSB = 1 << 9;
        const INVPCID = 1 << 10;
        const MPX = 1 << 14;
        const AVX512F = 1 << 16;
        const AVX512DQ = 1 << 17;
        const RDSEED = 1 << 18;
        const SMAP = 1 << 20;
        const CLFLUSHOPT = 1 << 23;
        const SHA = 1 << 29;
    }
}

/// Get extended feature flags
pub fn get_extended_features() -> ExtendedFeatures {
    let cpuid = __cpuid(0x0000_0007);
    ExtendedFeatures::from_bits_truncate(cpuid.ebx)
}
