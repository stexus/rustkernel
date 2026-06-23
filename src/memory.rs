#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryRegion {
    // CFI flash (`flash@0`).
    Firmware,
    // `intc@8000000`
    Gic,
    // PL011 (`pl011@9000000`).
    Uart,
    // (`memory@40000000`).
    Dram,
    Unmapped,
}

struct Region {
    base: u64,
    size: u64,
    region: MemoryRegion,
}

impl Region {
    const fn contains(&self, addr: u64) -> bool {
        addr >= self.base && addr - self.base < self.size
    }
}

const REGIONS: [Region; 5] = [
    // 64 MiB
    Region {
        base: 0x0000_0000,
        size: 0x0400_0000,
        region: MemoryRegion::Firmware,
    },
    // 64 KiB
    Region {
        base: 0x0800_0000,
        size: 0x0001_0000,
        region: MemoryRegion::Gic,
    },
    // 64KiB
    Region {
        base: 0x0801_0000,
        size: 0x0001_0000,
        region: MemoryRegion::Gic,
    },
    // 4KiB
    Region {
        base: 0x0900_0000,
        size: 0x0000_1000,
        region: MemoryRegion::Uart,
    },
    // 128MiB
    Region {
        base: 0x4000_0000,
        size: 0x0800_0000,
        region: MemoryRegion::Dram,
    },
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PhysAddr(u64);
pub struct VirtAddr(u64);

impl PhysAddr {
    pub const fn new(addr: u64) -> Self {
        PhysAddr(addr)
    }

    pub const fn as_u64(self) -> u64 {
        self.0
    }

    pub fn region(self) -> MemoryRegion {
        let mut i = 0;
        while i < REGIONS.len() {
            if REGIONS[i].contains(self.0) {
                return REGIONS[i].region;
            }
            i += 1;
        }
        MemoryRegion::Unmapped
    }
}

// ---------------------------------------------------------------------------
// Bitmap physical frame allocator
// ---------------------------------------------------------------------------

/// Size of one physical frame. One bit in the bitmap tracks one frame.
pub const PAGE_SIZE: u64 = 4096;

/// Where usable DRAM begins. Frame index 0 maps to this address.
pub const DRAM_BASE: u64 = 0x4000_0000;
/// Size of usable DRAM (hardcoded for QEMU `virt`).
pub const DRAM_SIZE: u64 = 0x0800_0000; // 128 MiB

/// Total number of frames in DRAM. 128 MiB / 4 KiB = 32768.
const FRAME_COUNT: usize = (DRAM_SIZE / PAGE_SIZE) as usize;
/// Number of `u64` words needed, one bit per frame. 32768 / 64 = 512.
const BITMAP_WORDS: usize = FRAME_COUNT / 64;
const BITMAP_BITS: usize = BITMAP_WORDS * 64;

/// Bitmap allocator. `0 = free`, `1 = used` (so a zeroed `.bss` starts all-free).
/// `next` is the search hint that keeps `alloc` amortized O(1).
pub struct FrameAllocator {
    bitmap: [u64; BITMAP_WORDS],
    next: usize, // word index to begin the next search from
}

impl FrameAllocator {
    pub const fn new() -> Self {
        FrameAllocator {
            bitmap: [0; BITMAP_WORDS],
            next: 0,
        }
    }

    /// Convert a frame index into (word index, bit within word).
    const fn locate(index: usize) -> (usize, usize) {
        (index >> 6, index & 0b11_1111)
    }

    /// Frame index -> physical address of that frame's base.
    const fn frame_addr(index: usize) -> PhysAddr {
        PhysAddr::new(DRAM_BASE + (index as u64) * PAGE_SIZE)
    }

    /// Physical address -> frame index. Caller must ensure addr is in DRAM.
    const fn frame_index(addr: PhysAddr) -> usize {
        ((addr.as_u64() - DRAM_BASE) / PAGE_SIZE) as usize
    }

    fn is_used(&self, index: usize) -> bool {
        let (word, bit) = Self::locate(index);
        (self.bitmap[word] >> bit) & 1 == 1
    }

    fn set_used(&mut self, index: usize) {
        let (word, bit) = Self::locate(index);
        self.bitmap[word] |= 1 << bit;
    }

    fn set_free(&mut self, index: usize) {
        let (word, bit) = Self::locate(index);
        self.bitmap[word] &= !(1 << bit);
    }

    /// Mark every frame overlapping `[start, end)` as used. Used at init to
    /// reserve the kernel image, MMIO, firmware, etc. (next milestone).
    pub fn reserve(&mut self, start: PhysAddr, end: PhysAddr) {
        // TODO: clamp the range to DRAM, convert to frame indices (round start
        // DOWN and end UP so partially-covered frames are fully reserved), and
        // set_used() across the range.
        // Clamp the requested range to DRAM; anything outside is ignored.
        let dram_end = DRAM_BASE + DRAM_SIZE;
        let start = start.as_u64().max(DRAM_BASE);
        let end = end.as_u64().min(dram_end);
        if start >= end {
            return;
        }
        // Round start DOWN and end UP so partially-covered frames are reserved.
        let start_idx = ((start - DRAM_BASE) / PAGE_SIZE) as usize;
        let end_idx = ((end - DRAM_BASE + PAGE_SIZE - 1) / PAGE_SIZE) as usize;
        for i in start_idx..end_idx {
            self.set_used(i);
        }
    }

    /// Allocate one free frame, or None if DRAM is full.
    pub fn alloc(&mut self) -> Option<PhysAddr> {
        // TODO:
        //   1. Scan words starting at `self.next`, wrapping once back to 0.
        //   2. Skip words equal to u64::MAX (all 64 frames used).
        //   3. In the first non-full word, the free bit is (!word).trailing_zeros().
        //   4. set_used() it, update self.next, return frame_addr(index).
        //   check if next is used
        //   if it is, take it and increment next; otherwise, continue incrementing next until we
        //   find a one, wrapping around when needed
        let mut free_idx = self.next;
        let mut count = 0;
        // Advance while frames are used; stop on the first free one.
        while count < BITMAP_BITS && self.is_used(free_idx) {
            free_idx = (free_idx + 1) % BITMAP_BITS;
            count += 1;
        }
        if count == BITMAP_BITS {
            None
        } else {
            self.set_used(free_idx);
            self.next = (free_idx + 1) % BITMAP_BITS;
            Some(Self::frame_addr(free_idx))
        }
    }

    /// Return a previously-allocated frame to the pool.
    pub fn free(&mut self, frame: PhysAddr) {
        // TODO:
        //   1. index = frame_index(frame).
        //   2. (optional debug) assert it's currently used and frame-aligned.
        //   3. set_free(index).
        //   4. Lower the search hint: self.next = min(self.next, word_of(index)).
        let _ = frame;
        todo!()
    }
}
