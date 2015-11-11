OUTPUT_FORMAT("elf64-x86-64")
ENTRY(entry)

KERNEL_PHYSICAL_BASE = 0x100000;
/* Should be synchronized with config.rs */
KERNEL_VIRTUAL_BASE = 0xFFFFFFFF80000000;

SECTIONS
{
    . = KERNEL_PHYSICAL_BASE;

    .bootstrap :
    {
        */bootstrap.o(.text)
    }

    . += KERNEL_VIRTUAL_BASE;

    __link_kernel_begin_vaddr = .;

    .text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_VIRTUAL_BASE)
    {
        *(.text.*)
        *(EXCLUDE_FILE(*/bootstrap.o) .text)
        *(.rodata.*)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_VIRTUAL_BASE)
    {
        *(.data.*)
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
        *(.comment)
        *(.eh_frame)
    }
}