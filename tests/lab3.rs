#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use chronos_labs::{interrupts, WRITER};
use core::fmt::Write;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    interrupts::init_idt();
    test_main();
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test();
    }
    writeln!(WRITER.lock(),"Running {} test(s)", tests.len()).unwrap();
    writeln!(WRITER.lock(),"All success.").unwrap();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    writeln!(WRITER.lock(),"Failed with error: {}", _info).unwrap();
    loop {}
}