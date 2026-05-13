//! Serial port driver for early logging

use super::ports::Port;
use spin::Mutex;
use core::fmt::{self, Write};

/// COM1 port address
const COM1: u16 = 0x3F8;

/// Serial port wrapper
pub struct SerialPort {
    data: Port<u8>,
    interrupt_enable: Port<u8>,
    fifo_control: Port<u8>,
    line_control: Port<u8>,
    modem_control: Port<u8>,
    line_status: Port<u8>,
}

impl SerialPort {
    /// Create a new serial port instance
    const fn new(base_port: u16) -> Self {
        Self {
            data: Port::new(base_port),
            interrupt_enable: Port::new(base_port + 1),
            fifo_control: Port::new(base_port + 2),
            line_control: Port::new(base_port + 3),
            modem_control: Port::new(base_port + 4),
            line_status: Port::new(base_port + 5),
        }
    }
    
    /// Initialize the serial port
    pub fn init(&mut self) {
        unsafe {
            // Disable interrupts
            self.interrupt_enable.write(0x00);
            
            // Enable DLAB (set baud rate divisor)
            self.line_control.write(0x80);
            
            // Set divisor to 1 (9600 baud)
            self.data.write(0x01);
            self.interrupt_enable.write(0x00);
            
            // 8 bits, no parity, one stop bit
            self.line_control.write(0x03);
            
            // Enable FIFO
            self.fifo_control.write(0xC7);
            
            // Enable IRQs, set RTS/DSR
            self.modem_control.write(0x0B);
        }
    }
    
    /// Check if transmitter is ready
    #[inline]
    fn is_transmit_empty(&self) -> bool {
        unsafe {
            (self.line_status.read() & 0x20) != 0
        }
    }
    
    /// Write a byte to the serial port
    pub fn write_byte(&mut self, byte: u8) {
        while !self.is_transmit_empty() {}
        unsafe {
            self.data.write(byte);
        }
    }
    
    /// Read a byte from the serial port (non-blocking)
    pub fn read_byte(&mut self) -> Option<u8> {
        unsafe {
            if (self.line_status.read() & 0x01) != 0 {
                Some(self.data.read())
            } else {
                None
            }
        }
    }
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            if byte == b'\n' {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

/// Global serial port instance
static SERIAL: Mutex<Option<SerialPort>> = Mutex::new(None);

/// Initialize the global serial port
pub fn init() {
    let mut port = SerialPort::new(COM1);
    port.init();
    
    *SERIAL.lock() = Some(port);
    
    log::info!("Serial port initialized on COM1");
}

/// Write a string to the serial port
pub fn write(s: &str) {
    let mut serial = SERIAL.lock();
    if let Some(ref mut port) = *serial {
        let _ = port.write_str(s);
    }
}

/// Write a byte to the serial port
pub fn write_byte(byte: u8) {
    let mut serial = SERIAL.lock();
    if let Some(ref mut port) = *serial {
        port.write_byte(byte);
    }
}

/// Read a byte from the serial port
pub fn read_byte() -> Option<u8> {
    let mut serial = SERIAL.lock();
    serial.as_mut().and_then(|p| p.read_byte())
}

/// Kernel logger implementation using serial port
pub struct SerialLogger;

impl log::Log for SerialLogger {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }
    
    fn log(&self, record: &log::Record) {
        let message = format!("[{}] {}\n", record.level(), record.args());
        write(&message);
    }
    
    fn flush(&self) {}
}

/// Initialize the kernel logger
pub fn init_logger() {
    static LOGGER: SerialLogger = SerialLogger;
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Info);
}
