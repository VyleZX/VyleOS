//! I/O Port access for x86_64

use core::marker::PhantomData;

/// Represents an I/O port
pub struct Port<T> {
    port: u16,
    _marker: PhantomData<T>,
}

impl<T: PortType> Port<T> {
    /// Create a new port
    pub const fn new(port: u16) -> Self {
        Self {
            port,
            _marker: PhantomData,
        }
    }
    
    /// Read from the port
    #[inline]
    pub unsafe fn read(&mut self) -> T::Value {
        T::read(self.port)
    }
    
    /// Write to the port
    #[inline]
    pub unsafe fn write(&mut self, value: T::Value) {
        T::write(self.port, value);
    }
}

/// Trait for port data types
pub trait PortType {
    type Value;
    
    unsafe fn read(port: u16) -> Self::Value;
    unsafe fn write(port: u16, value: Self::Value);
}

/// 8-bit port operations
pub struct Port8;

impl PortType for Port8 {
    type Value = u8;
    
    #[inline]
    unsafe fn read(port: u16) -> u8 {
        let ret: u8;
        core::arch::asm!(
            "in al, dx",
            out("al") ret,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        ret
    }
    
    #[inline]
    unsafe fn write(port: u16, value: u8) {
        core::arch::asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags),
        );
    }
}

/// 16-bit port operations
pub struct Port16;

impl PortType for Port16 {
    type Value = u16;
    
    #[inline]
    unsafe fn read(port: u16) -> u16 {
        let ret: u16;
        core::arch::asm!(
            "in ax, dx",
            out("ax") ret,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        ret
    }
    
    #[inline]
    unsafe fn write(port: u16, value: u16) {
        core::arch::asm!(
            "out dx, ax",
            in("dx") port,
            in("ax") value,
            options(nomem, nostack, preserves_flags),
        );
    }
}

/// 32-bit port operations
pub struct Port32;

impl PortType for Port32 {
    type Value = u32;
    
    #[inline]
    unsafe fn read(port: u16) -> u32 {
        let ret: u32;
        core::arch::asm!(
            "in eax, dx",
            out("eax") ret,
            in("dx") port,
            options(nomem, nostack, preserves_flags),
        );
        ret
    }
    
    #[inline]
    unsafe fn write(port: u16, value: u32) {
        core::arch::asm!(
            "out dx, eax",
            in("dx") port,
            in("eax") value,
            options(nomem, nostack, preserves_flags),
        );
    }
}

// Type aliases for convenience
pub type PortU8 = Port<Port8>;
pub type PortU16 = Port<Port16>;
pub type PortU32 = Port<Port32>;

// Re-export most common type
pub use PortU8 as Port;
