#![no_std]
#![no_main]

#[no_mangle]
pub extern "C" fn start() {
    ostd::logging::print(format_args!("Hello, world!\n"));
    ostd::services::exit();
}
