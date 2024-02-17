#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points


use chronos_labs::WRITER;
use core::fmt::Write;

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() {
    for i in 0..5 {
        writeln!(WRITER.lock(), "Hello World {}", i).unwrap();
    }

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}