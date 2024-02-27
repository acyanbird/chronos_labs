#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const BACKGROUND_COLOR: u16 = 0x1000; // blue background, black foreground

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> !{

    let vga_buffer = unsafe { core::slice::from_raw_parts_mut(0xb8000 as *mut u16, 2000) };

    for i in 0..(BUFFER_HEIGHT * BUFFER_WIDTH) {
        vga_buffer[i] = BACKGROUND_COLOR;
    }

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}