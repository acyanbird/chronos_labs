use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use crate::WRITER;
use core::fmt::Write;
use spin::Mutex;
use pic8259::ChainedPics;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.load();
        IDT[0].set_handler_fn(timer);
    }
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0, 8) });


extern "x86-interrupt" fn breakpoint(_: InterruptStackFrame)

{
    writeln!(WRITER.lock(), "Break point works.\n").unwrap();
}

extern "x86-interrupt" fn timer(_: InterruptStackFrame)
{
    write!(WRITER.lock(), "_").unwrap();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(0);
    }
}

