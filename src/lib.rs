//! [Writing an OS in Rust]
//!
//! [writing an os in rust]: https://os.phil-opp.com/
//!
//! # Examples
//!
//! Here is the current entry point, `start_kernel()`, and the panic handler.
//!
//! ```
//! #![no_std]
//! #![no_main]
//! #![feature(custom_test_frameworks)]
//! #![test_runner(rustos::test_runner)]
//! #![reexport_test_harness_main = "test_main"]
//! extern crate alloc;
//! extern crate bootloader;
//! extern crate rustos;
//! extern crate x86_64;
//! use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
//! use bootloader::{entry_point, BootInfo};
//! use core::panic::PanicInfo;
//! use rustos::println;
//!
//! entry_point!(start_kernel);
//!
//! fn start_kernel(boot_info: &'static BootInfo) -> ! {
//!     println!("Welcome to the real world!");
//!
//!     // Initialize the kernel.
//!     rustos::init();
//!     rustos::init_memory(boot_info);
//!
//!     // Let's box it on heap!
//!     let x = Box::new(41);
//!     println!("x={:p}", x);
//!
//!     // and then vector!
//!     let mut vec = Vec::new();
//!     for i in 0..500 {
//!         vec.push(i);
//!     }
//!     println!("vec at {:p}", vec.as_slice());
//!
//!     // now, a reference counted vector.
//!     let reference = Rc::new(vec![1, 2, 3]);
//!     let cloned = Rc::clone(&reference);
//!     println!("current reference count is {}", Rc::strong_count(&cloned));
//!     core::mem::drop(reference);
//!     println!("reference count is {} now", Rc::strong_count(&cloned));
//!
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
#![feature(alloc_error_handler)]
extern crate bootloader;
extern crate lazy_static;
extern crate spin;
extern crate x86_64;

mod allocator;
mod gdt;
mod interrupts;
pub mod memory;
pub mod serial;
pub mod vga;

use bootloader::BootInfo;
use core::panic::PanicInfo;
use x86_64::VirtAddr;

// re-exports.
pub use allocator::HEAP_SIZE;
pub use allocator::HEAP_START;

/// Kernel initialization function.
pub fn init() {
    gdt::init();
    interrupts::init();
}

/// Kernel memory manager initialization function.
pub fn init_memory(boot_info: &'static BootInfo) {
    // frame allocator.
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator =
        unsafe { memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    allocator::init(&mut mapper, &mut frame_allocator).expect("allocator failed");
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
use bootloader::entry_point;

#[cfg(test)]
entry_point!(test_kernel);

#[cfg(test)]
fn test_kernel(boot_info: &'static BootInfo) -> ! {
    init();
    init_memory(boot_info);
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
