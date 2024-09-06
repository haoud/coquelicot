fn main() {
    // Tell cargo to pass the linker script to the linker and to rerun this
    // script if the linker script changes.
    println!("cargo:rerun-if-changed=../config/link.ld");
    println!("cargo:rustc-link-arg=../config/link.ld");
}
