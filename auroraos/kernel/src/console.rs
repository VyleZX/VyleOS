//! Console output for kernel debugging and logging

use core::fmt::{self, Write};
use spin::Mutex;
use lazy_static::lazy_static;

#[cfg(target_arch = "x86_64")]
use x86_64::instructions::port::Port;

/// VGA buffer address
const VGA_BUFFER: usize = 0xB8000;
const VGA_WIDTH: usize = 80;
const VGA_HEIGHT: usize = 25;

/// Console writer with VGA and serial output
pub struct Console {
    cursor_x: usize,
    cursor_y: usize,
}

lazy_static! {
    static ref CONSOLE: Mutex<Console> = Mutex::new(Console {
        cursor_x: 0,
        cursor_y: 0,
    });
}

/// Initialize the console subsystem
pub fn init() {
    let mut console = CONSOLE.lock();
    console.cursor_x = 0;
    console.cursor_y = 0;
    
    // Clear screen
    console.clear();
    
    #[cfg(feature = "serial")]
    init_serial();
}

#[cfg(feature = "serial")]
fn init_serial() {
    // Initialize serial port (COM1)
    unsafe {
        // Disable interrupts
        Port::<u8>::new(0x3F9).write(0x00);
        // Enable DLAB
        Port::<u8>::new(0x3FB).write(0x80);
        // Set divisor to 1 (115200 baud)
        Port::<u8>::new(0x3F8).write(0x01);
        Port::<u8>::new(0x3F9).write(0x00);
        // 8 bits, no parity, one stop bit
        Port::<u8>::new(0x3FB).write(0x03);
        // Enable FIFO
        Port::<u8>::new(0x3FC).write(0xC7);
        // Enable IRQs, RTS/DSR
        Port::<u8>::new(0x3FD).write(0x0B);
    }
}

impl Console {
    /// Clear the screen
    fn clear(&mut self) {
        let buffer = VGA_BUFFER as *mut u8;
        for i in 0..(VGA_WIDTH * VGA_HEIGHT * 2) {
            unsafe {
                buffer.add(i).write_volatile(0);
            }
        }
        self.cursor_x = 0;
        self.cursor_y = 0;
    }
    
    /// Write a character to VGA buffer
    fn write_char(&mut self, c: char) {
        match c {
            '\n' => {
                self.cursor_x = 0;
                self.cursor_y += 1;
            }
            '\r' => {
                self.cursor_x = 0;
            }
            '\t' => {
                self.cursor_x = (self.cursor_x + 8) & !7;
            }
            _ => {
                let buffer = VGA_BUFFER as *mut u8;
                let offset = (self.cursor_y * VGA_WIDTH + self.cursor_x) * 2;
                let color = 0x0F; // White on black
                
                unsafe {
                    buffer.add(offset).write_volatile(c as u8);
                    buffer.add(offset + 1).write_volatile(color);
                }
                
                self.cursor_x += 1;
                if self.cursor_x >= VGA_WIDTH {
                    self.cursor_x = 0;
                    self.cursor_y += 1;
                }
            }
        }
        
        // Scroll if needed
        if self.cursor_y >= VGA_HEIGHT {
            self.scroll();
            self.cursor_y = VGA_HEIGHT - 1;
        }
    }
    
    /// Scroll the screen up by one line
    fn scroll(&mut self) {
        let buffer = VGA_BUFFER as *mut u8;
        unsafe {
            // Move all lines up by one
            for y in 0..(VGA_HEIGHT - 1) {
                for x in 0..VGA_WIDTH {
                    let src_offset = ((y + 1) * VGA_WIDTH + x) * 2;
                    let dst_offset = (y * VGA_WIDTH + x) * 2;
                    let ch = buffer.add(src_offset).read_volatile();
                    let color = buffer.add(src_offset + 1).read_volatile();
                    buffer.add(dst_offset).write_volatile(ch);
                    buffer.add(dst_offset + 1).write_volatile(color);
                }
            }
            
            // Clear last line
            for x in 0..VGA_WIDTH {
                let offset = ((VGA_HEIGHT - 1) * VGA_WIDTH + x) * 2;
                buffer.add(offset).write_volatile(b' ');
                buffer.add(offset + 1).write_volatile(0x0F);
            }
        }
    }
    
    /// Write string to console
    fn write_str(&mut self, s: &str) {
        for c in s.chars() {
            self.write_char(c);
        }
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

/// Print to console (used by panic handler)
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    CONSOLE.lock().write_fmt(args).unwrap();
}

/// Serial port write (for debugging)
#[cfg(feature = "serial")]
pub fn serial_write(byte: u8) {
    unsafe {
        while Port::<u8>::new(0x3FD).read() & 0x20 == 0 {}
        Port::<u8>::new(0x3F8).write(byte);
    }
}

/// Macro for kernel printing
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::console::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
