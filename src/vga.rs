use volatile::Volatile;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const COLOR: u8 = 0x04; // black background, red foreground
const ERROR_COLOR: u8 = 0x10; // blue background, black foreground

#[repr(C)]
#[derive(Clone, Copy)]  // derive the `Clone` and `Copy` traits
struct VGAChar {
    ascii: u8,
    color: u8,
}

#[repr(transparent)]
pub(crate) struct Buffer {
    chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // 2D array
}

pub struct Writer {
    pub(crate) column_position: usize,
    pub(crate) row_position: usize,
    pub(crate) buffer: &'static mut Buffer,
}


impl Writer {
    pub fn write_byte(&mut self, byte: u8, color: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = self.row_position;
                let col = self.column_position;

                self.buffer.chars[row][col].write(VGAChar {
                    ascii: byte,
                    color,
                });
                self.column_position += 1;
            }
        }
    }
}

impl Writer {
    pub fn new_line(&mut self) {
        if self.row_position < BUFFER_HEIGHT - 1 {
            self.column_position = 0;
            self.row_position += 1; // change to new line
        } else {    // if the row is full, scroll up
            for row in 1..BUFFER_HEIGHT {
                for col in 0..BUFFER_WIDTH {
                    let character = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(character);
                }
            }
            self.clear_row(BUFFER_HEIGHT - 1);
            self.column_position = 0;
        }
    }

    fn clear_row(&mut self, row: usize) { // new function to clear a row
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(VGAChar {
                ascii: b' ',
                color: COLOR,
            });
        }
    }
}

impl Writer {
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // if not acceptable ASCII, print a space with error color
                0x20..=0x7e | b'\n' => self.write_byte(byte, COLOR),
                _ => self.write_byte(b' ', ERROR_COLOR),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}