#![feature(abi_x86_interrupt)]
#![no_std]
mod vga;
pub mod translate;
pub mod interrupts;

pub use vga::WRITER;