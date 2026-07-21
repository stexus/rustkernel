//! AArch64 translation tables — Phase 3 (plan milestones 3-0 .. 3-14).
//!
//! 4KB granule, 48-bit VA: four levels (L0→L3), each level one 4KB table of
//! 512 8-byte descriptors. Field layouts live in ARM ARM section D8.3
//! ("VMSAv8-64 translation table format descriptors") and the "Learn the
//! Architecture: MMU" guide — read them there, don't trust a tutorial.

use crate::memory::{FrameAllocator, PAGE_SIZE, PhysAddr, VirtAddr};

pub const ENTRIES_PER_TABLE: usize = 512;
/// Translation levels with a 4KB granule and 48-bit VA.
pub const LEVELS: usize = 4;
/// VA range covered by one L2 block descriptor.
pub const BLOCK_SIZE_2MB: u64 = 2 * 1024 * 1024;

// TODO(3-1): descriptor bit constants. Define them here as you need them —
// all from ARM ARM D8.3. Fields you will end up needing:
//   VALID, TABLE/PAGE, AttrIndx (→ mmu::ATTR_IDX_*), AP[2:1], SH[1:0],
//   AF (access flag — see 3-14 for why it exists), nG (non-global — user
//   mappings MUST set it; see the "TLB Is Not Coherent" deep dive),
//   the output-address field, Contiguous hint, PXN, UXN.

/// How a page should be mapped. `Descriptor::page`/`block` translate this
/// into AttrIndx + AP + SH + UXN/PXN + nG bits.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageFlags {
    pub writable: bool,
    pub executable: bool,
    /// EL0-accessible. Implies nG=1 (non-global).
    pub user: bool,
    /// Device-nGnRnE instead of Normal write-back (MMIO: UART, GIC).
    pub device: bool,
}

