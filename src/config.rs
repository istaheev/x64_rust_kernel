/* Virtual memory where the kernel is loaded. Should be synchronized with linker.ld */
pub const KERNEL_VIRTUAL_BASE: usize = 0xFFFFFFFF80000000;

// Symbols from linker
extern {
    static __link_kernel_begin_vaddr: u8;
    static __link_kernel_end_vaddr: u8;
    static __link_load_end: u8;
    static __link_bss_end: u8;
}

pub fn kernel_begin_vaddr() -> usize {
    &__link_kernel_begin_vaddr as *const u8 as usize
}

pub fn kernel_end_vaddr() -> usize {
    &__link_kernel_end_vaddr as *const u8 as usize
}

pub fn kernel_begin_phys_addr() -> usize {
    kernel_begin_vaddr() - KERNEL_VIRTUAL_BASE
}

pub fn kernel_end_phys_addr() -> usize {
    kernel_end_vaddr() - KERNEL_VIRTUAL_BASE
}

