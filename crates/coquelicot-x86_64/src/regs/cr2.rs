/// Read the value of the CR2 register on the current core.
#[must_use]
pub fn read() -> usize {
    let addr: usize;

    // SAFETY: This is safe because reading the cr2 register should not break
    // Rust's safety guarantees nor lead to undefined behavior.
    unsafe {
        core::arch::asm!("mov {}, cr2", out(reg) addr);
    }
    addr
}
