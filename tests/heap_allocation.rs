#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate alloc;
extern crate rustos;
use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use rustos::{serial_print, serial_println};

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    rustos::memory::init(boot_info);
    test_main();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rustos::test_panic_handler(info)
}

#[test_case]
fn simple_allocation() {
    use alloc::boxed::Box;
    serial_print!("tests::heap_allocation::simple_allocation... ");
    let heap_value = Box::new(41);
    assert_eq!(*heap_value, 41);
    serial_println!("[ok]");
}

#[test_case]
fn large_vec() {
    use alloc::vec::Vec;
    serial_print!("tests::heap_allocation::large_vec... ");
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
    serial_println!("[ok]");
}

#[test_case]
fn many_boxes() {
    use alloc::boxed::Box;
    serial_print!("tests::heap_allocation::many_boxes... ");
    for i in 0..rustos::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    serial_println!("[ok]");
}

#[test_case]
fn many_boxes_long_lived() {
    use alloc::boxed::Box;
    serial_print!("tests::heap_allocation::many_boxes_long_lived... ");
    let long_lived = Box::new(1);
    for i in 0..rustos::HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
    assert_eq!(*long_lived, 1);
    serial_println!("[ok]");
}
