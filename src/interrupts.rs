use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use crate::WRITER;
use core::fmt::Write;
use spin::Mutex;
use pic8259::ChainedPics;
use x86_64::instructions::port::Port;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.load();
        IDT[0].set_handler_fn(timer);
        IDT[1].set_handler_fn(keyboard);
    }
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0, 8) });


extern "x86-interrupt" fn breakpoint(stack_frame: InterruptStackFrame)
{
    writeln!(WRITER.lock(), "Break point:\n{:#?}", stack_frame).unwrap();
}

extern "x86-interrupt" fn timer(_stack_frame: InterruptStackFrame)
{
    write!(WRITER.lock(), "_").unwrap();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(0);
    }
}

extern "x86-interrupt" fn keyboard(_stack_frame: InterruptStackFrame)
{
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut key: Option<&str> = None;
    key = match scancode {
        0x02 | 0x4F => Some("1"),
        0x03 | 0x50 => Some("2"),
        0x04 | 0x51 => Some("3"),
        0x05 | 0x4B => Some("4"),
        0x06 | 0x4C => Some("5"),
        0x07 | 0x4D => Some("6"),
        0x08 | 0x47 => Some("7"),
        0x09 | 0x48 => Some("8"),
        0x0a | 0x49 => Some("9"),
        0x0b | 0x52 => Some("0"),
        0x53 => Some("."),
        _ => None,
    };


    if let Some(key) = key {
        write!(WRITER.lock(), "{}", key);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(1);
    }
}