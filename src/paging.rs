use spin::Mutex;
use physical_memory_manager;

type PageTable = [PageTableEntry; 512];

// TODO: this should be probably replaced with current_process
// structure holding page directory together with other information.
// Of course, only if the OS is process-based.

/* Currently active PML4 (physical address) */
static PML4: Mutex<usize> = Mutex::new(0);

pub unsafe fn set_cr3(addr: usize) {
    asm!("mov $0, %cr3"
         : /* outputs */
         : "r" (addr)
         : "memory"
         : "volatile");

}

pub fn map(physical_addr: usize, virtual_addr: usize) {
    // TODO
}

pub fn unmap(virtual_addr: usize) {
    // TODO
}

/* Resets the identity memory mapping prepared for us by the bootstrapper.
 * After this function finishes the kernel and the physical memory allocator
 * will only be mapped to their virtual placements. */
pub unsafe fn reset_bootstrap_paging() {
    let pml4_addr = physical_memory_manager::INSTANCE.lock().alloc_page().unwrap();
    let mut pml4 = &mut *(pml4_addr as *mut PageTable);

    /* XXX: here it is assumed that the allocated pages lie in the memory which
     * is identity-mapped by the bootstrapper and are accessible by their physical
     * address. This might change if the design of physical memory manager is changed,
     * for example. Be cautious. */
     // TODO: add bootstrap_allocated_memory* functions to layout.rs to check allocated
     // physical addresses against and see if they are accessible

     for pte in pml4.iter_mut() {
        pte.clear();
     }
}


#[repr(C)]
pub struct PageTableEntry(usize);

impl PageTableEntry {

    pub fn empty() -> PageTableEntry {
        PageTableEntry(0)
    }

    pub fn clear(&mut self) {

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
