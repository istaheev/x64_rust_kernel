/*
 * Bitmap implementation (only for little-endian architectures)
 */

use core::mem::size_of;
use core::slice::from_raw_parts_mut;

pub type BitmapBlock = u64;

pub struct Bitmap<'a> {
    array: &'a mut [BitmapBlock],
    /* Size of the bitmap in bits */
    size: usize
}

impl<'a> Bitmap<'a> {

    pub fn new(array: &'a mut [BitmapBlock], size: usize) -> Bitmap<'a> {
        assert!(size <= array.len() * size_of::<BitmapBlock>() * 8,
                "Array for bitmap has insufficient space to store specified amount of bits.");

        Bitmap {
            array: array,
            size: size
        }
    }

    pub fn from_raw_addr(addr: usize, size: usize) -> Bitmap<'static> {
        debug_assert!(size > 0, "Creating bitmap of zero size");

        let mut array_size = size / (size_of::<BitmapBlock>() * 8);
        if array_size * size_of::<BitmapBlock>() * 8 < size {
            array_size = array_size + 1
        }

        let array = unsafe {
            from_raw_parts_mut(addr as *mut BitmapBlock , array_size)
        };

        Bitmap {
            array: array,
            size: size
        }
    }

    /* Clear the entire bitmap (set all bits to zero) */
    pub fn clear(&mut self) {
        /* Clear entire blocks first */
        let full_blocks_count = self.size / (size_of::<BitmapBlock>() * 8);
        for block in 0..full_blocks_count {
            self.array[block] = 0;
        }

        /* For last bits which form an incompleted block clear them individually */
        for bit in (full_blocks_count * size_of::<BitmapBlock>() * 8) .. self.size {
            self.clear_bit(bit);
        }
    }

    pub fn set_bit(&mut self, bit: usize) {
        debug_assert!(bit < self.size, "Out of bitmap range");
        self.array[bit / (size_of::<BitmapBlock>() * 8)] |= 1 << (bit % (size_of::<BitmapBlock>() * 8));
    }

    pub fn clear_bit(&mut self, bit: usize) {
        debug_assert!(bit < self.size, "Out of bitmap range");
        self.array[bit / (size_of::<BitmapBlock>() * 8)] &= !(1 << (bit % (size_of::<BitmapBlock>() * 8)));
    }

    pub fn is_bit_set(&self, bit: usize) -> bool {
        self.array[bit / (size_of::<BitmapBlock>() * 8)] & (1 << (bit % (size_of::<BitmapBlock>() * 8))) > 0
    }

    pub fn find_first_zero(&self) -> Option<usize> {
        for (block_index,block) in self.array.iter().enumerate() {
            if *block != !0 {
                /* Current block contains at least one zero bit */
                let mut b = *block;
                let mut bit = block_index * size_of::<BitmapBlock>() * 8;
                while b % 2 == 1 {
                    b = b >> 1;
                    bit = bit + 1;
                }
                return if bit < self.size { Some(bit) } else { None };
            }
        }
        None
    }
}

pub fn bitmap_test() {
    let mut array: [u64; 1] = [0; 1];

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        for i in 0..16 {
            assert!(!bitmap.is_bit_set(i));
        }
    }
    assert_eq!(0, array[0]);

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        bitmap.set_bit(1);
        bitmap.set_bit(3);
        bitmap.set_bit(5);
        bitmap.set_bit(7);
        bitmap.set_bit(11);
        bitmap.set_bit(13);
    }
    assert_eq!(0b0010100010101010, array[0]);

    {
        let mut bitmap = Bitmap::new(&mut array, 16);
        bitmap.clear();
        bitmap.set_bit(1);
        bitmap.set_bit(3);
        bitmap.set_bit(5);
        bitmap.set_bit(7);
        bitmap.set_bit(11);
        bitmap.set_bit(13);
        bitmap.clear_bit(5);
        bitmap.clear_bit(11);
    }
    assert_eq!(0b0010000010001010, array[0]);

    {
        array[0] = 0b0011110111111111;
        let bitmap = Bitmap::new(&mut array, 16);
        assert_eq!(Some(9), bitmap.find_first_zero());
    }

    {
        array[0] = 0b1111111111111111;
        let bitmap = Bitmap::new(&mut array, 16);
        assert_eq!(None, bitmap.find_first_zero());
    }
}