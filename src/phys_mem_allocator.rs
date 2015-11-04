/*
 * The module manages pages of physical memory.
 *
 */

use config;
use memory;
use multiboot::{PhysicalMemoryMap, MemoryRegionType};

const PAGE_SIZE: usize = 4096;

// IDEA: keep separate allocators for every available memory region

pub struct PhysMemAllocator {
    bitmap_addr: usize,
    bitmap_size: usize
}

impl PhysMemAllocator {
    pub fn init(mem_map: &PhysicalMemoryMap) -> PhysMemAllocator {
        /* Find the end of the available physical memory (end address of the last available memory region).
         * The allocator will use a region of 0..<end_address> for allocations. The region can have reserved areas
         * in it, they will be marked as already allocated in the allocator.
         */

        let last_avail_region = mem_map.memory_regions()
                                       .filter(|r| r.region_type == MemoryRegionType::Available)
                                       .last()
                                       .unwrap();

        /* Total amount of physical pages we have to control */
        let total_phys_pages = (last_avail_region.end_address() + 1) / PAGE_SIZE;

        /* Size of bitmap required to keep information about all physical pages */
        let bitmap_size = total_phys_pages / 8;

        /* Amount of pages we need to store the bitmap */
        let total_bitmap_pages = bitmap_size / PAGE_SIZE;

        println!("PhysMemAllocator: total pages: {}, bitmap size: {} bytes ({} pages).", total_phys_pages, bitmap_size, total_bitmap_pages);

        /* The bitmap is placed immediately after the kernel aligned on a page boundary.
           XXX: It is assumed that there is enough available memory after the kernel. */
        // TODO: is alignment really required?
        let bitmap_addr = memory::next_page_addr(config::kernel_end_phys_addr(), PAGE_SIZE);

        let mut allocator = PhysMemAllocator {
            bitmap_addr: bitmap_addr,
            bitmap_size: bitmap_size
        };

        /* Mark all memory as occupied */
        allocator.mark_region(0, last_avail_region.end_address() + 1, true);

        /* Mark all available regions from memory map as free */
        for region in mem_map.memory_regions().filter(|r| r.region_type == MemoryRegionType::Available) {
            allocator.mark_region(region.address, region.length, false);
        }

        /* Mark kernel location as occupied */
        allocator.mark_region(config::kernel_begin_phys_addr(), config::kernel_end_phys_addr() - config::kernel_begin_phys_addr(), true);

        /* Mark bitmap location as occupied */
        allocator.mark_region(bitmap_addr, bitmap_size, true);

        // TODO: kernel stack and page tables set up by bootstrapper are not marked as occupied!

        allocator
    }

    pub fn alloc_page(&mut self) -> Option<usize> {
        for block_index in 0..self.bitmap_size {
            let ptr = unsafe { &*((self.bitmap_addr + block_index) as *const u8) };
            let mut b = *ptr;
            if b != 0xff {
                // there are free pages in the current block
                let mut b_index = 0;
                while b % 2 == 1 {
                    b = b >> 1;
                    b_index = b_index + 1;
                }
                let page_addr = (block_index * 8 + b_index) * PAGE_SIZE;
                self.mark_page(page_addr, true);
                return Some(page_addr);
            }
        }
        None
    }

    fn mark_region(&mut self, addr: usize, length: usize, occupied: bool) {
        let mut page = memory::page_addr(addr, PAGE_SIZE);
        while page <= addr + length {
            self.mark_page(page, occupied);
            page = page + PAGE_SIZE;
        }
    }

    fn mark_page(&mut self, addr: usize, occupied: bool) {
        debug_assert_eq!(0, addr % PAGE_SIZE);
        let page_index = addr / PAGE_SIZE;
        let ptr = unsafe { &mut *((self.bitmap_addr + (page_index / 8))  as *mut u8) };
        if occupied {
            *ptr = *ptr | (1 << (page_index % 8));
        } else {
            *ptr = *ptr & !(1 << (page_index % 8));
        }
    }
}
