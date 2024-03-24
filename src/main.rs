#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use bootloader::BootInfo;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
    let (l4_entry, _) = Cr3::read();
    writeln!(WRITER.lock(),
             "Level 4 page table at: {:?}",
             l4_entry.start_address()
    ).unwrap();


    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let phys = l4_entry.start_address();
    let virt = phys_mem_offset + phys.as_u64();
    let l4_ptr: *mut PageTable = virt.as_mut_ptr();
    let l4_table = unsafe { &*l4_ptr };
    writeln!(WRITER.lock(), "L4 Table at address: {:?}", l4_ptr).unwrap();
    for (i, entry) in l4_table.iter().enumerate() {
        // try to print all entries?
        // writeln!(WRITER.lock(), "L4 Entry {}: {:?}", i, entry).unwrap();

        if !entry.is_unused() {
            writeln!(WRITER.lock(), "L4 Entry {}: {:?}", i, entry).unwrap();
        }
    }

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}