use x86_64::structures::idt::InterruptDescriptorTable;
use x86_64::structures::idt::InterruptStackFrame;
use crate::WRITER;
use core::fmt::Write;
use spin::Mutex;
use pic8259::ChainedPics;
use x86_64::instructions::port::Port;

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();
static NUMLOCK: Mutex<bool> = Mutex::new(false);
pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(0, 8) });



pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint);
        IDT.load();
        IDT[0].set_handler_fn(timer);
        IDT[1].set_handler_fn(keyboard);
    }
}

extern "x86-interrupt" fn breakpoint(_: InterruptStackFrame)
{
    writeln!(WRITER.lock(), "Break point works.\n").unwrap();}

extern "x86-interrupt" fn timer(_: InterruptStackFrame)
{
    write!(WRITER.lock(), "_").unwrap();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(0);
    }
}

extern "x86-interrupt" fn timer_off(_: InterruptStackFrame)
{
    write!(WRITER.lock(), "").unwrap();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(0);
    }
}

extern "x86-interrupt" fn keyboard(_: InterruptStackFrame)
{
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    let mut key: Option<&str> = None;
    let mut numlock = NUMLOCK.lock();
    if scancode == 0x45 {
        *numlock = !*numlock;
        key = Some("numlock");
    } else if !*numlock {
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
    } else {
        key = match scancode {
            0x02 => Some("1"),
            0x03 => Some("2"),
            0x04 => Some("3"),
            0x05 => Some("4"),
            0x06 | 0x4C => Some("5"),
            0x07 => Some("6"),
            0x08 => Some("7"),
            0x09 => Some("8"),
            0x0a => Some("9"),
            0x0b => Some("0"),
            0x4F => Some("end"),
            0x50 => Some("DownArrow"),
            0x51 => Some("PageDown"),
            0x4B => Some("LeftArrow"),
            0x4D => Some("RightArrow"),
            0x47 => Some("home"),
            0x48 => Some("UpArrow"),
            0x49 => Some("PageUp"),
            0x52 => Some("Insert"),
            0x53 => Some("Delete"),
            _ => None,
        };
    }


    if let Some(key) = key {
        writeln!(WRITER.lock(), "{}", key);
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(1);
    }
}