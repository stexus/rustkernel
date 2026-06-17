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
