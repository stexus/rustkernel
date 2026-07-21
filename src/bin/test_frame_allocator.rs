#![no_std]
#![no_main]

use rustkernel::memory::{DRAM_BASE, FrameAllocator, PAGE_SIZE};
use rustkernel::{init, shutdown, uart};

// 4 KiB bitmap — parked in .bss as a static so it never lands on the boot stack.
static mut FA: FrameAllocator = FrameAllocator::new();

/// Frame allocator round-trip: alloc order, free + reuse, reserve skipping.
/// A failed assert! panics -> panic handler emits [FAIL] -> harness fails us.
#[unsafe(no_mangle)]
pub extern "C" fn kernel_main() -> ! {
    init();
    let fa = unsafe { &mut *core::ptr::addr_of_mut!(FA) };

    // --- Property 1 (example): a fresh allocator hands out frame 0 first. ---
    let f0 = fa.alloc().expect("fresh allocator should have a free frame");
    assert_eq!(f0.as_u64(), DRAM_BASE);

    // --- Property 2 (you): what should the *second* alloc return, and why? ---

    // --- Property 3 (you): free(f0). What must the very next alloc return?
    //     (Remember what free() does to the search hint.) ---

    // --- Property 4 (you): reserve a range, then confirm alloc never hands
    //     back a frame inside it. ---

    let _ = PAGE_SIZE; // remove once you use it above

    uart::write_str("[PASS]\n");
    shutdown();
}
