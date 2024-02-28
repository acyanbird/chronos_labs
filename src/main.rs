#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() -> !{
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    writeln!(WRITER.lock(),
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    ).unwrap();

    // WRITER.lock().clear_screen();   // uncomment this line to clear the screen

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}