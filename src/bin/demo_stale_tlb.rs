#![no_std]
#![no_main]

use rustkernel::{init, shutdown, uart};

/// Milestone 3-11, the do-it-wrong-on-purpose half. Not part of `just test`
/// (demo_, not test_) — run it with `just run demo_stale_tlb` and read the
/// output yourself.
///
/// Remap a VA from frame A to frame B *without* a TLBI and watch the core
/// keep reading frame A through the stale entry; then do it right and watch
/// the same code read frame B. QEMU's software TLB generally reproduces
/// this; real hardware is even less forgiving.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    // TODO:
    //   1. MMU bring-up. Map scratch VA -> frame A; write marker 0xAAAA...
    //      through it. Read it back (translation is now cached in the TLB).
    //   2. Write marker 0xBBBB... into frame B directly (via its identity
    //      mapping), then rewrite the L3 entry to point at frame B —
    //      deliberately skipping BBM and TLBI.
    //   3. Read through scratch VA and print what you see: the stale entry
    //      still translates to frame A. Print translate(root, va) next to it —
    //      the tables say B, the TLB says A. Sit with that.
    //   4. Now mmu::tlbi_page(va), read again, print: frame B's marker.
    uart::write_str("demo_stale_tlb: not implemented\n");
    shutdown();
}
