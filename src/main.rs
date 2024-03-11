#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel);

#[no_mangle]    // don't mangle the name of this function
fn kernel(boot_info: &'static BootInfo) -> !{
    let (level_4_page_table, _) = Cr3::read();
    writeln!(WRITER.lock(),
        "Level 4 page table at: {:?}",
        level_4_page_table.start_address()
    ).unwrap();

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}