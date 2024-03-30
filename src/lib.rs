#![feature(abi_x86_interrupt)]
#![no_std]
mod vga;
pub use vga::WRITER;

pub mod interrupts;