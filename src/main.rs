#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use chronos_labs::interrupts;

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> !{
    interrupts::init_idt();
    x86_64::instructions::interrupts::int3(); // invoke a breakpoint exception

    // WRITER.lock().clear_screen();   // uncomment this line to clear the screen

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}