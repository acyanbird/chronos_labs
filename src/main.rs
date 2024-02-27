#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use volatile::Volatile;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const COLOR: u8 = 0x04; // black background, red foreground


#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u8, 4000) };

    vga_buffer[0] = b'H';
    vga_buffer[1] = COLOR;
    vga_buffer[2] = b'e';
    vga_buffer[3] = COLOR;
    vga_buffer[4] = b'l';
    vga_buffer[5] = COLOR;
    vga_buffer[6] = b'l';
    vga_buffer[7] = COLOR;
    vga_buffer[8] = b'o';
    vga_buffer[9] = COLOR;

    // write "World" at the next line
    vga_buffer[160] = b'W';
    vga_buffer[161] = COLOR;
    vga_buffer[162] = b'o';
    vga_buffer[163] = COLOR;
    vga_buffer[164] = b'r';
    vga_buffer[165] = COLOR;
    vga_buffer[166] = b'l';
    vga_buffer[167] = COLOR;
    vga_buffer[168] = b'd';
    vga_buffer[169] = COLOR;

    // End
    vga_buffer[3998] = b'!';
    vga_buffer[3999] = COLOR;

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}