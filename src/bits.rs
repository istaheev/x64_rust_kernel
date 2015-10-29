// A try to implement bitwise operations on numbers using ranges

use core::ops::{BitOr, BitAnd, Shl, Shr, Range};

#[derive(Clone, Copy, Debug)]
pub struct Bits<T>(pub T) where T: Copy + BitAnd + Shr<usize>;


impl<T> Bits<T> where T: Copy + BitAnd<Output=T> + Shr<usize,Output=T>  {
    pub fn value(&self) -> T {
        let &Bits(ref value) = self;
        *value
    }

    pub fn get(&self, range: Range<usize>) -> T {
        let bits = range.end - range.start;
        if bits < 1 {
            panic!("End should be greater than start");
        }
        let t = self.value() >> range.start;
        Self::mask(t, bits)
    }

    fn mask(v: T, bits: usize) -> T {
        v & (1 << bits)
    }
}

