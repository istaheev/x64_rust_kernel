/* Defines kernel layout in memory */

use memory::MemoryRegion;

/* Virtual memory where the kernel is loaded. Should be synchronized with linker.ld */
const KERNEL_VIRTUAL_BASE: usize = 0xFFFFFFFF80000000;

// Symbols from linker
extern {
    static __link_kernel_begin_vaddr: u8;
    static __link_kernel_end_vaddr: u8;
    static __link_load_end: u8;
    static __link_bss_end: u8;
}

fn kernel_begin_vaddr() -> usize {
    &__link_kernel_begin_vaddr as *const u8 as usize
}

fn kernel_end_vaddr() -> usize {
    &__link_kernel_end_vaddr as *const u8 as usize
}

fn kernel_begin_phys_addr() -> usize {
    kernel_begin_vaddr() - KERNEL_VIRTUAL_BASE
}

fn kernel_end_phys_addr() -> usize {
    kernel_end_vaddr() - KERNEL_VIRTUAL_BASE
}

fn kernel_size() -> usize {
    kernel_end_vaddr() - kernel_begin_vaddr()
}

/* Returns region containing the entire kernel in physical memory */
pub fn physical_kernel_placement() -> MemoryRegion {
    MemoryRegion {
        addr: kernel_begin_phys_addr(),
        size: kernel_size()
    }
}

/* Returns region containing the entire kernel in virtual memory */
pub fn virtual_kernel_placement() -> MemoryRegion {
    MemoryRegion {
        addr: kernel_begin_vaddr(),
        size: kernel_size()
    }
}

/* Returns physical address for the corresponding virtual one.
 * Should only be applied for kernel addresses, might be invalid
 * after remapping */
pub fn physical_addr(virtual_addr: usize) -> usize {
    virtual_addr - KERNEL_VIRTUAL_BASE
}

/* Returns region in physical memory corresponding to the specified virtual one.
 * Should only be applied to kernel-related memory which placement is known */
pub fn physical_region(virtual_region: MemoryRegion) -> MemoryRegion {
    MemoryRegion {
        addr: physical_addr(virtual_region.addr),
        size: virtual_region.size
    }
}
