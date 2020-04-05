//! [Double fault]
//!
//! [double fault]: https://os.phil-opp.com/double-fault-exceptions/
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate rustos;
use core::panic::PanicInfo;
use rustos::{serial_print, serial_println};

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
fn page_fault() {
    serial_print!("tests::page_fault::page_fault... ");
    // setup the interrupt descriptor table to catch the page fault.
    rustos::init();
    /* This code will be enabled with the correct page address
       once we fixes the page fault handler.
    unsafe {
        *(0xdead_beef as *mut u64) = 42;
    }
    */
    serial_println!("[ok]");
}
