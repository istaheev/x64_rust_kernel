#![feature(no_std)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(core_str_ext)]
#![feature(core_slice_ext)]
#![feature(asm)]
#![feature(zero_one)]
#![no_std]

extern crate rlibc;
extern crate spin;

mod bits;
mod bochs;
mod config;
mod memory;
mod multiboot;
#[macro_use]
mod vga;
mod cpuid;
mod paging;
mod phys_mem_allocator;

use multiboot::PhysicalMemoryMap;
use phys_mem_allocator::PhysMemAllocator;

#[no_mangle]
pub extern fn kernel_main(multiboot_info_ptr: *const multiboot::Info) -> ! {
    //bochs::magic_break();

    println!("");
    println!("Kernel placement: 0x{:016x} - 0x{:016x} ({} bytes).", config::kernel_begin_vaddr(), config::kernel_end_vaddr(), config::kernel_end_vaddr() - config::kernel_begin_vaddr());

    print!("Running tests..");
    bits::tests();
    phys_mem_allocator::bitmap_test();
    println!(" successfully.");

    display_cpu_info();
    display_multiboot_info(multiboot_info_ptr);

    let multiboot_info = unsafe { &*multiboot_info_ptr };

    /* Warning: kernel stack and page tables set by bootstrap marked as free
     * and might be allocated by the allocator.
     * They should be reinitialized by the kernel asap.
     */
    let mut phys_mem_allocator = PhysMemAllocator::init(multiboot_info);
    println!("Physical memory: {} total pages, {} free pages ({} pages occupied).",
             phys_mem_allocator.total_pages_count(),
             phys_mem_allocator.free_pages_count(),
             phys_mem_allocator.total_pages_count() - phys_mem_allocator.free_pages_count());

    let lower_mem_pages = multiboot_info.get_lower_memory() / 4096;
    let p1 = alloc_pages(&mut phys_mem_allocator, lower_mem_pages as u32);
    let p2 = phys_mem_allocator.alloc_page().unwrap();
    let p3 = phys_mem_allocator.alloc_page().unwrap();
    let p4 = phys_mem_allocator.alloc_page().unwrap();
    let p5 = phys_mem_allocator.alloc_page().unwrap();
    println!("pages: 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}, 0x{:x}.", p1, p2, p3, p4, p5);

    halt();
}

fn alloc_pages(allocator: &mut PhysMemAllocator, n: u32) -> usize {
    let mut p = allocator.alloc_page().unwrap();
    for i in 1..n {
        p = allocator.alloc_page().unwrap();
    }
    p
}

fn display_cpu_info() {
    let vendor_id = cpuid::get_vendor_id();
    println!("CPU vendor: {}.", unsafe { ::core::str::from_utf8_unchecked(&vendor_id.vendor) });
    println!("CPUID: max basic function 0x{:x}, max extended function 0x{:x}.", vendor_id.max_basic_func, vendor_id.max_extended_func);

    if vendor_id.is_cpu_info_available() {
        let cpu_info = cpuid::get_cpu_info();
        println!("CPU: stepping {}, model {}, family {}, type {}.", cpu_info.stepping, cpu_info.model, cpu_info.family, cpu_info.cpu_type);
        print!("CPU flags: ");
        cpuid::print_cpu_features(cpu_info.features1, cpuid::CPU_FEATURES1_MAP);
        cpuid::print_cpu_features(cpu_info.features2, cpuid::CPU_FEATURES2_MAP);
        println!("");
    }
}

fn display_multiboot_info(multiboot_info_ptr: *const multiboot::Info) {
    let multiboot_info = unsafe { &*multiboot_info_ptr };

    if multiboot_info.is_memory_size_available() {
        println!("Lower memory: {}; Upper: {}; Total: {}.", multiboot_info.get_lower_memory(), multiboot_info.get_upper_memory(), multiboot_info.get_lower_memory() + multiboot_info.get_upper_memory());
    } else {
        println!("No memory size available from multiboot.");
    }

    if multiboot_info.is_memory_map_available() {
        println!("Memory map:");
        for region in multiboot_info.memory_regions() {
            println!("  0x{:016x} - 0x{:016x} ({} bytes): {:?}", region.address, region.address + region.length - 1, region.length, region.region_type);
        }
        println!("Memory available in total: {} bytes.", multiboot_info.total_memory_available());
    } else {
        println!("No memory map available from multiboot.");
    }
}

fn halt() -> ! {
    let syms = b"|\\-//||\\-//";
    let mut pos = 0;
    loop {
        for (i,s) in syms.iter().enumerate() {
            let column = (pos + i) % syms.len();
            vga::CONSOLE.lock().buffer().set_char_and_color(*s, column as u8, 0, vga::Color::Yellow, vga::Color::Black);
        }
        pos = pos + 1;
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(args: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    use core::fmt::Write;
    println!("Panic in {}:{}", file, line);
    vga::CONSOLE.lock().write_fmt(args).unwrap();
    halt()
}
