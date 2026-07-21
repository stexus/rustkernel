#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// Milestones 3-0..3-8: identity-map the kernel and turn the MMU on.
///
/// The verdict is printed THROUGH the new mapping — if the UART page's
/// device attributes or any kernel mapping is wrong, the tell is silence
/// (or garbage), and `just test` reports "no verdict".
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    // TODO:
    //   1. Bring up a FrameAllocator; reserve the kernel image + MMIO + firmware
    //      (Phase 2's reserve()).
    //   2. PageTable::alloc a root; identity-map DRAM (PageFlags::KERNEL_RW —
    //      split .text as KERNEL_RX if you're feeling honest) and the UART page
    //      (PageFlags::DEVICE) with map_range/map_page.
    //   3. mmu::init_mair_el1(); mmu::init_tcr_el1(); mmu::install_ttbr0(root_pa, 0);
    //   4. mmu::enable_mmu();
    //   5. assert!(mmu::mmu_enabled()), then print the verdict.
    uart::write_str("[SKIP] mmu_enable: not implemented\n");
    shutdown();
}
