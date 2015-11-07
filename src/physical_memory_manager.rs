/*
 * The module manages pages of physical memory.
 *
 */

use spin::Mutex;

use config;
use memory;
use multiboot::{PhysicalMemoryMap, MemoryRegionType};
use bitmap::Bitmap;

const PAGE_SIZE: usize = 4096;

pub static INSTANCE: Mutex<PhysicalMemoryManager> = Mutex::new(PhysicalMemoryManager {
    bitmap:            None,
    total_pages_count: 0,
    free_pages_count:  0
});

// IDEA: keep separate allocators for every available memory region

pub struct PhysicalMemoryManager {
    /* None is kept here until the allocator is initialized.
     * A try to use uninitialized allocator will cause panic.
     */
    bitmap: Option<Bitmap<'static>>,
    total_pages_count: u64,
    free_pages_count: u64,
}

impl PhysicalMemoryManager {

    // Warning: kernel stack and page tables set up by bootstrapper are not marked as occupied!
    pub fn init(&mut self, mem_map: &PhysicalMemoryMap) {
        /* Find the end of the available physical memory (end address of the last available memory region).
         * The allocator will use a region of 0..<end_address> for allocations. The region can have reserved areas
         * in it, they will be marked as already allocated in the allocator.
         */
        let last_avail_region = mem_map.memory_regions()
                                       .filter(|r| r.region_type == MemoryRegionType::Available)
                                       .last()
                                       .unwrap();

        /* Total amount of physical pages we have to control */
        let total_phys_pages = ((last_avail_region.end_address() + 1) / PAGE_SIZE) as u64;

        /* Amount of bytes required to store the memory bitmap */
        let mut bitmap_bytes = total_phys_pages / 8;
        if bitmap_bytes * 8 < total_phys_pages {
            bitmap_bytes = bitmap_bytes + 1;
        }

        /* The bitmap is placed immediately after the kernel aligned on a page boundary.
           XXX: It is assumed that there is enough available memory after the kernel. */
        // TODO: is alignment really required?
        let bitmap_addr = memory::next_page_addr(config::kernel_end_phys_addr(), PAGE_SIZE);
        let mut bitmap = Bitmap::from_raw_addr(bitmap_addr, total_phys_pages as usize);
        bitmap.clear();

        self.bitmap = Some(bitmap);
        self.total_pages_count = total_phys_pages;
        self.free_pages_count = total_phys_pages;

        /* Mark all memory as occupied */
        self.mark_region(0, last_avail_region.end_address() + 1, true);

        /* Mark all available regions from memory map as free */
        for region in mem_map.memory_regions().filter(|r| r.region_type == MemoryRegionType::Available) {
            self.mark_region(region.address, region.length, false);
        }

        /* Mark kernel location as occupied */
        self.mark_region(config::kernel_begin_phys_addr(), config::kernel_end_phys_addr() - config::kernel_begin_phys_addr(), true);

        /* Mark bitmap location as occupied */
        self.mark_region(bitmap_addr, bitmap_bytes as usize, true);
    }

    pub fn total_pages_count(&self) -> u64 {
        self.total_pages_count
    }

    pub fn free_pages_count(&self) -> u64 {
        self.free_pages_count
    }

    pub fn alloc_page(&mut self) -> Option<usize> {
        let bitmap = self.bitmap.as_mut().unwrap();
        match bitmap.find_first_zero() {
            Some(bit) => {
                bitmap.set_bit(bit);
                Some(bit * PAGE_SIZE)
            },
            None => None
        }
    }

    pub fn free_page(&mut self, addr: usize) {
        let bit = addr / PAGE_SIZE;
        self.mark_page(bit, false);
    }

    fn mark_region(&mut self, addr: usize, length: usize, occupied: bool) {
        let mut page = memory::page_addr(addr, PAGE_SIZE);
        while page < addr + length {
            self.mark_page(page, occupied);
            page = page + PAGE_SIZE;
        }
    }

    fn mark_page(&mut self, addr: usize, occupied: bool) {
        debug_assert_eq!(0, addr % PAGE_SIZE);

        let bitmap = self.bitmap.as_mut().unwrap();
        let bit = addr / PAGE_SIZE;
        if occupied {
            debug_assert!(!bitmap.is_bit_set(bit), "Bit already set for new page");
            bitmap.set_bit(bit);
            self.free_pages_count = self.free_pages_count - 1;
        } else {
            debug_assert!(bitmap.is_bit_set(bit), "Bit already cleared for occupied page");
            bitmap.clear_bit(bit);
            self.free_pages_count = self.free_pages_count + 1;
        }
    }
}
