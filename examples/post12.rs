//! Writing an [OS] in Rust
//!
//! [os]: https://os.phil-opp.com
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate bootloader;
extern crate rustos;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustos::{println, task};

entry_point!(start_kernel);

fn start_kernel(boot_info: &'static BootInfo) -> ! {
    println!("Welcome to the real world!");

    // Initialize the kernel.
    rustos::init();
    rustos::memory::init(boot_info);

    // Spawn async task(s).
    let mut executor = task::Executor::new();
    executor.spawn(task::Task::new(example_task()));

    #[cfg(test)]
    test_main();
    println!("It did not crash!!!");

    // Run forever.
    executor.run()
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

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

async fn async_number() -> u32 {
    42
}
