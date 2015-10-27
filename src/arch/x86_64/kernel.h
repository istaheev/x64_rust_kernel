#ifndef __KERNEL_H__
#define __KERNEL_H__ 1

// The kernel is linked to this virtual address.
// linker.ld should be updated accordingly if changed.
#define KERNEL_VMA 0xFFFFFFFF80000000

// Size of the kernel stack
#define KERNEL_STACK_SIZE 0x2000

#define GDT_ENTRIES_COUNT 3

#define IDT_ENTRIES_COUNT 0x30

// Returns physical address of a pointer located in the higher half
#define PHYS_ADDR(x) (x - KERNEL_VMA)

#ifndef ASM_FILE

#include <stdint.h>

// Symbols defined by linker script
extern char __link_kernel_begin_vaddr[];
extern char __link_kernel_end_vaddr[];

// Parts of segment_descriptor.seg_type
#define SEG_TYPE_DATA               0
#define SEG_TYPE_CODE               (1<<3)

#define SEG_TYPE_CODE_NON_CONFORMING    0
#define SEG_TYPE_CODE_CONFORMING        (1<<2)

#define SEG_TYPE_CODE_EXECUTE_ONLY  0
#define SEG_TYPE_CODE_READ_EXECUTE  (1<<1)

#define SEG_TYPE_DATA_EXPAND_UP     0
#define SEG_TYPE_DATA_EXPAND_DOWN   (1<<2)

#define SEG_TYPE_DATA_READ_ONLY     0
#define SEG_TYPE_DATA_READ_WRITE    (1<<1)

#define SEG_TYPE_NON_ACCESSED       0
#define SEG_TYPE_ACCESSED           (1<<0)

// Values for segment_descriptor.desc_type
#define DESC_TYPE_SYSTEM    0x00
#define DESC_TYPE_CODE_DATA 0x01

// segment_descriptor.op_size
#define SEG_OP_SIZE_16      0x00
#define SEG_OP_SIZE_32      0x01

// segment_descriptor.code_size
#define SEG_CODE_32         0x00
#define SEG_CODE_64         0x01

// segment_descriptor.granularity
#define SEG_GRAN_1B         0x00 
#define SEG_GRAN_4KB        0x01

struct segment_descriptor
{
    unsigned int limit_low  : 16;
    unsigned int base_low   : 24;
    unsigned int seg_type    : 4;
    unsigned int desc_type   : 1;
    unsigned int dpl         : 2;
    unsigned int present     : 1;
    unsigned int limit_high  : 4;
    unsigned int available   : 1;
    unsigned int code_size   : 1;
    unsigned int op_size     : 1;
    unsigned int granularity : 1;
    unsigned int base_high   : 8;
} __attribute__((packed));

typedef struct segment_descriptor segment_descriptor_t;

struct segment_descriptor_entry
{
    union
    {
        segment_descriptor_t desc;
        uint64_t value;
    } u;
};

typedef struct segment_descriptor_entry segment_descriptor_entry_t;

#define IDT_INTR_GATE 0x0E
#define IDT_TRAP_GATE 0x0F

struct idt_descriptor
{
    unsigned int offset_0_15  : 16;
    unsigned int seg_selector : 16;
    unsigned int stack_table  : 3;
    unsigned int _zero1       : 1;
    unsigned int _zero2       : 1;
    unsigned int _zero3       : 3;
    unsigned int type         : 4;
    unsigned int _zero4       : 1;
    unsigned int dpl          : 2;
    unsigned int present      : 1;
    unsigned int offset_16_31 : 16;
    unsigned int offset_32_63 : 32;
    unsigned int reserved     : 32;
} __attribute__((packed));

typedef struct idt_descriptor idt_descriptor_t;

struct trap_regs
{
    uint64_t rax;
    uint64_t rbx;
    uint64_t rcx;
    uint64_t rdx;
    uint64_t rdi;
    uint64_t rsi;
    uint64_t r8;
    uint64_t r9;
    uint64_t r10;
    uint64_t r11;
    uint64_t r12;
    uint64_t r13;
    uint64_t r14;
    uint64_t r15;
    uint64_t rbp;
    uint64_t ds;
    uint64_t es;
    uint64_t fs;
    uint64_t gs;
    uint64_t trap_no;

    // Stack frame set by CPU
    uint64_t error_code;
    uint64_t rip;
    uint64_t cs;
    uint64_t rflags;
    uint64_t rsp;
    uint64_t ss;
};

typedef struct trap_regs trap_regs_t;

extern void trap_0 ( void );
extern void trap_1 ( void );
extern void trap_2 ( void );
extern void trap_3 ( void );
extern void trap_4 ( void );
extern void trap_5 ( void );
extern void trap_6 ( void );
extern void trap_7 ( void );
extern void trap_8 ( void );
extern void trap_9 ( void );
extern void trap_10 ( void );
extern void trap_11 ( void );
extern void trap_12 ( void );
extern void trap_13 ( void );
extern void trap_14 ( void );
extern void trap_15 ( void );
extern void trap_16 ( void );
extern void trap_17 ( void );
extern void trap_18 ( void );
extern void trap_19 ( void );
extern void trap_20 ( void );

extern void interrupt_entry ( void );

void kprint(const char *, ...);

#endif // ASM_FILE

// Offsets of fields in trap_regs
#define TRAP_REGS_RAX           0x0000
#define TRAP_REGS_RBX           0x0008
#define TRAP_REGS_RCX           0x0010
#define TRAP_REGS_RDX           0x0018
#define TRAP_REGS_RDI           0x0020
#define TRAP_REGS_RSI           0x0028
#define TRAP_REGS_R8            0x0030
#define TRAP_REGS_R9            0x0038
#define TRAP_REGS_R10           0x0040
#define TRAP_REGS_R11           0x0048
#define TRAP_REGS_R12           0x0050
#define TRAP_REGS_R13           0x0058
#define TRAP_REGS_R14           0x0060
#define TRAP_REGS_R15           0x0068
#define TRAP_REGS_RBP           0x0070
#define TRAP_REGS_DS            0x0078
#define TRAP_REGS_ES            0x0080
#define TRAP_REGS_FS            0x0088
#define TRAP_REGS_GS            0x0090
#define TRAP_REGS_TRAP_NO       0x0098
#define TRAP_REGS_ERROR_CODE    0x00a0
#define TRAP_REGS_RIP           0x00a8
#define TRAP_REGS_CS            0x00b0
#define TRAP_REGS_RFLAGS        0x00b8
#define TRAP_REGS_RSP           0x00c0
#define TRAP_REGS_SS            0x00c8

#endif // __KERNEL_H__
