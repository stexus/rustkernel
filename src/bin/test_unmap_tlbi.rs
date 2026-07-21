#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// Milestone 3-11: unmap_page with the full TLBI recipe.
///
/// Sequence: map a scratch page → write/read a marker through it (this pulls
/// the translation into the TLB — that's the point) → unmap_page → touch it
/// again → the access MUST fault. If it doesn't, the TLB served the stale
/// entry and your tlbi_page is wrong or missing.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    // TODO:
    //   1. MMU bring-up, plus one scratch frame mapped at a scratch VA.
    //   2. write_volatile + read_volatile a marker through the scratch VA.
    //   3. unmap_page(root, scratch_va).
    //   4. Read the scratch VA again — expect a translation fault the handler
    //      recovers (it needs to know this VA is an expected fault; the
    //      EXPECTED_FAULT_ADDR pattern generalizes).
    //   5. Fault seen → [PASS]; read succeeds → [FAIL] stale TLB entry.
    uart::write_str("[SKIP] unmap_tlbi: not implemented\n");
    shutdown();
}
