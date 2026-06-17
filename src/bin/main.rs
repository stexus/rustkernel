#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// The real kernel image (default for `just run` / `just debug`); not a test.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    uart::write_str("rustkernel booted\n");
    shutdown();
    loop {}
}
