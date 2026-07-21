#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// Milestone 3-9: with the MMU on, a load from an unmapped VA must take a
/// translation fault, and the handler must round-trip recover (same bar as
/// test_sync_exception, but now the fault comes from your page tables, not
/// from bare physical memory).
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    // TODO:
    //   1. Same MMU bring-up as test_mmu_enable (worth factoring into a
    //      shared helper in the library once both tests exist).
    //   2. Load from a VA you deliberately did not map (EXPECTED_FAULT_ADDR
    //      works — it's outside your tables; keep the handler's contract).
    //   3. Extend interrupt_handler to also decode ESR_EL1.DFSC (low 6 bits):
    //      translation faults are 0b0001LL where LL = the level that missed —
    //      print it, and check the level is the one you predicted.
    //   4. Reaching here after the fault = [PASS].
    uart::write_str("[SKIP] translation_fault: not implemented\n");
    shutdown();
}
