//! [Writing an OS in Rust]
//!
//! [writing an os in rust]: https://os.phil-opp.com/
//!
//! # Examples
//!
//! Current `start_kernel()` function and the panic handler.
//!
//! ```
//! #![no_std]
//! #![no_main]
//! #![feature(custom_test_frameworks)]
//! #![test_runner(rustos::test_runner)]
//! #![reexport_test_harness_main = "test_main"]
//! extern crate bootloader;
//! extern crate rustos;
//! extern crate x86_64;
//! use bootloader::{entry_point, BootInfo};
//! use core::panic::PanicInfo;
//! use rustos::println;
//! use x86_64::{
//!    structures::paging::{MapperAllSizes, Page},
//!    VirtAddr,
//! };
//!
//! entry_point!(start_kernel);
//!
//! fn start_kernel(boot_info: &'static BootInfo) -> ! {
//!     println!("Welcome to the real world!");
//!
//!     rustos::init();
//!
//!     // frame allocator.
//!     let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
//!     let mut mapper = unsafe { rustos::memory::init(phys_mem_offset) };
//!     let mut frame_allocator =
//!         unsafe { rustos::memory::BootInfoFrameAllocator::init(&boot_info.memory_map) };
//!
//!     // write the string "New!" to the screen!
//!     let page = Page::containing_address(VirtAddr::new(0));
//!     create_example_mapping(page, &mut mapper, &mut frame_allocator);
//!     let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
//!     unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };
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
//!
//! use x86_64::{
//!     structures::paging::{
//!         FrameAllocator, Mapper, OffsetPageTable, PhysFrame, Size4KiB, UnusedPhysFrame,
//!     },
//!     PhysAddr,
//! };
//!
//! fn create_example_mapping(
//!     page: Page,
//!     mapper: &mut OffsetPageTable,
//!     frame_allocator: &mut impl FrameAllocator<Size4KiB>,
//! {
//!     use x86_64::structures::paging::PageTableFlags as Flags;
//!
//!     let frame = PhysFrame::containing_address(PhysAddr::new(0xb8000));
//!     let unused_frame = unsafe { UnusedPhysFrame::new(frame) };
//!     let flags = Flags::PRESENT | Flags::WRITABLE;
//!
//!     let map_to_result = mapper.map_to(page, unused_frame, flags, frame_allocator);
//!     map_to_result.expect("map_to failed").flush();
//! }
//! ```
#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
extern crate bootloader;
extern crate lazy_static;
extern crate spin;
extern crate x86_64;

mod gdt;
mod interrupts;
pub mod memory;
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
use bootloader::{entry_point, BootInfo};

#[cfg(test)]
entry_point!(start_test_kernel);

#[cfg(test)]
fn start_test_kernel(_boot_info: &'static BootInfo) -> ! {
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
