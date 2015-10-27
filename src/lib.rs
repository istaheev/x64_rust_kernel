#![feature(no_std, lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn kernel_main() {
    let vga = 0xb8000 as *mut u8;
    let message = b"Hello, world!";
    let mut offset = 0;
    for b in message {
        unsafe {
            *vga.offset(offset) = *b;
            offset = offset + 2;
        }
    }
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
