#![no_std]
#![no_main]

mod uart;

use core::{arch::asm, panic::PanicInfo};

core::arch::global_asm!(include_str!("boot.s"));
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

unsafe fn init_sctlr_el1() {
    let ci_set: u64 = (1 << 2) | (1 << 12);
    unsafe {
        asm!(
        "mrs {0}, sctlr_el1",
        "orr {0}, {0}, {1}",
        "msr sctlr_el1, {0}",
        out(reg) _,
        in(reg) ci_set
        );
    }
}
unsafe fn init_vbar_el1() {
    unsafe {
        asm!(
            "ldr {0}, =vbar_entry",
            "msr vbar_el1, {0}",
            out(reg) _
        )
    }
}
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        init_sctlr_el1();
        init_vbar_el1();
    }
    uart::write_str("Hello World????????\n");
    loop {}
}
