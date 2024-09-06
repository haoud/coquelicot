use crate::opcode;

/// Represents a serial port. It should be the same on all `x86_64` systems,
/// since the `x86_64` architecture try to keep compatibility with the
/// original IBM PC. However, it's not guaranteed that the serial ports
/// are present on all systems.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Port {
    COM1 = 0x3F8,
    COM2 = 0x2F8,
    COM3 = 0x3E8,
    COM4 = 0x2E8,
}

impl From<Port> for u16 {
    fn from(port: Port) -> u16 {
        port as u16
    }
}

/// A serial port object. This object can be used to send and receive data
/// from a serial port.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Serial {
    port: Port,
}

impl Serial {
    /// Creates a new serial port object and initializes it. If the port is
    /// not present on the system or is faulty, this function returns `None`.
    #[must_use]
    pub fn new(port: Port) -> Option<Self> {
        // SAFETY: This should be safe because we use standard I/O ports to
        // initialize the serial device. If the serial device is not present,
        // the initialization will gracefully fail and return None.
        unsafe {
            // Disable interrupts
            opcode::outb(u16::from(port) + 1, 0x00);

            // Enable DLAB (set baud rate divisor)
            opcode::outb(u16::from(port) + 3, 0x80);

            // Set divisor to 3 (lo byte) 38400 baud
            opcode::outb(u16::from(port), 0x03);
            opcode::outb(u16::from(port) + 1, 0x00);

            // Set 8 bits, no parity, one stop bit
            opcode::outb(u16::from(port) + 3, 0x03);

            // Enable FIFO, clear them, with 14-byte threshold
            opcode::outb(u16::from(port) + 2, 0xC7);

            // IRQs enabled, RTS/DSR set
            opcode::outb(u16::from(port) + 4, 0x0B);

            // Set loopback mode (test the serial chip) and send a byte
            opcode::outb(u16::from(port) + 4, 0x1E);
            opcode::outb(u16::from(port), 0xAE);

            // If the byte is not echoed, the serial port is not present
            // or not functioning, and we should return None.
            if opcode::inb(u16::from(port)) != 0xAE {
                return None;
            }

            // Disable loopback mode, set the port to normal operation mode
            opcode::outb(u16::from(port) + 4, 0x0F);
        };

        Some(Self { port })
    }

    /// Sends a byte to the serial port. This function blocks until the
    /// port is ready to send data. If the port is not present or is
    /// faulty, this function may hang indefinitely !
    pub fn send(&self, byte: u8) {
        // SAFETY: This is safe because we checked in the `new` function
        // that the port is avaible and functioning. Readed or writting
        // to a serial port should not break memory safety.
        unsafe {
            while opcode::inb(u16::from(self.port) + 5) & 0x20 == 0 {
                core::hint::spin_loop();
            }
            opcode::outb(u16::from(self.port), byte);
        }
    }

    /// Receives a byte from the serial port. This function blocks until
    /// the port is ready to receive data. If the port is not present or is
    /// faulty, this function may hang indefinitely !
    pub fn recv(&self) -> u8 {
        // SAFETY: This is safe because we checked in the `new` function
        // that the port is avaible and functioning. Readed or writting to
        // a serial port should not break memory safety.
        unsafe {
            while opcode::inb(u16::from(self.port) + 5) & 0x01 == 0 {
                core::hint::spin_loop();
            }
            opcode::inb(u16::from(self.port))
        }
    }

    /// Tries to send a byte to the serial port. If the port is not ready
    /// to send data, this function immediately returns `Err(())` and does
    /// not block.
    pub fn try_send(&self, byte: u8) -> Result<(), ()> {
        // SAFETY: This is safe because we checked in the `new` function
        // that the port is avaible and functioning. Readed or writting
        // to a serial port should not break memory safety.
        unsafe {
            if opcode::inb(u16::from(self.port) + 5) & 0x20 != 0 {
                opcode::outb(u16::from(self.port), byte);
                return Ok(());
            }
        }

        Err(())
    }

    /// Tries to receive a byte from the serial port.
    ///
    /// # Errors
    /// If the port is not ready to receive data, this function immediately
    /// returns `Err(())` and does not block. Otherwise, it returns the byte
    /// that was received.
    pub fn try_recv(&self) -> Result<u8, ()> {
        // SAFETY: This is safe because we checked in the `new` function
        // that the port is avaible and functioning. Readed or writting to
        // a serial port should not break memory safety.
        unsafe {
            if opcode::inb(u16::from(self.port) + 5) & 0x01 != 0 {
                return Ok(opcode::inb(u16::from(self.port)));
            }
        }

        Err(())
    }
}

impl core::fmt::Write for Serial {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.send(byte);
        }
        Ok(())
    }
}
