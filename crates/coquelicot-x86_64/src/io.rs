use crate::opcode;

/// A trait for reading and writing values to I/O ports.
pub trait IO {
    /// Write a value to a port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side
    /// effects, including causing the hardware to do something unexpected
    /// and possibly violating memory safety.
    unsafe fn write(port: u16, value: Self);

    /// Read a value from a port.
    ///
    /// # Safety
    /// This function is unsafe because reading from a port can have side
    /// effects, including causing the hardware to do something unexpected
    /// and possibly violating memory safety.
    unsafe fn read(port: u16) -> Self;
}

impl IO for u8 {
    unsafe fn write(port: u16, value: u8) {
        opcode::outb(port, value);
    }

    unsafe fn read(port: u16) -> u8 {
        opcode::inb(port)
    }
}

impl IO for u16 {
    unsafe fn write(port: u16, value: u16) {
        opcode::outw(port, value);
    }

    unsafe fn read(port: u16) -> u16 {
        opcode::inw(port)
    }
}

impl IO for u32 {
    unsafe fn write(port: u16, value: u32) {
        opcode::outd(port, value);
    }

    unsafe fn read(port: u16) -> u32 {
        opcode::ind(port)
    }
}

/// Represents a port that can be read from and written to. This is a wrapper
/// around a port number and a type that implements the `IO` trait (currently
/// `u8`, `u16`, or `u32`).
#[derive(Debug)]
pub struct Port<T> {
    phantom: core::marker::PhantomData<T>,
    port: u16,
}

impl<T: IO> Port<T> {
    /// Create a new port. This function is safe because it does not access
    /// any hardware, it simply encapsulates a port number and a type that
    /// implements the `IO` trait.
    #[must_use]
    pub const fn new(port: u16) -> Port<T> {
        Port {
            port,
            phantom: core::marker::PhantomData,
        }
    }

    /// Read a value from the port.
    ///
    /// # Safety
    /// This function is unsafe because reading from a port can have side
    /// effects, including causing the hardware to do something unexpected
    /// and possibly violating memory safety.
    #[must_use]
    pub unsafe fn read(&self) -> T {
        T::read(self.port)
    }

    /// Write a value to the port.
    ///
    /// # Safety
    /// This function is unsafe because writing to a port can have side
    /// effects, including causing the hardware to do something unexpected
    /// and possibly violating memory safety.
    pub unsafe fn write(&self, value: T) {
        T::write(self.port, value);
    }
}
