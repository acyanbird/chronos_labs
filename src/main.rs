#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

mod vga; // import the `vga` module

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start() {
    let mut writer = vga::Writer {
        column_position: 0,
        row_position: 0,
        buffer: unsafe { &mut *(0xb8000 as *mut vga::Buffer) },
    };

    use core::fmt::Write;
    writeln!(writer, "Hello, World! {}", 1).unwrap();
    write!(writer, "汉字").unwrap();

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}