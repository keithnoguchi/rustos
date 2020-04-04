//! [Double fault]
//!
//! [double fault]: https://os.phil-opp.com/double-fault-exceptions/
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate rustos;
use core::panic::PanicInfo;
use rustos::{exit_qemu, serial_print, serial_println, QemuExitCode};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
        serial_println!("[test did not panic]");
        exit_qemu(QemuExitCode::Failed);
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn page_fault() {
    serial_print!("tests::page_fault::page_fault... ");
    // setup the interrupt descriptor table to catch the page fault.
    rustos::init();
    unsafe {
        *(0xdead_beef as *mut u64) = 42;
    }
}
