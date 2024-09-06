//! This module provides utilities for logging messages. By default, coquelicot
//! ostd uses the first serial port (COM1) for displaying log messages. If no
//! serial port is available, log messages will simply be ignored.
//!
//! All functions in this module are synchronous and will block until the log
//! message has been written to the serial port. This can greatly slow down the
//! system, so it is recommended to only use logging for debugging purposes and
//! should be disabled in production builds.
use core::fmt::Write;
use sync::{Lazy, Spinlock};
use x86_64::serial::{self, Serial};

struct Logger {
    serial: Option<Serial>,
}

/// The global logger instance, protected by a spinlock to allow for concurrent
/// access.
static LOGGER: Lazy<Spinlock<Logger>> = Lazy::new(|| {
    Spinlock::new(Logger {
        serial: Serial::new(serial::Port::COM1),
    })
});

/// Print a message to the log.
pub fn print(args: core::fmt::Arguments) {
    if let Some(serial) = &mut LOGGER.lock().serial {
        _ = serial.write_fmt(args);
    }
}
