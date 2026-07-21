//! MMU control: system-register setup, the enable sequence, and TLB
//! maintenance — Phase 3 (plan milestones 3-3 .. 3-8, 3-11).
//!
//! Everything here is MSR/MRS + barriers you write yourself (ground rule:
//! no helper crates). Register field layouts: ARM ARM sections on TCR_EL1,
//! MAIR_EL1, TTBR0_EL1, SCTLR_EL1, and the TLBI instruction pages.

use crate::memory::{PhysAddr, VirtAddr};

/// MAIR_EL1 slot indices — page descriptors name attributes by these.
/// Keep paging.rs and init_mair_el1() agreeing through the constants.
pub const ATTR_IDX_NORMAL: u64 = 0;
pub const ATTR_IDX_DEVICE: u64 = 1;

/// (3-4) Program MAIR_EL1: slot 0 = Normal memory, inner+outer write-back;
/// slot 1 = Device-nGnRnE.
pub unsafe fn init_mair_el1() {
    // TODO: each slot is one byte of MAIR_EL1; the encodings are in the
    // ARM ARM MAIR_EL1 description. Checkpoint question while you're here:
    // what would the CPU do to the UART if its page were Normal cacheable?
    todo!()
}

/// (3-3, 3-6) Program TCR_EL1: 4KB granule (TG0), 48-bit VA (T0SZ = 16),
/// inner-shareable write-back cacheable table walks (SH0/IRGN0/ORGN0),
/// 16-bit ASIDs (AS), and EPD1 = 1 so the unused TTBR1 half can't walk.
pub unsafe fn init_tcr_el1() {
    todo!()
}

/// (3-5) Point TTBR0_EL1 at the root table. The ASID lives in TTBR0[63:48] —
/// one register write switches both, which is exactly what makes the
/// context-switch TLB story work (deep dive: ASID rollover / nG).
pub unsafe fn install_ttbr0(root: PhysAddr, asid: u16) {
    let _ = (root, asid);
    todo!()
}

/// (3-7) Set SCTLR_EL1.M. The plan gives the barrier choreography
/// (dsb sy → isb → set M → dsb sy → isb); the checkpoint asks what each
/// barrier prevents — answer that BEFORE writing the asm, then break it on
/// purpose and see if you were right.
pub unsafe fn enable_mmu() {
    todo!()
}

/// Read SCTLR_EL1.M — sanity helper for tests and GDB sessions.
pub fn mmu_enabled() -> bool {
    todo!()
}

/// (3-11) Invalidate the translation for one page on every core in the
/// Inner Shareable domain, and don't return until they have all finished.
///
/// The recipe: `dsb ishst` → `tlbi vaae1is` → `dsb ish` → `isb`.
/// Know what each step individually guarantees (checkpoint question) —
/// and note the operand quirk: TLBI takes VA[55:12] in the low bits of the
/// register (a right-shifted address), not the raw VA.
///
/// Only after this returns may the caller free/reuse the old frame — and
/// only after this returns may a disconnected table page be freed (the walk
/// engine prefetches through stale table entries speculatively, 3-13).
pub unsafe fn tlbi_page(va: VirtAddr) {
    let _ = va;
    todo!()
}

/// Big hammer: invalidate all EL1 entries, Inner Shareable (`vmalle1is`).
/// Legitimate uses: ASID rollover, table teardown. Illegitimate use: hiding
/// a missing tlbi_page you couldn't find.
pub unsafe fn tlbi_all() {
    todo!()
}
