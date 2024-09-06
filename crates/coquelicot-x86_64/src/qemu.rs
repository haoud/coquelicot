use crate::opcode;

/// Exit codes for QEMU.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u32)]
pub enum ExitCode {
    Success = 1,
    Failed = 2,
}

/// Exits QEMU with the given exit code. This function assumes that the kernel
/// is run in QEMU with the `isa-debug-exit` extension.
pub fn exit(code: ExitCode) -> ! {
    loop {
        unsafe {
            opcode::outd(0xF4, code as u32);
        }
    }
}
