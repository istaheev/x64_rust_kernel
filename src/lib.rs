#![feature(no_std)]
#![feature(lang_items)]
#![feature(const_fn)]
#![feature(core_str_ext)]
#![feature(core_slice_ext)]
#![no_std]

extern crate rlibc;
extern crate spin;

mod vga;

use core::fmt::Write;

#[no_mangle]
pub extern fn kernel_main() -> ! {
    vga::CONSOLE.lock().write_str("Hello, World!").unwrap();

    //let ab = core::sync::atomic::AtomicBool::new(true);
    //ab.compare_and_swap(true, false, core::sync::atomic::Ordering::SeqCst);

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
