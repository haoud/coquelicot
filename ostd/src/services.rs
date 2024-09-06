//! Services for the kernel.

/// Exit the kernel, shutting down the system. If running in QEMU, this will
/// cause QEMU to exit with a success code. If you want to exit with a failure
/// code, use [`ostd::panic::abort()`] instead.
pub fn exit() -> ! {
    x86_64::qemu::exit(x86_64::qemu::ExitCode::Success);
}
