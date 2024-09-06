//! Panic support for the kernel.
use crate::logging;

/// Aborts the execution of the program. This function should only be called
/// when something goes horribly wrong and the program cannot continue. It
/// will cause QEMU to exit with a failed status code.
pub fn abort() -> ! {
    x86_64::qemu::exit(x86_64::qemu::ExitCode::Failed);
}

/// The panic handler, called when a panic occurs.
#[panic_handler]
pub fn panic(info: &core::panic::PanicInfo) -> ! {
    logging::print(format_args!("Fatal error: {} ", info.message()));
    if let Some(location) = info.location() {
        logging::print(format_args!(
            "at {}:{}:{}\n",
            location.file(),
            location.line(),
            location.column()
        ));
    } else {
        logging::print(format_args!("(no location information)\n"));
    }
    abort();
}
