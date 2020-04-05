//! [Writing an OS in Rust]
//!
//! [writing an os in rust]: https://os.phil-opp.com/
//!
//! # Examples
//!
//! Example `_start()` function and the panic handler.
//!
//! ```
//! #![no_std]
//! #![no_main]
//! #![feature(custom_test_frameworks)]
//! #![test_runner(rustos::test_runner)]
//! #![reexport_test_harness_main = "test_main"]
//! extern crate rustos;
//! extern crate x86_64;
//! use core::panic::PanicInfo;
//! use rustos::println;
//!
//! #[no_mangle]
//! pub extern "C" fn _start() -> ! {
//!     println!("Welcome to the real world!");
//!     rustos::init();
//!     use x86_64::registers::control::Cr3;
//!     let (level_4_page_table, _) = Cr3::read();
//!     println!("Level 4 page table at: {:?}", level_4_page_table);
//!     #[cfg(test)]
//!     test_main();
//!     println!("It did not crash!!!");
//!     rustos::hlt_loop();
//! }
//!
//! #[cfg(not(test))]
//! #[panic_handler]
//! fn panic(info: &PanicInfo) -> ! {
//!     println!("{}", info);
//!     rustos::hlt_loop();
//! }
//!
//! #[cfg(test)]
//! #[panic_handler]
//! fn panic(info: &PanicInfo) -> ! {
//!     rustos::test_panic_handler(info)
//! }
//! ```
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
extern crate lazy_static;
extern crate spin;
extern crate x86_64;

mod gdt;
mod interrupts;
pub mod serial;
pub mod vga;

use core::panic::PanicInfo;

/// Kernel initialization function.
pub fn init() {
    gdt::init();
    interrupts::init();
}

/// hlt instruction based kernel loop.
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

/// Qemu exit codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    /// Success code.
    Success = 0x10,
    /// Failed code.
    Failed = 0x11,
}

/// Qemu exit function.
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;
    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    init();
    test_main();
    hlt_loop();
}

/// Unit and the integration test runner.
pub fn test_runner(tests: &[&dyn Fn()]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
    exit_qemu(QemuExitCode::Success);
}

/// Unit and the integration test panic handler.
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}
