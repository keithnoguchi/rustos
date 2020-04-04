//! Writing an [OS] in Rust: [Testing]
//!
//! [os]: https://os.phil-opp.com
//! [testing]: https://os.phil-opp.com/testing/
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate rustos;
use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}
