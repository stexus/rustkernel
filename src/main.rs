#![no_std]
#![no_main]

mod uart;

use core::panic::PanicInfo;

core::arch::global_asm!(include_str!("boot.s"));
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    uart::write_str("Hello World????????\n");
    loop {}
}
