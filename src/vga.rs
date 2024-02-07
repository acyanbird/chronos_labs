use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const COLOR: u8 = 0x04; // black background, red foreground

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
    ascii: u8,
    color: u8,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT], // 二维数组
}

pub struct Writer {
    column_position: usize,
    row_position: usize,
    buffer: &'static mut Buffer,
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
pub fn test_print() {
    let mut writer = Writer {
        column_position: 0,
        row_position: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    writer.write_byte(b'H', COLOR);
    writer.write_byte(b'\n', COLOR);
    writer.write_byte(b'e', COLOR);
}

pub fn test_clean() {
    let mut writer = Writer {
        column_position: 0,
        row_position: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    };

    for i in 1..= 26 {
        let line: u8 = i + b'0';
        writer.write_byte(line, COLOR);
        writer.write_byte(b'\n', COLOR);
    }
}