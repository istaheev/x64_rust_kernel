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
mod vga;

use core::fmt::Write;

#[no_mangle]
pub extern fn kernel_main() -> ! {
    bochs::magic_break();
    vga::CONSOLE.lock().write_str("Hello, World!").unwrap();
    halt();
}

fn halt() -> ! {
    let syms = [b'|', b'/', b'-', b'\\'];
    let video = 0xb8000 as *mut u8;
    loop {
        for s in syms.iter() {
            unsafe { *video = *s; };
        }
    }
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
