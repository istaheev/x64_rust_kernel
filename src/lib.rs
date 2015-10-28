#![feature(no_std)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(core_str_ext)]
#![feature(core_slice_ext)]
#![feature(asm)]
#![no_std]

extern crate rlibc;
extern crate spin;

mod bochs;
#[macro_use]
mod vga;

extern {
    static __link_kernel_begin_vaddr: u8;
    static __link_kernel_end_vaddr: u8;
    static __link_load_end: u8;
    static __link_bss_end: u8;
}

#[no_mangle]
pub extern fn kernel_main() -> ! {
    bochs::magic_break();
    println!("Kernel placement: {:?} - {:?}", &__link_kernel_begin_vaddr as *const u8, &__link_kernel_end_vaddr as *const u8);
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
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
