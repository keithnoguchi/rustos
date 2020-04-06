//! Writing an [OS] in Rust
//!
//! [os]: https://os.phil-opp.com
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
extern crate bootloader;
extern crate rustos;
extern crate x86_64;
use alloc::{boxed::Box, rc::Rc, vec, vec::Vec};
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustos::println;

entry_point!(start_kernel);

fn start_kernel(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to the real world!");

    // Initialize the kernel.
    rustos::init();
    rustos::init_memory(boot_info);

    // Let's box it on heap!
    let x = Box::new(41);
    println!("x={:p}", x);

    // and then vector!
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // now, a reference counted vector.
    let reference = Rc::new(vec![1, 2, 3]);
    let cloned = Rc::clone(&reference);
    println!("current reference count is {}", Rc::strong_count(&cloned));
    core::mem::drop(reference);
    println!("reference count is {} now", Rc::strong_count(&cloned));

    #[cfg(test)]
    test_main();
    println!("It did not crash!!!");
    rustos::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rustos::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
