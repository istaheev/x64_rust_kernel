#![feature(no_std, lang_items)]
#![no_std]

#[no_mangle]
pub extern fn kernel_main() {
    let mut vp = 0xb8000 as *mut u8;
    unsafe {
        *vp = 'A' as u8;
    }
    loop {}
}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
