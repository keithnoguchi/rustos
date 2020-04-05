# rustos

[![drone]](https://cloud.drone.io/keithnoguchi/rustos)

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

Happy Hackin'!

[drone]: https://cloud.drone.io/api/badges/keithnoguchi/rustos/status.svg
[writing an os in rust]: https://os.phil-opp.com/
[a freestanding rust binary]: https://os.phil-opp.com/freestanding-rust-binary/
[a minimal rust kernel]: https://os.phil-opp.com/minimal-rust-kernel/
[vga text mode]: https://os.phil-opp.com/vga-text-mode/
[testing]: https://os.phil-opp.com/testing/
[cpu exceptions]: https://os.phil-opp.com/cpu-exceptions/
[double faults]: https://os.phil-opp.com/double-fault-exceptions/
[hardware interrupts]: https://os.phil-opp.com/hardware-interrupts/
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
