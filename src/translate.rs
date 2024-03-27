use x86_64::structures::paging::PageTable;
use x86_64::PhysAddr;
use x86_64::VirtAddr;
use x86_64::registers::control::Cr3;


pub unsafe fn translate_address(addr: VirtAddr, physical_memory_offset: VirtAddr)
                        -> Option<PhysAddr>
{
    let (l4_frame, _) = Cr3::read();
    let mut frame_addr = l4_frame;


    // get offset of each table
    let table_offset = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];

    // traverse the page table to find the frame corresponding to the address
    for &index in &table_offset {
        // calculate the virtual address of the next table
        let virt = physical_memory_offset + frame_addr.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // get the frame from the table entry and update the frame address
        let entry = &table[index];
        frame_addr = match entry.frame() {
            Ok(frame) => frame,
            Err(_) => return None
        };
    }

    // calculate the physical address
    Some(frame_addr.start_address() + u64::from(addr.page_offset()))
}