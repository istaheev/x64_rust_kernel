#[allow(dead_code)]
#[inline(always)]
pub fn magic_break() {
    unsafe {
        asm!("xchgw %bx, %bx");
    }
}