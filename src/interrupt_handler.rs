use crate::{EXPECTED_FAULT_ADDR, shutdown, uart};
use core::arch::asm;

// Data Abort, same Exception Level.
const EC_DATA_ABORT_SAME_EL: u64 = 0x25;

/// Shared sync-exception handler. Returns so boot.s can `eret` and resume.
/// Recovers the one expected fault (ELR += 4); fails closed on anything else.
/// Hard-coded to one fault for now — see docs/adr/0001-kernel-test-harness.md.
#[unsafe(no_mangle)]
pub extern "C" fn interrupt_handler(esr: u64, far: u64) {
    let exception_class = (esr >> 26) & 0x3F;

    if exception_class == EC_DATA_ABORT_SAME_EL && far == EXPECTED_FAULT_ADDR {
        // Skip the faulting instruction (4 bytes).
        unsafe {
            asm!(
                "mrs {0}, elr_el1",
                "add {0}, {0}, #4",
                "msr elr_el1, {0}",
                out(reg) _,
            );
        }
        return;
    }

    uart::write_str("[FAIL]\n");
    uart::write_str("unexpected exception\n");
    uart::write_str("exception class: ");
    uart::print_hex_u64(exception_class);
    uart::write_str("\n");
    uart::write_reg("esr", esr);
    uart::write_reg("far", far);
    shutdown();
}
