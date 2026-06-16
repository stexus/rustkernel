#![no_std]

use core::{arch::asm, panic::PanicInfo};

pub mod uart;
mod interrupt_handler;

core::arch::global_asm!(include_str!("boot.s"));

/// Address tests fault on, and the FAR the handler recovers. Shared so they can't drift.
pub const EXPECTED_FAULT_ADDR: u64 = 0xdeadbeaf;

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

/// Enable caches (SCTLR) and install the vector table (VBAR).
pub fn init() {
    unsafe {
        init_sctlr_el1();
        init_vbar_el1();
    }
}

/// Power off via PSCI SYSTEM_OFF (QEMU exits).
pub fn shutdown() -> ! {
    unsafe {
        asm!(
            "mov x0, {0}",
            "smc #0",
            in(reg) 0x84000008_u64,
        );
    }
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    uart::write_str("[FAIL]\n");
    uart::write_str("panic: ");
    uart::write_str(info.message().as_str().unwrap_or("unknown panic message"));
    uart::write_byte(b'\n');
    shutdown();
}
