//! Integration [Tests]
//!
//! [tests]: https://os.phil-opp.com/testing/#integration-tests
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate rustos;
use core::panic::PanicInfo;
use rustos::{println, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    serial_print!("tests::basic_boot::test_println... ");
    println!("test_println output");
    serial_println!("[ok]");
}
