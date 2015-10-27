OUTPUT_FORMAT("elf64-x86-64")
ENTRY(entry)

KERNEL_LMA = 0x100000;
KERNEL_VMA = 0xFFFFFFFF80000000;

SECTIONS
{
    . = KERNEL_LMA;

    .bootstrap :
    {
        */bootstrap.o(.text)
    }

    . += KERNEL_VMA;

    __link_kernel_begin_vaddr = .;

    .text ALIGN(0x1000) : AT(ADDR(.text) - KERNEL_VMA)
    {
        *(.text)
        *(EXCLUDE_FILE(*/bootstrap.o) .text)
        *(.rodata)
    }

    .data ALIGN(0x1000) : AT(ADDR(.data) - KERNEL_VMA)
    {
        *(.data)
    }

    __link_load_end = . - KERNEL_VMA;

    .bss ALIGN(0x1000) : AT(ADDR(.bss) - KERNEL_VMA)
    {
        *(.bss)
    }

    __link_bss_end = . - KERNEL_VMA;

    __link_kernel_end_vaddr = .;

    /DISCARD/ :
    {
        *(.comment)
        *(.eh_frame)
    }
}