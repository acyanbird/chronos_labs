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
    for test in tests {
        test();
    }
    writeln!(WRITER.lock(),"Running {} test(s)", tests.len()).unwrap();
    writeln!(WRITER.lock(),"All success.").unwrap();
}

#[test_case]
fn simple_printline() {
    writeln!(WRITER.lock(),"simple_printline").unwrap();
    WRITER.lock().clear_screen();
}

#[test_case]
fn print_a_lot() {
    for _ in 0..200 {
        writeln!(WRITER.lock(),"print_a_lot").unwrap();
    }
    WRITER.lock().clear_screen();
}

#[test_case]
fn print_newline() {
    writeln!(WRITER.lock(),"print_newline").unwrap();
    write!(WRITER.lock(),"a").unwrap();
    let char = WRITER.lock().get_ascii(1,0);
    assert_eq!(char, b'a', "\nFailed to write newline");
    WRITER.lock().clear_screen();
}

#[test_case]
fn exception() {
    writeln!(WRITER.lock(),"中文").unwrap();
    for i in 0..6 {
        let char = WRITER.lock().get_ascii(0,i);
        assert_eq!(char, b' ');
    }
}

// if you want to see what happen if a test fail, uncomment the following code
// #[test_case]
// fn make_error() {
//     assert_eq!(0,1);
//     writeln!(WRITER.lock(),"should not go here").unwrap();
// }

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    writeln!(WRITER.lock(),"Failed with error: {}", _info).unwrap();
    loop {}
}