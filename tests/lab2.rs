#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use chronos_labs::WRITER;
use core::fmt::Write;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {

    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    writeln!(WRITER.lock(),"Running {} tests", tests.len()).unwrap();
    for test in tests {
        test();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}