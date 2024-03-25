#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use bootloader::BootInfo;
use x86_64::VirtAddr;
use chronos_labs::translate::translate_address;


#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> !{
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // try to translate some addresses
    // the VGA buffer page
    let virt_VGA = VirtAddr::new(0xb8000);
    let phys_VGA = unsafe { translate_address(virt_VGA, phys_mem_offset) };
    writeln!(WRITER.lock(),"Translate from virtual address from {:?} to physical address {:?}", virt_VGA, phys_VGA);

    // exsting page
    let virt_existing = VirtAddr::new(0x201008);
    let phys_existing = unsafe { translate_address(virt_existing, phys_mem_offset) };
    writeln!(WRITER.lock(),"Translate from virtual address from {:?} to physical address {:?}", virt_existing, phys_existing);

    // non-existing page
    let virt_non_existing = VirtAddr::new(0xffffffffc0000000);
    let phys_non_existing = unsafe { translate_address(virt_non_existing, phys_mem_offset) };
    writeln!(WRITER.lock(),"Translate from virtual address from {:?} to physical address {:?}", virt_non_existing, phys_non_existing);

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}