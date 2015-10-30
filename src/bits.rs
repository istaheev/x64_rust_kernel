// A try to implement bitwise operations on numbers using ranges

use core::ops::{Sub, BitOr, BitAnd, Not, Shl, Shr, Range};
use core::num::One;

pub fn get_single<T>(number: T, pos: usize) -> T where T: One + Sub<Output=T> + BitAnd<Output=T> + Shl<usize,Output=T>  + Shr<usize,Output=T> {
    get_range(number, pos..pos+1)
}

pub fn get_range<T>(number: T, range: Range<usize>) -> T where T: One + Sub<Output=T> + BitAnd<Output=T> + Shl<usize,Output=T> + Shr<usize,Output=T> {
    let bits_count = range.end - range.start;
    if bits_count < 1 {
        panic!("End should be greater than start");
    }
    let mask = (T::one() << bits_count) - T::one();
    (number >> range.start) & mask
}

pub fn set_single<T>(number: T, pos: usize, value: T) -> T  where T: Copy + One + Sub<Output=T> + BitAnd<Output=T> + BitOr<Output=T> + Not<Output=T> + Shl<usize,Output=T> {
    set_range(number, pos..pos+1, value)
}

pub fn set_range<T>(number: T, range: Range<usize>, value: T) -> T  where T: Copy + One + Sub<Output=T> + BitAnd<Output=T> + BitOr<Output=T> + Not<Output=T> + Shl<usize,Output=T> {
    let bits_count = range.end - range.start;
    if bits_count < 1 {
        panic!("End should be greater than start");
    }
    let mask = (T::one() << bits_count) - T::one();
    (number & !(mask << range.start)) | ((value & mask) << range.start)
}


pub fn tests() {
    let num = 0b10101010usize;

    // get bits
    assert_eq!(0b010, get_range(num, 0..3));
    assert_eq!(0b101, get_range(num, 1..4));
    assert_eq!(0b1010101, get_range(num, 1..64));
    assert_eq!(1, get_single(num, 5));
    assert_eq!(0, get_single(num, 6));

    // set bits
    assert_eq!(0b11001100, set_range(num, 0..16, 0b11001100));
    assert_eq!(0b10101011, set_range(num, 0..3, 0b011));
    assert_eq!(0b10100110, set_range(num, 1..4, 0b011));
    assert_eq!(0b10100110, set_range(num, 2..6, 0b1001));
    assert_eq!(0b10111010, set_single(num, 4, 1));
}