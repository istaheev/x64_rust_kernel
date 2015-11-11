OUTPUT_FORMAT("elf64-x86-64")
ENTRY(entry)

/* Starting position of kernel placement in the physical memory */
KERNEL_PHYSICAL_BASE = 0x100000;

/* Starting position of kernel in the virtual memory.
 * Should be synchronized with config.rs */
KERNEL_VIRTUAL_BASE = 0xFFFFFFFF80000000;

SECTIONS
{
    . = KERNEL_PHYSICAL_BASE;

    .multiboot : AT(ADDR(.multiboot))
    {
        KEEP( *(.multiboot) )
    }

    .bootstrap : AT(ADDR(.bootstrap))
    {
        *(.bootstrap)
    }

    . += KERNEL_VIRTUAL_BASE;

    __link_kernel_begin_vaddr = .;

    .text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_VIRTUAL_BASE)
    {
        *(.text .text.*)
        *(.rodata .rodata.*)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_VIRTUAL_BASE)
    {
        *(.data .data.*)
    }

    __link_load_end = . - KERNEL_VIRTUAL_BASE;

    .bss ALIGN(0x1000) : AT(ADDR(.bss) - KERNEL_VIRTUAL_BASE)
    {
        *(.bss)
    }

    __link_bss_end = . - KERNEL_VIRTUAL_BASE;

    __link_kernel_end_vaddr = .;

    /DISCARD/ :
    {
        *(.gcc_except_table)
        *(.comment)
        *(.eh_frame)
    }
}