use x86_64::structures::paging::PageTable;
use x86_64::PhysAddr;
use x86_64::VirtAddr;

pub unsafe fn translate_address(addr: VirtAddr, physical_memory_offset: VirtAddr)
                        -> Option<PhysAddr>
{
    use x86_64::structures::paging::page_table::FrameError;
    use x86_64::registers::control::Cr3;

    let (level_4_table_frame, _) = Cr3::read();

    let table_indexes = [
        addr.p4_index(), addr.p3_index(), addr.p2_index(), addr.p1_index()
    ];
    let mut frame = level_4_table_frame;

    // 遍历多级页表
    for &index in &table_indexes {
        // 将该框架转换为页表参考
        let virt = physical_memory_offset + frame.start_address().as_u64();
        let table_ptr: *const PageTable = virt.as_ptr();
        let table = unsafe {&*table_ptr};

        // 读取页表条目并更新`frame`。
        let entry = &table[index];
        frame = match entry.frame() {
            Ok(frame) => frame,
            Err(FrameError::FrameNotPresent) => return None,
            Err(FrameError::HugeFrame) => panic!("huge pages not supported"),
        };
    }

    // 通过添加页面偏移量来计算物理地址
    Some(frame.start_address() + u64::from(addr.page_offset()))
}