impl PageFlags {
    pub const KERNEL_RW: PageFlags = PageFlags {
        writable: true,
        executable: false,
        user: false,
        device: false,
    };
    pub const KERNEL_RX: PageFlags = PageFlags {
        writable: false,
        executable: true,
        user: false,
        device: false,
    };
    pub const KERNEL_RO: PageFlags = PageFlags {
        writable: false,
        executable: false,
        user: false,
        device: false,
    };
    pub const DEVICE: PageFlags = PageFlags {
        writable: true,
        executable: false,
        user: false,
        device: true,
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MapError {
    /// Frame allocator ran dry while allocating an intermediate table.
    OutOfFrames,
    /// L3 slot already holds a valid entry — changing it is remap territory
    /// (break-before-make, 3-12), not map territory.
    AlreadyMapped,
    /// Nothing mapped at this VA.
    NotMapped,
    /// The walk hit a block descriptor where a table was expected —
    /// `split_block` (3-13) is the cure.
    BlockInTheWay,
    /// VA or PA not aligned to the requested mapping size.
    Misaligned,
}

/// One 8-byte translation table descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Descriptor(pub u64);

impl Descriptor {
    pub const INVALID: Descriptor = Descriptor(0);

    pub const fn is_valid(self) -> bool {
        todo!()
    }

    /// At L0–L2: points at a next-level table (vs. a block)? At L3 the same
    /// bit means "page" — decide how you want to expose that.
    pub const fn is_table(self) -> bool {
        todo!()
    }

    /// Physical address this descriptor outputs (next table, block, or frame).
    pub const fn output_addr(self) -> PhysAddr {
        todo!()
    }

    /// Build a table descriptor pointing at the next-level table `next`.
    pub fn table(next: PhysAddr) -> Descriptor {
        let _ = next;
        todo!()
    }

    /// Build an L3 page descriptor mapping `frame` with `flags`.
    pub fn page(frame: PhysAddr, flags: PageFlags) -> Descriptor {
        let _ = (frame, flags);
        todo!()
    }

    /// Build an L1/L2 block descriptor mapping `block` with `flags`.
    pub fn block(block: PhysAddr, flags: PageFlags) -> Descriptor {
        let _ = (block, flags);
        todo!()
    }
}

/// One 4KB translation table. 4KB-aligned so TTBR and table descriptors can
/// address it by frame number.
#[repr(C, align(4096))]
pub struct PageTable {
    pub entries: [Descriptor; ENTRIES_PER_TABLE],
}

impl PageTable {
    pub const fn empty() -> Self {
        PageTable {
            entries: [Descriptor::INVALID; ENTRIES_PER_TABLE],
        }
    }

    /// Allocate a zeroed table from the frame allocator; return the pointer
    /// and its physical address.
    ///
    /// While the kernel runs identity-mapped (or with the MMU off) a frame's
    /// PA doubles as a usable pointer — that's what makes bootstrapping
    /// possible. This assumption breaks at the higher-half move (3-10);
    /// leave yourself a loud comment when it does.
    pub fn alloc(falloc: &mut FrameAllocator) -> Option<(*mut PageTable, PhysAddr)> {
        // TODO: falloc.alloc(), cast the PA to *mut PageTable, zero it
        // (ptr::write of Self::empty()), return both.
        let _ = falloc;
        todo!()
    }
}

/// Map one 4KB page: walk L0→L3 from `root`, allocating intermediate tables
/// as needed, install a page descriptor at L3. (3-2)
pub fn map_page(
    root: &mut PageTable,
    va: VirtAddr,
    pa: PhysAddr,
    flags: PageFlags,
    falloc: &mut FrameAllocator,
) -> Result<(), MapError> {
    // TODO:
    //   1. Check va/pa page alignment.
    //   2. For level 0..3: index with va.table_index(level);
    //      - invalid entry  -> PageTable::alloc + install Descriptor::table
    //      - block entry    -> Err(BlockInTheWay)
    //      - table entry    -> descend via output_addr()
    //   3. At L3: valid entry -> Err(AlreadyMapped); else write Descriptor::page.
    //   4. NOTE(3-11): invalid->valid on a never-touched VA needs no TLBI —
    //      nothing can be cached for it. Every OTHER transition does.
    let _ = (root, va, pa, flags, falloc);
    todo!()
}

/// Map `count` consecutive pages starting at (`va`, `pa`).
pub fn map_range(
    root: &mut PageTable,
    va: VirtAddr,
    pa: PhysAddr,
    count: usize,
    flags: PageFlags,
    falloc: &mut FrameAllocator,
) -> Result<(), MapError> {
    for i in 0..count as u64 {
        map_page(
            root,
            VirtAddr::new(va.as_u64() + i * PAGE_SIZE),
            PhysAddr::new(pa.as_u64() + i * PAGE_SIZE),
            flags,
            falloc,
        )?;
    }
    Ok(())
}

/// Remove the L3 mapping for `va` and invalidate it everywhere. Returns the
/// frame that was mapped — the CALLER decides when it's safe to free it,
/// and "safe" means "after the TLBI completes", which this function must
/// guarantee before returning. (3-11)
pub fn unmap_page(root: &mut PageTable, va: VirtAddr) -> Result<PhysAddr, MapError> {
    // TODO:
    //   1. Walk to the L3 entry (no allocation this time); NotMapped on any
    //      invalid step.
    //   2. Save output_addr(), write Descriptor::INVALID.
    //   3. mmu::tlbi_page(va) — the full dsb/tlbi/dsb/isb recipe lives there.
    //   4. Return the old frame.
    let _ = (root, va);
    todo!()
}

/// Change permissions on an existing L3 mapping. Permission-only changes are
/// the one live transition that does NOT require break-before-make — but the
/// TLBI is still mandatory or old permissions linger in the TLB. (3-11)
pub fn protect_page(root: &mut PageTable, va: VirtAddr, flags: PageFlags) -> Result<(), MapError> {
    let _ = (root, va, flags);
    todo!()
}

/// Map a 2MB-aligned range with a single L2 block descriptor. (3-13)
pub fn map_block_2mb(
    root: &mut PageTable,
    va: VirtAddr,
    pa: PhysAddr,
    flags: PageFlags,
    falloc: &mut FrameAllocator,
) -> Result<(), MapError> {
    let _ = (root, va, pa, flags, falloc);
    todo!()
}

/// Split a live 2MB block into 512 4KB pages under break-before-make. (3-13)
///
/// This is the BBM dance for real: build the L3 table on the side (same
/// output addresses, same attributes), then break (INVALID + dsb + tlbi +
/// dsb), then make (install the table descriptor). Any core touching the
/// range mid-window takes a translation fault your handler must treat as
/// spurious — see the "TLB Is Not Coherent" deep dive.
pub fn split_block(
    root: &mut PageTable,
    va: VirtAddr,
    falloc: &mut FrameAllocator,
) -> Result<(), MapError> {
    let _ = (root, va, falloc);
    todo!()
}

/// Software walk: what does `va` translate to in these tables right now?
/// Pure table-reading, no TLB involved — which is exactly why it's useful:
/// when translate() and actual loads disagree, you're looking at a stale
/// TLB entry. Your best debugging tool this phase (pairs with GDB's
/// `monitor info mem`).
pub fn translate(root: &PageTable, va: VirtAddr) -> Option<PhysAddr> {
    // TODO: walk L0→L3, honoring block descriptors at L1/L2 (their offset
    // math differs from L3's).
    let _ = (root, va);
    todo!()
}
