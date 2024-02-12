#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use crate::vga::WRITER;

mod vga; // import the `vga` module

const COLOR: u8 = 0x04; // black background, red foreground

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() {
    use core::fmt::Write;
    writeln!(WRITER.lock(), "Hello, World!").unwrap();
    write!(WRITER.lock(), "汉字").unwrap();
    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}