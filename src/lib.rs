//! [Writing an OS in Rust]
//!
//! [writing an os in rust]: https://os.phil-opp.com/
//!
//! # Examples
//!
//! Here is the current entry point, `start_kernel()`, and panic handlers,
//! and async task.
//!
//! ```
//! #![no_std]
//! #![no_main]
//! #![feature(custom_test_frameworks)]
//! #![test_runner(rustos::test_runner)]
//! #![reexport_test_harness_main = "test_main"]
//! extern crate bootloader;
//! extern crate rustos;
//! use bootloader::{entry_point, BootInfo};
//! use core::panic::PanicInfo;
//! use rustos::{println, task};
//!
//! entry_point!(start_kernel);
//!
//! fn start_kernel(boot_info: &'static BootInfo) -> ! {
//!     println!("Welcome to the real world!");
//!
//!     // Initialize the kernel.
//!     rustos::init();
//!     rustos::memory::init(boot_info);
//!
//!     // Spawn async task(s).
//!     let mut executor = task::Executor::new();
//!     executor.spawn(task::Task::new(example_task()));
//!
//!     #[cfg(test)]
//!     test_main();
//!     println!("It did not crash!!!");
//!
//!     // Run forever.
//!     executor.run()
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
//!
//! async fn example_task() {
//!     let number = async_number().await;
//!     println!("async number: {}", number);
//! }
//!
//! async fn async_number() -> u32 {
//!     42
//! }
//! ```
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(alloc_error_handler)]
#![feature(alloc_layout_extra)]
#![feature(const_fn)]
#![feature(const_in_array_repeat_expressions)]
#![feature(custom_test_frameworks)]
#![feature(abi_x86_interrupt)]
#![feature(wake_trait)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate bootloader;
extern crate lazy_static;
extern crate spin;
extern crate x86_64;

mod allocator;
mod gdt;
mod interrupts;
pub mod memory;
pub mod serial;
pub mod task;
pub mod vga;

use core::panic::PanicInfo;

// re-exports.
pub use allocator::HEAP_SIZE;
pub use allocator::HEAP_START;

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
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(test_kernel);

#[cfg(test)]
fn test_kernel(boot_info: &'static BootInfo) -> ! {
    init();
    memory::init(boot_info);
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
