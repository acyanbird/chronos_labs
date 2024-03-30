#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

use lazy_static::lazy_static;
use spin::Mutex;
use chronos_labs::WRITER;
use core::fmt::Write;
