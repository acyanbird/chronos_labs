#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use x86_64::registers::control::Cr3;
use bootloader::{BootInfo, entry_point};
use x86_64::structures::paging::Translate;
use x86_64::VirtAddr;
use chronos_labs::memory;


entry_point!(kernel);

#[no_mangle]    // don't mangle the name of this function
fn kernel(boot_info: &'static BootInfo) -> !{
    let (level_4_page_table, _) = Cr3::read();
    writeln!(WRITER.lock(),
             "Level 4 page table at: {:?}",
             level_4_page_table.start_address()
    ).unwrap();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // new: initialize a mapper
    let mapper = unsafe { memory::init(phys_mem_offset) };

    let addresses = [
        // the identity-mapped vga buffer page
        0xb8000,
        // some code page
        0x201008,
        // some stack page
        0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        boot_info.physical_memory_offset,
    ];

    for &address in &addresses {
        let virt = VirtAddr::new(address);
        let phys = mapper.translate_addr(virt);
        writeln!(WRITER.lock(),"{:?} -> {:?}", virt, phys).unwrap();
    }

    loop {}
}

#[panic_handler]    // this function is called on panic
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // the `!` type means "this function never returns"
    // place holder for now, we'll write this function later
    loop {}
}