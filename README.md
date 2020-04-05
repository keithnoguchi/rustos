# rustos

[![drone]](https://cloud.drone.io/keithnoguchi/rustos)
[![crate]](https://lib.rs/rustos)
[![docs]](https://docs.rs/rustos)

Philipp Oppermann's *awesome* [Writing an OS in Rust]

- Bare bones
  - [A Freestanding Rust Binary] : [post01.rs]
  - [A Minimal Rust Kernel] : [post02.rs]
  - [VGA Text Mode] : [post03.rs]
  - [Testing] : [post04.rs]
    - [tests/basic_boot.rs]
    - [tests/should_panic.rs]
- Interrupts
  - [CPU Exceptions] : [post05.rs]
  - [Double Faults] : [post06.rs]
    - [tests/page_fault.rs]
    - [tests/stack_overflow.rs]
  - [Hardware Interrupts] : [post07.rs]
- Memroy Management
  - [Introduction to Paging] : [post08.rs]

## Examples

Current [main.rs]:

```rust
#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rustos::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate rustos;
extern crate x86_64;
use core::panic::PanicInfo;
use rustos::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Welcome to the real world!");
    rustos::init();
    use x86_64::registers::control::Cr3;
    let (level_4_page_table, _) = Cr3::read();
    println!("Level 4 page table at: {:?}", level_4_page_table);
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
```

Happy Hackin'!

[drone]: https://cloud.drone.io/api/badges/keithnoguchi/rustos/status.svg
[crate]: https://img.shields.io/crates/v/rustos.svg
[docs]: https://docs.rs/rustos/badge.svg
[writing an os in rust]: https://os.phil-opp.com/
[a freestanding rust binary]: https://os.phil-opp.com/freestanding-rust-binary/
[a minimal rust kernel]: https://os.phil-opp.com/minimal-rust-kernel/
[vga text mode]: https://os.phil-opp.com/vga-text-mode/
[testing]: https://os.phil-opp.com/testing/
[cpu exceptions]: https://os.phil-opp.com/cpu-exceptions/
[double faults]: https://os.phil-opp.com/double-fault-exceptions/
[hardware interrupts]: https://os.phil-opp.com/hardware-interrupts/
[introduction to paging]: https://os.phil-opp.com/paging-introduction/
[main.rs]: src/main.rs
[post01.rs]: examples/post01.rs
[post02.rs]: examples/post02.rs
[post03.rs]: examples/post03.rs
[post04.rs]: examples/post04.rs
[tests/basic_boot.rs]: tests/basic_boot.rs
[tests/should_panic.rs]: tests/should_panic.rs
[post05.rs]: examples/post05.rs
[post06.rs]: examples/post06.rs
[tests/page_fault.rs]: tests/page_fault.rs
[tests/stack_overflow.rs]: tests/stack_overflow.rs
[post07.rs]: examples/post07.rs
[post08.rs]: examples/post08.rs
