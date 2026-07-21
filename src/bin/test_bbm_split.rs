#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// Milestones 3-12/3-13: split a live 2MB block into 4KB pages under
/// break-before-make, and prove the world didn't change.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    // TODO:
    //   1. MMU bring-up, but map a 2MB-aligned DRAM region with map_block_2mb.
    //   2. Write markers at several offsets through the block mapping.
    //   3. split_block() — the break/make windows live inside it.
    //   4. Read the markers back through the (now 4KB-grained) mapping:
    //      identical contents = the split was invisible, as required.
    //   5. Bonus proof the grain really changed: protect_page one 4KB page
    //      inside the range read-only, write to it, expect a permission
    //      fault — impossible while it was one 2MB block.
    //   6. Rule check while you're in there: split_block must not free or
    //      reuse anything until its TLBI completes (speculative walks).
    uart::write_str("[SKIP] bbm_split: not implemented\n");
    shutdown();
}
