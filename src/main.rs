#![no_std]
#![no_main]

mod uart;

use core::arch::global_asm;
use core::panic::PanicInfo;

global_asm!(
    ".section .text._start",
    ".global _start",
    "_start:",
    "mov x0, #0x4800", // \  build 0x48000000
    "lsl x0, x0, #16", // /  (top of QEMU's 128MB RAM)
    "mov sp, x0",      // set stack pointer
    "b kernel_main",   // jump into Rust
);

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart::write_str("Hello World????????\n");
    loop {}
}
