#![no_std]  // disable the Rust standard library
#![no_main] // disable all Rust-level entry points

use chronos_labs::WRITER;
use core::fmt::Write;
use x86_64::registers::control::Cr3;
use bootloader::BootInfo;
use x86_64::structures::paging::PageTable;
use x86_64::VirtAddr;

const DISPLAY_ENTRY: i32 = 1;


#[no_mangle]    // don't mangle the name of this function
pub extern "C" fn _start(boot_info: &'static BootInfo) -> !{
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);

    // counter
    let mut l4_counter = 0;
    let mut l3_counter = 0;
    let mut l2_counter = 0;
    let mut l1_counter = 0;

    let (l4_entry, _) = Cr3::read();

    let virt = phys_mem_offset + l4_entry.start_address().as_u64();
    let l4_ptr: *mut PageTable = virt.as_mut_ptr();
    let l4_table = unsafe { &*l4_ptr };
    for (i, entry) in l4_table.iter().enumerate() {
        if !entry.is_unused() && l4_counter < DISPLAY_ENTRY {
            writeln!(WRITER.lock(), "L4 Entry {}: {:?}", i, entry).unwrap();
            l4_counter += 1;

            let phys = entry.frame().unwrap().start_address();
            let virt = phys.as_u64() + boot_info.physical_memory_offset;
            let l3_ptr = VirtAddr::new(virt).as_mut_ptr();
            let l3_table: &PageTable = unsafe { &*l3_ptr };

            // print non-empty entries of the level 3 table
            for (i, entry) in l3_table.iter().enumerate() {
                if !entry.is_unused() && l3_counter < DISPLAY_ENTRY {
                    writeln!(WRITER.lock(), "   L3 Entry {}: {:?}", i, entry).unwrap();
                    l3_counter += 1;

                    let phys = entry.frame().unwrap().start_address();
                    let virt = phys.as_u64() + boot_info.physical_memory_offset;
                    let l2_ptr = VirtAddr::new(virt).as_mut_ptr();
                    let l2_table: &PageTable = unsafe { &*l2_ptr };

                    // print non-empty entries of the level 2 table
                    for (i, entry) in l2_table.iter().enumerate() {
                        if !entry.is_unused() && l2_counter < DISPLAY_ENTRY {
                            writeln!(WRITER.lock(), "    L2 Entry {}: {:?}", i, entry).unwrap();
                            l2_counter += 1;

                            let phys = entry.frame().unwrap().start_address();
                            let virt = phys.as_u64() + boot_info.physical_memory_offset;
                            let l1_ptr = VirtAddr::new(virt).as_mut_ptr();
                            let l1_table: &PageTable = unsafe { &*l1_ptr };
                            // print non-empty entries of the level 1 table
                            for (i, entry) in l1_table.iter().enumerate() {
                                if !entry.is_unused() && l1_counter < DISPLAY_ENTRY {
                                    writeln!(WRITER.lock(), "      L1 Entry {}: {:?}", i, entry).unwrap();
                                    l1_counter += 1;
                                }
                            }
                            l1_counter = 0;
                        }
                    }
                    l2_counter = 0;
                }
            }
            l3_counter = 0;
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