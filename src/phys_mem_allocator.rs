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
    bitmap: Bitmap<'static>,
    total_pages_count: u64,
    free_pages_count: u64,
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

        let mut allocator = PhysMemAllocator {
            bitmap: bitmap,
            total_pages_count: total_phys_pages,
            free_pages_count: total_phys_pages,
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
        allocator.mark_region(bitmap_addr, bitmap_bytes as usize, true);

        // TODO: kernel stack and page tables set up by bootstrapper are not marked as occupied!

        allocator
    }

    pub fn total_pages_count(&self) -> u64 {
        self.total_pages_count
    }

    pub fn free_pages_count(&self) -> u64 {
        self.free_pages_count
    }

    pub fn alloc_page(&mut self) -> Option<usize> {
        match self.bitmap.find_first_zero() {
            Some(bit) => {
                self.bitmap.set_bit(bit);
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

        let bit = addr / PAGE_SIZE;
        if occupied {
            debug_assert!(!self.bitmap.is_bit_set(bit), "Bit already set for new page");
            self.bitmap.set_bit(bit);
            self.free_pages_count = self.free_pages_count - 1;
        } else {
            debug_assert!(self.bitmap.is_bit_set(bit), "Bit already cleared for occupied page");
            self.bitmap.clear_bit(bit);
            self.free_pages_count = self.free_pages_count + 1;
        }
    }
}

/*
 * Bitmap implementation (only for little-endian architectures)
 */

use core::mem::size_of;
use core::slice::from_raw_parts_mut;

type BitmapBlock = u64;

struct Bitmap<'a> {
    array: &'a mut [BitmapBlock],
    /* Size of the bitmap in bits */
    size: usize
}

impl<'a> Bitmap<'a> {

    pub fn new(array: &'a mut [BitmapBlock], size: usize) -> Bitmap<'a> {
        assert!(size <= array.len() * size_of::<BitmapBlock>() * 8,
                "Array for bitmap has insufficient space to store specified amount of bits.");

        Bitmap {
            array: array,
            size: size
        }
    }

    pub fn from_raw_addr(addr: usize, size: usize) -> Bitmap<'static> {
        debug_assert!(size > 0, "Creating bitmap of zero size");

        let mut array_size = size / (size_of::<BitmapBlock>() * 8);
        if array_size * size_of::<BitmapBlock>() * 8 < size {
            array_size = array_size + 1
        }

        let array = unsafe {
            from_raw_parts_mut(addr as *mut BitmapBlock , array_size)
        };

        Bitmap {
            array: array,
            size: size
        }
    }

    /* Clear the entire bitmap (set all bits to zero) */
    pub fn clear(&mut self) {
        /* Clear entire blocks first */
        let full_blocks_count = self.size / (size_of::<BitmapBlock>() * 8);
        for block in 0..full_blocks_count {
            self.array[block] = 0;
        }

        /* For last bits which form an incompleted block clear them individually */
        for bit in (full_blocks_count * size_of::<BitmapBlock>() * 8) .. self.size {
            self.clear_bit(bit);
        }
    }

    pub fn set_bit(&mut self, bit: usize) {
        debug_assert!(bit < self.size, "Out of bitmap range");
        self.array[bit / (size_of::<BitmapBlock>() * 8)] |= 1 << (bit % (size_of::<BitmapBlock>() * 8));
    }

    pub fn clear_bit(&mut self, bit: usize) {
        debug_assert!(bit < self.size, "Out of bitmap range");
        self.array[bit / (size_of::<BitmapBlock>() * 8)] &= !(1 << (bit % (size_of::<BitmapBlock>() * 8)));
    }

    pub fn is_bit_set(&self, bit: usize) -> bool {
        self.array[bit / (size_of::<BitmapBlock>() * 8)] & (1 << (bit % (size_of::<BitmapBlock>() * 8))) > 0
    }

    pub fn find_first_zero(&self) -> Option<usize> {
        for (block_index,block) in self.array.iter().enumerate() {
            if *block != !0 {
                /* Current block contains at least one zero bit */
                let mut b = *block;
                let mut bit = block_index * size_of::<BitmapBlock>() * 8;
                while b % 2 == 1 {
                    b = b >> 1;
                    bit = bit + 1;
                }
                return if bit < self.size { Some(bit) } else { None };
            }
        }
        None
    }
}

pub fn bitmap_test() {
    let mut array: [u64; 1] = [0; 1];

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        for i in 0..16 {
            assert!(!bitmap.is_bit_set(i));
        }
    }
    assert_eq!(0, array[0]);

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        bitmap.set_bit(1);
        bitmap.set_bit(3);
        bitmap.set_bit(5);
        bitmap.set_bit(7);
        bitmap.set_bit(11);
        bitmap.set_bit(13);
    }
    assert_eq!(0b0010100010101010, array[0]);

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        bitmap.set_bit(1);
        bitmap.set_bit(3);
        bitmap.set_bit(5);
        bitmap.set_bit(7);
        bitmap.set_bit(11);
        bitmap.set_bit(13);
        bitmap.clear_bit(5);
        bitmap.clear_bit(11);
    }
    assert_eq!(0b0010000010001010, array[0]);

    {
        array[0] = 0b0011110111111111;
        let bitmap = Bitmap::new(&mut array, 16);
        assert_eq!(Some(9), bitmap.find_first_zero());
    }

    {
        array[0] = 0b1111111111111111;
        let bitmap = Bitmap::new(&mut array, 16);
        assert_eq!(None, bitmap.find_first_zero());
    }
}