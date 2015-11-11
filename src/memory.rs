/* Basic memory-related definitions */

/* What is the correct placement for this? */
pub const PAGE_SIZE: usize = 4096;

/* Defines arbitrary memory region */
pub struct MemoryRegion {
    pub addr: usize,
    pub size: usize
}

impl MemoryRegion {
    /* Returns address of the last byte in the region */
    pub fn end_addr(&self) -> usize {
        self.addr + self.size - 1
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