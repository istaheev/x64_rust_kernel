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
#[macro_use]
mod vga;
mod cpuid;

// Symbols from linker
extern {
    static __link_kernel_begin_vaddr: u8;
    static __link_kernel_end_vaddr: u8;
    static __link_load_end: u8;
    static __link_bss_end: u8;
}

#[no_mangle]
pub extern fn kernel_main() -> ! {
    //bochs::magic_break();

    println!("");
    println!("Kernel placement: {:?} - {:?}", &__link_kernel_begin_vaddr as *const u8, &__link_kernel_end_vaddr as *const u8);

    println!("Running tests..");
    bits::tests();
    println!("Tests passed successfully.");

    let vendor_id = cpuid::get_vendor_id();
    println!("CPU vendor: {}.", unsafe { ::core::str::from_utf8_unchecked(&vendor_id.vendor) });
    println!("CPUID: max basic function 0x{:x}, max extended function 0x{:x}.", vendor_id.max_basic_func, vendor_id.max_extended_func);

    if vendor_id.is_cpu_info_available() {
        let cpu_info = cpuid::get_cpu_info();
        println!("CPU: stepping {}, model {}, family {}, type {}.", cpu_info.stepping, cpu_info.model, cpu_info.family, cpu_info.cpu_type);
        print!("CPU flags: ");
        cpuid::print_cpu_features(cpu_info.features1, cpuid::CPU_FEATURES1_MAP);
        print!("CPU flags: ");
        cpuid::print_cpu_features(cpu_info.features2, cpuid::CPU_FEATURES2_MAP);
    }

    halt();
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
