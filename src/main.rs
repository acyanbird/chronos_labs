#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;
const COLOR: u8 = 0x04; // black background, red foreground


#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() {
    // vga::test_print();
    vga::test_clean();
    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}