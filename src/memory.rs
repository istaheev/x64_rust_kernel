/* Basic memory-related definitions */

/* What is the correct placement for this? */
pub const PAGE_SIZE: usize = 4096;

/* Defines arbitrary memory region */
#[derive(Debug)]
pub struct MemoryRegion {
    pub addr: usize,
    pub size: usize
}

impl MemoryRegion {
    pub fn addr_in(&self, addr: usize) -> bool {
        addr >= self.addr && addr < self.addr + self.size
    }

    /* Returns address of the last byte in the region */
    pub fn end_addr(&self) -> usize {
        self.addr + self.size - 1
    }

    /* Returns next address immediately after the end of the region */
    pub fn next_addr_after(&self) -> usize {
        self.addr + self.size
    }

    /* Returns next region of specified size adjacent to this one */
    pub fn next_adjacent(&self, size: usize) -> MemoryRegion {
        MemoryRegion {
            addr: self.addr + self.size,
            size: size
        }
    }

    /* Returns new minimal region which contains the source region and
     * both its boundaries are page aligned */
    pub fn page_align(&self, page_size: usize) -> MemoryRegion {
        let start_addr = page_addr(self.addr, page_size);
        MemoryRegion {
            addr: start_addr,
            size: next_page_addr(self.end_addr(), page_size) - start_addr
        }
    }
}

/* Returns address of a page which the specified address belongs to */
pub fn page_addr(addr: usize, page_size: usize) -> usize {
    (addr / page_size) * page_size
}

/* Returns address of the next page after a page the specified address belongs to */
pub fn next_page_addr(addr: usize, page_size: usize) -> usize {
    page_addr(addr, page_size) + page_size
}