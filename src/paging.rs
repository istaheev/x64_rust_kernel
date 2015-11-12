#[repr(C)]
pub struct PageTableEntry(usize);

impl PageTableEntry {

    pub fn empty() -> PageTableEntry {
        PageTableEntry(0)
    }

    pub fn is_present(&self) -> bool {
        let &PageTableEntry(ref pte) = self;
        *pte & 1 == 1
    }

    pub fn present(&mut self) {
        let &mut PageTableEntry(ref mut pte) = self;
        *pte = *pte | 1;
    }

    pub fn not_present(&mut self) {
        let &mut PageTableEntry(ref mut pte) = self;
        *pte = *pte & !1;
    }

    pub fn set_phys_addr(&mut self, phys_address: usize) {
        let &mut PageTableEntry(ref mut pte) = self;
        *pte = (*pte & ((1 << 12) - 1)) | phys_address;
    }
}

#[inline(never)]
pub fn paging_tests() {
    let mut pteptr = 0xb8000 as *mut PageTableEntry;
    unsafe {
        let mut pte = &mut *pteptr;
        pte.set_phys_addr(0x1000);
        pte.present();
    }
}


/* */
pub fn build_initial_memory_mapping() {

}