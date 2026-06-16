#![no_std]
#![no_main]

use core::arch::asm;
use rustkernel::{EXPECTED_FAULT_ADDR, init, shutdown, uart};

/// Round-trip recovery test: fault on an unmapped read, the handler must catch
/// it and resume here. Reaching `[PASS]` proves take → handle → resume.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();

    unsafe {
        asm!(
            "ldr {tmp}, [{addr}]",
            addr = in(reg) EXPECTED_FAULT_ADDR,
            tmp = out(reg) _,
        );
    }

    uart::write_str("[PASS]\n");
    shutdown();
}
