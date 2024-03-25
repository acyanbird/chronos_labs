#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use lazy_static::lazy_static;
use spin::Mutex;
use chronos_labs::WRITER;
use core::fmt::Write;
use bootloader::BootInfo;
use x86_64::{PhysAddr, VirtAddr};
use chronos_labs::translate::translate_address;

lazy_static! {
    static ref PHYS_MEM_OFFSET: Mutex<VirtAddr> = Mutex::new(VirtAddr::new(0));
}

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    *PHYS_MEM_OFFSET.lock() = phys_mem_offset;
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
fn translate_vga() {
    let phys_mem_offset = *PHYS_MEM_OFFSET.lock();
    let virt_vga = VirtAddr::new(0xb8000);
    let phys_vga = unsafe { translate_address(virt_vga, phys_mem_offset) };
    match phys_vga {
        Some(phys_vga) => assert_eq!(phys_vga, PhysAddr::new(0xb8000), "\n Failed to translate VGA address"),
        None => panic!("Failed to translate VGA address: {:?}", virt_vga),
    }
}

#[test_case]
fn translate_exsiting() {
    let phys_mem_offset = *PHYS_MEM_OFFSET.lock();
    let virt_existing = VirtAddr::new(0x201008);
    let phys_existing = unsafe { translate_address(virt_existing, phys_mem_offset) };
    match phys_existing {
        Some(phys_exsting) => assert_eq!(phys_exsting, PhysAddr::new(0x401008), "\n Failed to translate exsting page"),
        None => panic!("Failed to translate VGA address: {:?}", virt_existing),
    }
}

#[test_case]
fn translate_non_exsiting() {
    let phys_mem_offset = *PHYS_MEM_OFFSET.lock();
    let virt_non_existing = VirtAddr::new(0xffffffffc0000000);
    let phys_non_existing = unsafe { translate_address(virt_non_existing, phys_mem_offset) };
    match phys_non_existing {
        Some(_) => panic!("Should be none"),
        None => assert!(true),
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    writeln!(WRITER.lock(),"Failed with error: {}", _info).unwrap();
    loop {}
}