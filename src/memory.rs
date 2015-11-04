/* Returns address of a page which the specified address belongs to */
pub fn page_addr(addr: usize, page_size: usize) -> usize {
    (addr / page_size) * page_size
}

/* Returns address of the next page after a page the specified address belongs to */
pub fn next_page_addr(addr: usize, page_size: usize) -> usize {
    page_addr(addr, page_size) + page_size
}