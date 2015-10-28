// Simple VGA access

use spin::Mutex;

const VGA_MEM: usize = 0xb8000;

pub static CONSOLE: Mutex<Terminal> = Mutex::new(Terminal {
    column:   0,
    row:      0,
    fg_color: Color::White,
    bg_color: Color::Black,
    buffer:   TextBuffer {
        width:  80,
        height: 25,
        ptr:    VGA_MEM
    }
});

#[repr(u8)]
#[derive(Clone,Copy,Debug)]
#[allow(dead_code)]
pub enum Color {
    Black      = 0,
    Blue       = 1,
    Green      = 2,
    Cyan       = 3,
    Red        = 4,
    Magenta    = 5,
    Brown      = 6,
    LightGray  = 7,
    DarkGray   = 8,
    LightBlue  = 9,
    LightGreen = 10,
    LightCyan  = 11,
    LightRed   = 12,
    Pink       = 13,
    Yellow     = 14,
    White      = 15,
}

#[repr(C)]
#[derive(Clone,Copy,Debug)]
struct CharWithColor {
    ascii: u8,
    color: u8
}

impl CharWithColor {
    pub fn new(ascii: u8, fg_color: Color, bg_color: Color) -> CharWithColor {
        CharWithColor {
            ascii: ascii,
            color: ((bg_color as u8) << 4) | (fg_color as u8)
        }
    }
}

// Represents generic rectangular text buffer which can keep
// colored ascii characters. Being mapped to the VGA video memory
// region allows to write to the screen directly.
pub struct TextBuffer {
    width:  u8,
    height: u8,
    ptr:    usize
}

impl TextBuffer {
    pub fn set_char_and_color(&mut self, ascii: u8, column: u8, row: u8, fg_color: Color, bg_color: Color) {
        self.check_bounds(column, row);
        unsafe {
            *self.get_offset(column, row) = CharWithColor::new(ascii, fg_color, bg_color);
        }
    }

    pub fn scroll_up(&mut self) {
        for row in 0..self.height-1 {
            for column in 0..self.width {
                unsafe {
                    let row1 = self.get_offset(column, row);
                    let row2 = self.get_offset(column, row+1) as *const _;
                    *row1 = *row2
                }
            }
        }
        // clear last row
        for column in 0..self.width {
            unsafe {
                let row = self.get_offset(column, self.height - 1);
                *row = CharWithColor::new(' ' as u8, Color::Black, Color::Black);
            }
        }
    }

    fn check_bounds(&self, column: u8, row: u8) {
        if column >= self.width || row >= self.height {
            panic!("Write position is out of bounds!");
        }
    }

    fn get_offset(&self, column: u8, row: u8) -> *mut CharWithColor {
        (self.ptr + (self.width as usize) * (row as usize) * 2 + (column as usize) * 2) as *mut _
    }
}

// Converts a text buffer into a terminal allowing teletyping on it
pub struct Terminal {
    column: u8,
    row: u8,
    fg_color: Color,
    bg_color: Color,
    buffer: TextBuffer
}

impl Terminal {
    pub fn buffer(&mut self) -> &mut TextBuffer {
        &mut self.buffer
    }

    pub fn write_ascii(&mut self, ascii: u8) {
        if ascii == b'\n' {
            self.cr_lf();
        } else {
            let column = self.column;
            let row = self.row;
            let fg_color = self.fg_color;
            let bg_color = self.bg_color;
            self.buffer.set_char_and_color(ascii, column, row, fg_color, bg_color);
            self.advance_cursor();
        }
    }

    pub fn cr_lf(&mut self) {
        self.column = 0;
        if self.row >= self.buffer.height-1 {
            self.buffer.scroll_up();
        } else {
            self.row = self.row + 1;
        }
    }

    fn advance_cursor(&mut self) {
        self.column = self.column + 1;
        if self.column >= self.buffer.width - 1 {
            self.cr_lf();
        }
    }
}

impl ::core::fmt::Write for Terminal {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        for byte in s.bytes() {
          self.write_ascii(byte)
        }
        Ok(())
    }

    fn write_char(&mut self, c: char) -> ::core::fmt::Result {
        self.write_ascii(c as u8);
        Ok(())
    }
}

macro_rules! println {
    ($fmt:expr) => (print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}

macro_rules! print {
    ($($arg:tt)*) => ({
            use ::core::fmt::Write;
            $crate::vga::CONSOLE.lock().write_fmt(format_args!($($arg)*)).unwrap();
    });
}