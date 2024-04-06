use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use crate::WRITER;
use core::fmt::Write;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.load();
    }
}

extern "x86-interrupt" fn breakpoint(
    stack_frame: InterruptStackFrame)
{
    writeln!(WRITER.lock(), "Break point works.\n").unwrap();
}