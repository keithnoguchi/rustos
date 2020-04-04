//! Writing an [OS] in Rust: [CPU Exceptions]
//!
//! [os]: https://os.phil-opp.com
//! [cpu exceptions]: https://os.phil-opp.com/cpu-exceptions/
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]
extern crate rustos;
use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the real world!");
    init();
    x86_64::instructions::interrupts::int3();
    #[cfg(test)]
    test_main();
    println!("It did not crash!!!");
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

extern crate lazy_static;
extern crate x86_64;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

fn init() {
    IDT.load();
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt
    };
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: &mut InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

#[cfg(test)]
mod tests {
    use rustos::{serial_print, serial_println};
    #[test_case]
    fn breakpoint_exception() {
        serial_print!("main::breakpoint_exception... ");
        x86_64::instructions::interrupts::int3();
        serial_println!("[ok]");
    }
}
